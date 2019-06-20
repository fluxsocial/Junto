use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        cas::content::Address,
        entry::Entry
    },
    api::DNA_ADDRESS,
    holochain_wasm_utils::api_serialization::{
        get_entry::{
            GetEntryOptions, GetEntryResultType
        }
    }
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
use super::indexing;
use super::random;
use super::group;

//tags = attributes
pub fn handle_post_expression(expression: app_definitions::ExpressionPost, mut tags: Vec<String>, context: Vec<Address>) -> ZomeApiResult<Address>{
    hdk::debug("Handling post expression")?;
    //TODO implement expression type assertion
    tags = tags.into_iter().map(|tag| tag.to_lowercase()).collect::<Vec<String>>();
    if tags.len() > 4 { //TODO implement duplicate tag checking
        return Err(ZomeApiError::from("You are not allowed to specify more than 4 tags on a given expression".to_string()))
    } else if tags.len() < 4{
        if utils::has_unique_elements(tags.clone()) == false {return Err(ZomeApiError::from("You have duplicated tags".to_string()))};
        tags.sort_by(|a, b| b.cmp(&a)); //Order tags vector in reverse alphabetical order
        for _ in tags.len()..4{
            tags.push("*Null*".to_string());
        };
    } else {
        if utils::has_unique_elements(tags.clone()) == false {return Err(ZomeApiError::from("You have duplicated tags".to_string()))};
        tags.sort_by(|a, b| b.cmp(&a)); //Order tags vector in reverse alphabetical order
    };
    hdk::debug(format!("Sorted tags vector: {:?}", tags))?;
    let mut query_points: Vec<HashMap<String, String>> = tags.iter().map(|tag| hashmap!{"type".to_string() => "tag".to_string(), "value".to_string() => tag.to_string().to_lowercase()}).collect();

    let expression_type = expression.expression_type.clone();
    let entry = Entry::App("expression_post".into(), expression.into());
    let address = hdk::commit_entry(&entry)?;
    let username_entry_address = user::get_user_username_by_agent_address()?;

    hdk::debug("Link user to expression as owner")?;
    hdk::api::link_entries(&address, &username_entry_address.address, "auth".to_string(), "owner".to_string())?;
    query_points.push(hashmap!{"type".to_string() => "user".to_string(), "value".to_string() => username_entry_address.entry.username.to_string().to_lowercase()});
    query_points.push(hashmap!{"type".to_string() => "type".to_string(), "value".to_string() => expression_type.to_string().to_lowercase()});

     match hdk::get_entry_result(&address, GetEntryOptions {headers: true, ..Default::default()},)?.result {
        GetEntryResultType::Single(result) => {
            let iso_timestamp = serde_json::to_string(&result.headers[0].timestamp()).map_err(|err| ZomeApiError::from(err.to_string()))?; //TODO: ensure this is the actual header we want to use
            hdk::debug(format!("Got iso timestamp: {:?}", iso_timestamp))?;
            query_points.push(hashmap!{"type".to_string() => "time:y".to_string(), "value".to_string() => iso_timestamp[1..5].to_string().to_lowercase()}); //add year slice to query params
            query_points.push(hashmap!{"type".to_string() => "time:m".to_string(), "value".to_string() => iso_timestamp[6..8].to_string().to_lowercase()}); //add month slice to query params
            query_points.push(hashmap!{"type".to_string() => "time:d".to_string(), "value".to_string() => iso_timestamp[9..11].to_string().to_lowercase()}); //add day slice to query params
            query_points.push(hashmap!{"type".to_string() => "time:h".to_string(), "value".to_string() => iso_timestamp[12..14].to_string().to_lowercase()}); //add hour slice to query params
        },  
        GetEntryResultType::All(_entry_history) => {
            return Err(ZomeApiError::from("EntryResultType not of enum variant Single".to_string()))
        }
    };

    hdk::debug(format!("Generated query_points: {:?}", query_points))?;
    //query params are saved in following order: tag1<tag>/tag2<tag>/tag3<tag>/tag4<tag>/user<user>/type<type>/time:y<time>/time:m<time>/time:d<time>/time:h<time> 
    //thus tag for each expression link will also be in this order and if there is not four tags present placeholder value will be used
    let index_string = query_points.clone().iter().map(|qp| qp["value"].clone() + "<" + &qp["type"].clone() + ">" ).collect::<Vec<String>>().join("/");
    hdk::debug(format!("Index string: {}", index_string))?;
    indexing::create_post_attributes(&query_points, &address)?;
    hdk::debug("Created post attributes")?;
    let hook_definitions = build_hooks(context, &address, &query_points, index_string)?; //build function hooks that need to be ran on expression based on which contexts are being used
    hdk::debug("Hook defnitions generated")?;

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
    let user_pack = user::get_user_pack(user_name_address.clone())?.address;
    let member_results: Vec<Address> = user::get_user_member_packs(user_name_address.clone())?.iter().map(|pack| pack.address.clone()).collect();
    let den_result = user::get_user_dens(user_name_address.clone())?;
    let private_den = den_result.private_den.address;
    let shared_den = den_result.shared_den.address;
    let public_den = den_result.public_den.address;
    let mut local_contexts = vec![&private_den, &shared_den, &public_den, &user_pack];
    local_contexts.extend(&member_results);
    hdk::debug(format!("Building hooks for following contexts Member results: {:?}, private den: {}, shared_den: {}, public_den: {}, pack: {:?}", member_results, private_den, shared_den, public_den, user_pack))?;
    let mut hook_definitions = vec![];
    let current_bit_prefix = random::get_current_bit_prefix()?;
    let bit_prefix_value = random::hash_prefix(address.clone(), current_bit_prefix);
    hdk::debug(format!("App being linked to prefix bucket: {}", bit_prefix_value))?;
    let bit_bucket = hdk::commit_entry(&Entry::App("bucket".into(), app_definitions::Bucket{id: bit_prefix_value}.into()))?;

    for context in &contexts {
        if context == &dna_hash_string {
            hdk::debug("Context is a global context")?;
            //Link expression to user
            hook_definitions.push(FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "expression_post".to_string(), tag: index_string.clone(), direction: "forward".to_string(), parent_expression: user_name_address.clone(), child_expression: address.clone()}});
            //Link between random bit bucket and expression so random post querying can happen on this post
            hook_definitions.push(FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "expression_post".to_string(), tag: index_string.clone(), direction: "forward".to_string(), parent_expression: bit_bucket.clone(), child_expression: address.clone()}});
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
                        hdk::debug("Creating index in private den")?;
                        //Link expression to private den
                        hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: private_den.clone(), privacy: app_definitions::Privacy::Private, expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});
                    } else if *&context == &shared_den { //shared den match
                    hdk::debug("Creating index in shared den")?;
                        //Link expression to shared den
                        hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: shared_den.clone(), privacy: app_definitions::Privacy::Shared, expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});     
                    } else if *&context == &public_den { //public den match
                    hdk::debug("Creating index in public den")?;
                        //Link expression to public den
                        hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: public_den.clone(), privacy: app_definitions::Privacy::Public, expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});
                    } else if *&context == &user_pack { //pack match
                    hdk::debug("Creating index in users pack")?;
                        //Link expression to user pack
                        hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: user_pack.clone(), privacy: app_definitions::Privacy::Shared, expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});    
                    } else { //only other possible match is in pack_member results
                    hdk::debug("Creating index in pack which member is a part of")?;
                        hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: context.clone(), privacy: app_definitions::Privacy::Shared, expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});
                    };
                };
            } else {
                //Only other context possible is another group which is not a pack - check if current user is memeber - if so then insert to hook definitions
                hdk::debug("Creating index in other group")?;
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
    let user_pack = user::get_user_pack(user_name_address.clone())?.address;

    let channels = utils::get_links_and_load_type::<app_definitions::Tag>(&expression, Some("tags".to_string()), None)?;
    let times = utils::get_links_and_load_type::<app_definitions::Time>(&expression, Some("time".to_string()), None)?;
    let exp_type = utils::get_links_and_load_type::<app_definitions::Tag>(&expression, Some("expression_type".to_string()), None)?;
    
    let mut query_points: Vec<HashMap<String, String>> = channels.iter().map(|channel| hashmap!{"value".to_string() => channel.entry.value.clone(), "type".to_string() => "channel".to_string()}).collect();
    for time in times{
        match time.entry.time_type{
            app_definitions::TimeType::Year => {query_points.push(hashmap!{"value".to_string() => time.entry.time.clone(), "type".to_string() => "time:Y".to_string()});},
            app_definitions::TimeType::Month => {query_points.push(hashmap!{"value".to_string() => time.entry.time.clone(), "type".to_string() => "time:M".to_string()});},
            app_definitions::TimeType::Day => {query_points.push(hashmap!{"value".to_string() => time.entry.time.clone(), "type".to_string() => "time:D".to_string()});},
            app_definitions::TimeType::Hour => {query_points.push(hashmap!{"value".to_string() => time.entry.time.clone(), "type".to_string() => "time:H".to_string()});}
        };
    }
    query_points.push(hashmap!{"value".to_string() => exp_type[0].entry.value.clone(), "type".to_string() => "type".to_string()});
    let index_string = query_points.clone().iter().map(|qp| qp["value"].clone()).collect::<Vec<String>>().join("/");
    //add link on expression to user who made the resonation?
    let hook_definitions = vec![FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{query_points: query_points.clone(), context: user_pack.clone(), privacy: app_definitions::Privacy::Shared, expression: expression.clone(), index_string: index_string.clone(), link_type: "resonation".to_string()}},
                                FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "resonation".to_string(), tag: "".to_string(), direction: "both".to_string(), parent_expression: user_pack, child_expression: expression}}];
    utils::handle_hooks("Resonation".to_string(), hook_definitions)?;
    Ok("Resonation Generated".to_string())
}