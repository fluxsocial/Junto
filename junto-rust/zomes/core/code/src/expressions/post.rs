use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        cas::content::Address,
        entry::Entry
    },
    api::DNA_ADDRESS
};

use std::collections::HashMap;

//Our modules for holochain actins
use super::definitions::{
    app_definitions,
    function_definitions::{
        FunctionDescriptor,
        FunctionParameters
    }
};

use super::utils;
use super::user;
use super::group;
use super::indexing;

pub fn handle_post_expression(expression: app_definitions::ExpressionPost, mut tags: Vec<String>, context: Vec<Address>) -> ZomeApiResult<Address>{
    if tags.len() > 4 {
        return Err(ZomeApiError::from("You are not allowed to specify more than 4 tags on a given expression".to_string()))
    } else if tags.len() < 4{
        tags.sort_by(|a, b| b.cmp(&a)); //Order tags vector in reverse alphabetical order
        for _ in tags.len()..4{
            tags.push("*Null*".to_string());
        };
    } else {
        tags.sort_by(|a, b| b.cmp(&a)); //Order tags vector in reverse alphabetical order
    };
    let mut query_points: Vec<HashMap<String, String>> = tags.iter().map(|tag| hashmap!{"type".to_string() => "tag".to_string(), "value".to_string() => tag.to_string().to_lowercase()}).collect();

    let expression_type = expression.expression_type.clone();
    let entry = Entry::App("expression_post".into(), expression.into());
    let address = hdk::commit_entry(&entry)?;
    let username_entry_address = user::get_user_username_by_agent_address()?;

    hdk::api::link_entries(&address, &username_entry_address.address, "auth".to_string(), "owner".to_string())?;
    query_points.push(hashmap!{"type".to_string() => "user".to_string(), "value".to_string() => username_entry_address.entry.username.to_string().to_lowercase()});
    query_points.push(hashmap!{"type".to_string() => "type".to_string(), "value".to_string() => expression_type.to_string().to_lowercase()});

    match entry{
        Entry::ChainHeader(header) => {
            let iso_timestamp = serde_json::to_string(header.timestamp());
            match iso_timestamp{
                Ok(iso_timestamp) => {
                    query_points.push(hashmap!{"type".to_string() => "time:y".to_string(), "value".to_string() => iso_timestamp[0..4].to_string().to_lowercase()}); //add year slice to query params
                    query_points.push(hashmap!{"type".to_string() => "time:m".to_string(), "value".to_string() => iso_timestamp[5..7].to_string().to_lowercase()}); //add month slice to query params
                    query_points.push(hashmap!{"type".to_string() => "time:d".to_string(), "value".to_string() => iso_timestamp[8..10].to_string().to_lowercase()}); //add day slice to query params
                    query_points.push(hashmap!{"type".to_string() => "time:h".to_string(), "value".to_string() => iso_timestamp[11..13].to_string().to_lowercase()}) //add hour slice to query params
                },
                Err(hdk_err) => return Err(ZomeApiError::from(hdk_err.to_string()))
            }
        },
        _ => {}
    };

    //query params are saved in following order: tag1/tag2/tag3/tag4/user/type/time:y/time:m/time:d/time:h - thus tag for each expression link will also be in this order and if there is not four tags present placeholder value will be used
    let index_string = query_points.clone().iter().map(|qp| qp["value"].clone()).collect::<Vec<String>>().join("/");
    hdk::api::link_entries(&username_entry_address.address, &address, "expression_post".to_string(), index_string.clone())?; //link expression to users agent - with index string
    indexing::create_post_attributes(&query_points, &address)?;
    let hook_definitions = build_hooks(context, &address, &query_points, index_string)?; //build function hooks that need to be ran on expression based on which contexts are being used

    utils::handle_hooks("ExpressionPost".to_string(), hook_definitions)?;
    Ok(address)
}

pub fn build_hooks(contexts: Vec<Address>, address: &Address, query_points: &Vec<HashMap<String, String>>, index_string: String) -> ZomeApiResult<Vec<FunctionDescriptor>> {
    let dna_hash_string = Address::from(DNA_ADDRESS.to_string());
    let collective_count = contexts.iter().filter(|&c| *c == *&dna_hash_string).count();
    if collective_count > 1{
        return Err(ZomeApiError::from("You have submitted more than one DNA address - this would cause duplicate posting of the same post".to_string()))
    };

    let user_name_address = user::get_user_username_by_agent_address()?.address;
    let user_pack = user::get_user_pack(user_name_address.clone())?.pack.address;
    let member_results: Vec<Address> = user::get_user_member_packs(user_name_address.clone())?.iter().map(|pack| pack.address.clone()).collect();
    let den_result = user::get_user_dens(user_name_address.clone())?;
    let private_den = den_result.private_den.address;
    let shared_den = den_result.shared_den.address;
    let public_den = den_result.public_den.address;
    let mut local_contexts = vec![&private_den, &shared_den, &public_den, &user_pack];
    local_contexts.extend(&member_results);
    
    let mut hook_definitions = vec![];

    for context in &contexts {
        if context == &dna_hash_string {
            //Link expression to private den
            hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: private_den.clone(), privacy: app_definitions::Privacy::Private, expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});
            //Link expression to shared den
            hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: shared_den.clone(), privacy: app_definitions::Privacy::Shared, expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});
            //Link expression to public den
            hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: public_den.clone(), privacy: app_definitions::Privacy::Public, expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});
            //Link expression to user pack
            hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: user_pack.clone(), privacy: app_definitions::Privacy::Shared, expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});
            for pack in &member_results{ //Link expression to each pack user is a member of
                hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: pack.clone(), privacy: app_definitions::Privacy::Shared, expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});
            };        
        } else {
            if local_contexts.contains(&context){
                if collective_count == 0 { //avoid duplicate posting into private den if already posted in by global context
                    if *&context == &private_den {//private den match
                        //Link expression to private den
                        hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: private_den.clone(), privacy: app_definitions::Privacy::Private, expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});
                    } else if *&context == &shared_den { //shared den match
                        //Link expression to shared den
                        hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: shared_den.clone(), privacy: app_definitions::Privacy::Shared, expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});     
                    } else if *&context == &public_den { //public den match
                        //Link expression to public den
                        hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: public_den.clone(), privacy: app_definitions::Privacy::Public, expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});
                    } else if *&context == &user_pack { //pack match
                        //Link expression to user pack
                        hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: user_pack.clone(), privacy: app_definitions::Privacy::Shared, expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});    
                    } else { //only other possible match is in pack_member results
                        hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: context.clone(), privacy: app_definitions::Privacy::Shared, expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});
                    };
                };
            } else {
                //Only other context possible is another group which is not a pack - check if current user is memeber - if so then insert to hook definitions
                if group::is_group_member(context.clone(), user_name_address.clone())? == true {
                    hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: context.clone(), privacy: app_definitions::Privacy::Shared, expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});   
                } else if group::is_group_owner(context.clone(), user_name_address.clone())? == true {
                    hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: context.clone(), privacy: app_definitions::Privacy::Shared, expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}}); 
                };
            };
        }
    }
    Ok(hook_definitions)
}

//Function to handle the resonation of an expression post - will put the post into packs which the post should be resonated into
pub fn handle_resonation(expression: Address) -> ZomeApiResult<String>{
    match hdk::api::get_entry(&expression)?{
        None => {return Err(ZomeApiError::from("No expression at given address".to_string()))},
        _ => {}
    };
    let user_name_address = user::get_user_username_by_agent_address()?.address;
    let user_pack = user::get_user_pack(user_name_address.clone())?.pack.address;

    let channels = utils::get_links_and_load_type::<app_definitions::Channel>(&expression, Some("expression_channels".to_string()), None)?;
    let times = utils::get_links_and_load_type::<app_definitions::Time>(&expression, Some("time".to_string()), None)?;
    let exp_type = utils::get_links_and_load_type::<app_definitions::Channel>(&expression, Some("expression_type".to_string()), None)?;
    
    let mut query_points: Vec<HashMap<String, String>> = channels.iter().map(|channel| hashmap!{"value".to_string() => channel.entry.name.clone(), "type".to_string() => "channel".to_string()}).collect();
    for time in times{
        match time.entry.time_type{
            app_definitions::TimeType::Year => {query_points.push(hashmap!{"value".to_string() => time.entry.time.clone(), "type".to_string() => "time:Y".to_string()});},
            app_definitions::TimeType::Month => {query_points.push(hashmap!{"value".to_string() => time.entry.time.clone(), "type".to_string() => "time:M".to_string()});},
            app_definitions::TimeType::Day => {query_points.push(hashmap!{"value".to_string() => time.entry.time.clone(), "type".to_string() => "time:D".to_string()});},
            app_definitions::TimeType::Hour => {query_points.push(hashmap!{"value".to_string() => time.entry.time.clone(), "type".to_string() => "time:H".to_string()});}
        };
    }
    query_points.push(hashmap!{"value".to_string() => exp_type[0].entry.name.clone(), "type".to_string() => "type".to_string()});
    let index_string = query_points.clone().iter().map(|qp| qp["value"].clone()).collect::<Vec<String>>().join("/");
    //add link on expression to user who made the resonation?
    let hook_definitions = vec![FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: user_pack.clone(), privacy: app_definitions::Privacy::Shared, expression: expression.clone(), index_string: index_string.clone(), link_type: "resonation".to_string()}},
                                FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "resonation", tag: "", direction: "both", parent_expression: user_pack, child_expression: expression}}];
    utils::handle_hooks("Resonation".to_string(), hook_definitions)?;
    Ok("Resonation Generated".to_string())
}