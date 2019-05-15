use hdk::{
    AGENT_ADDRESS,
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        cas::content::Address,
        entry::Entry, 
        entry::AppEntryValue,
        json::JsonString,
        hash::HashString
    },
    api::DNA_ADDRESS
};

use std::collections::HashMap;
use multihash::Hash;
use std::convert::TryFrom;

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

//Function to handle the posting of an expression - will link to any specified channels and insert into relevant groups/packs
pub fn handle_post_expression(expression: app_definitions::ExpressionPost, channels: Vec<String>, context: Vec<Address>) -> ZomeApiResult<Address>{
    let expression_type = expression.expression_type.clone();
    let mut query_params: Vec<HashMap<String, String>> = channels.iter().map(|channel| hashmap!{"type".to_string() => "Channel".to_string(), "value".to_string() => channel.to_string().to_lowercase()}).collect();

    let entry = Entry::App("expression_post".into(), expression.into());
    let address = hdk::commit_entry(&entry)?;

    match utils::get_links_and_load_type::<String, app_definitions::UserName>(&AGENT_ADDRESS, "username".to_string()){
        Ok(result_vec) => {
            if result_vec.len() > 1{
                return Err(ZomeApiError::from("Post Failed links on user greater than 1".to_string()))
            }
            hdk::api::link_entries(&address, &result_vec[0].address, "owner".to_string())?;
            query_params.push(hashmap!{"type".to_string() => "User".to_string(), "value".to_string() => result_vec[0].entry.username.to_string().to_lowercase()});
        },
        Err(hdk_err) => return Err(hdk_err)
    };
    query_params.push(hashmap!{"type".to_string() => "Type".to_string(), "value".to_string() => expression_type.to_string().to_lowercase()});
    
    match entry{
        Entry::ChainHeader(header) => {
            let iso_timestamp = serde_json::to_string(header.timestamp());
            match iso_timestamp{
                Ok(iso_timestamp) => {
                    query_params.push(hashmap!{"type".to_string() => "Time:Y".to_string(), "value".to_string() => iso_timestamp[0..4].to_string().to_lowercase()}); //add year slice to query params
                    query_params.push(hashmap!{"type".to_string() => "Time:M".to_string(), "value".to_string() => iso_timestamp[5..7].to_string().to_lowercase()}); //add month slice to query params
                    query_params.push(hashmap!{"type".to_string() => "Time:D".to_string(), "value".to_string() => iso_timestamp[8..10].to_string().to_lowercase()}); //add day slice to query params
                    query_params.push(hashmap!{"type".to_string() => "Time:H".to_string(), "value".to_string() => iso_timestamp[11..13].to_string().to_lowercase()}) //add hour slice to query params
                },
                Err(hdk_err) => return Err(ZomeApiError::from(hdk_err.to_string()))
            }
        },
        _ => {}
    }
    query_params.sort_by(|a, b| b["value"].cmp(&a["value"])); //Order vector in reverse alphabetical order
    let hook_definitions = build_hooks(context, &address, &query_params)?; //build function hooks that need to be ran on expression based on which contexts are being used

    utils::handle_hooks("ExpressionPost".to_string(), hook_definitions)?;
    Ok(address)
}

pub fn build_hooks(contexts: Vec<Address>, address: &Address, query_params: &Vec<HashMap<String, String>>) -> ZomeApiResult<Vec<FunctionDescriptor>> {
    let dna_hash_string = HashString::from(hdk::api::DNA_ADDRESS.to_string());
    let collective_count = contexts.iter().filter(|&c| *c == *&dna_hash_string).count();
    if collective_count > 1{
        return Err(ZomeApiError::from("You have submitted more than one DNA address - this would cause duplicate posting of the same post".to_string()))
    };

    let user_name_address = user::get_user_username_address_by_agent_address()?;
    let user_pack = user::get_user_pack(user_name_address.clone())?.pack.unwrap().address;
    let member_results: Vec<Address> = user::get_user_member_packs(user_name_address.clone())?.iter().map(|pack| pack.address.clone()).collect();
    let den_result = user::get_user_dens(user_name_address.clone())?;
    let private_den = den_result.private_den.unwrap().address;
    let shared_den = den_result.shared_den.unwrap().address;
    let public_den = den_result.public_den.unwrap().address;
    let mut local_contexts = vec![&private_den, &shared_den, &public_den, &user_pack];
    local_contexts.extend(&member_results);
    
    let mut hook_definitions = vec![];

    for context in &contexts {
        if context == &dna_hash_string {
            //Link expression to public area
            hook_definitions.push(FunctionDescriptor{name: "time_to_expression", parameters: FunctionParameters::TimeToExpression{tag: "expression", direction: "forward", expression_address: address.clone(), context: Address::from(DNA_ADDRESS.to_string())}});
            hook_definitions.push(FunctionDescriptor{name: "create_query_points", parameters: FunctionParameters::CreateQueryPoints{query_points: query_params.clone(), context: dna_hash_string.clone(), privacy: app_definitions::Privacy::Public, query_type: "Contextual".to_string(), expression: address.clone()}});
            //Link expression to private den
            hook_definitions.push(FunctionDescriptor{name: "local_time_to_expression", parameters: FunctionParameters::TimeToExpression{tag: "expression", direction: "forward", expression_address: address.clone(), context: private_den.clone()}});
            hook_definitions.push(FunctionDescriptor{name: "create_query_points", parameters: FunctionParameters::CreateQueryPoints{query_points: query_params.clone(), context: private_den.clone(), privacy: app_definitions::Privacy::Private, query_type: "Standard".to_string(), expression: address.clone()}});
            //Link expression to shared den
            hook_definitions.push(FunctionDescriptor{name: "local_time_to_expression", parameters: FunctionParameters::TimeToExpression{tag: "expression", direction: "forward", expression_address: address.clone(), context: shared_den.clone()}});
            hook_definitions.push(FunctionDescriptor{name: "create_query_points", parameters: FunctionParameters::CreateQueryPoints{query_points: query_params.clone(), context: shared_den.clone(), privacy: app_definitions::Privacy::Shared, query_type: "Standard".to_string(), expression: address.clone()}});
            //Link expression to public den
            hook_definitions.push(FunctionDescriptor{name: "local_time_to_expression", parameters: FunctionParameters::TimeToExpression{tag: "expression", direction: "forward", expression_address: address.clone(), context: public_den.clone()}});
            hook_definitions.push(FunctionDescriptor{name: "create_query_points", parameters: FunctionParameters::CreateQueryPoints{query_points: query_params.clone(), context: public_den.clone(), privacy: app_definitions::Privacy::Public, query_type: "Standard".to_string(), expression: address.clone()}});
            //Link expression to user pack
            hook_definitions.push(FunctionDescriptor{name: "local_time_to_expression", parameters: FunctionParameters::TimeToExpression{tag: "expression", direction: "forward", expression_address: address.clone(), context: user_pack.clone()}});
            hook_definitions.push(FunctionDescriptor{name: "create_query_points", parameters: FunctionParameters::CreateQueryPoints{query_points: query_params.clone(), context: user_pack.clone(), privacy: app_definitions::Privacy::Shared, query_type: "Standard".to_string(), expression: address.clone()}});
            for pack in &member_results{ //Link expression to each pack user is a member of
                hook_definitions.push(FunctionDescriptor{name: "local_time_to_expression", parameters: FunctionParameters::TimeToExpression{tag: "expression", direction: "forward", expression_address: address.clone(), context: pack.clone()}});
                hook_definitions.push(FunctionDescriptor{name: "create_query_points", parameters: FunctionParameters::CreateQueryPoints{query_points: query_params.clone(), context: pack.clone(), privacy: app_definitions::Privacy::Shared, query_type: "Standard".to_string(), expression: address.clone()}});
            };        
        } else {
            if local_contexts.contains(&context){
                if collective_count == 0 { //avoid duplicate posting into private den if already posted in by global context
                    if *&context == &private_den {//private den match
                        //Link expression to private den
                        hook_definitions.push(FunctionDescriptor{name: "local_time_to_expression", parameters: FunctionParameters::TimeToExpression{tag: "expression", direction: "forward", expression_address: address.clone(), context: private_den.clone()}});
                        hook_definitions.push(FunctionDescriptor{name: "create_query_points", parameters: FunctionParameters::CreateQueryPoints{query_points: query_params.clone(), context: private_den.clone(), privacy: app_definitions::Privacy::Private, query_type: "Standard".to_string(), expression: address.clone()}});
                    } else if *&context == &shared_den { //shared den match
                        //Link expression to shared den
                        hook_definitions.push(FunctionDescriptor{name: "local_time_to_expression", parameters: FunctionParameters::TimeToExpression{tag: "expression", direction: "forward", expression_address: address.clone(), context: shared_den.clone()}});
                        hook_definitions.push(FunctionDescriptor{name: "create_query_points", parameters: FunctionParameters::CreateQueryPoints{query_points: query_params.clone(), context: shared_den.clone(), privacy: app_definitions::Privacy::Shared, query_type: "Standard".to_string(), expression: address.clone()}});     
                    } else if *&context == &public_den { //public den match
                        //Link expression to public den
                        hook_definitions.push(FunctionDescriptor{name: "local_time_to_expression", parameters: FunctionParameters::TimeToExpression{tag: "expression", direction: "forward", expression_address: address.clone(), context: public_den.clone()}});
                        hook_definitions.push(FunctionDescriptor{name: "create_query_points", parameters: FunctionParameters::CreateQueryPoints{query_points: query_params.clone(), context: public_den.clone(), privacy: app_definitions::Privacy::Public, query_type: "Standard".to_string(), expression: address.clone()}});
                    } else if *&context == &user_pack { //pack match
                        //Link expression to user pack
                        hook_definitions.push(FunctionDescriptor{name: "local_time_to_expression", parameters: FunctionParameters::TimeToExpression{tag: "expression", direction: "forward", expression_address: address.clone(), context: user_pack.clone()}});
                        hook_definitions.push(FunctionDescriptor{name: "create_query_points", parameters: FunctionParameters::CreateQueryPoints{query_points: query_params.clone(), context: user_pack.clone(), privacy: app_definitions::Privacy::Shared, query_type: "Standard".to_string(), expression: address.clone()}});    
                    } else { //only other possible match is in pack_member results
                        hook_definitions.push(FunctionDescriptor{name: "local_time_to_expression", parameters: FunctionParameters::TimeToExpression{tag: "expression", direction: "forward", expression_address: address.clone(), context: context.clone()}});
                        hook_definitions.push(FunctionDescriptor{name: "create_query_points", parameters: FunctionParameters::CreateQueryPoints{query_points: query_params.clone(), context: context.clone(), privacy: app_definitions::Privacy::Shared, query_type: "Standard".to_string(), expression: address.clone()}});
                    };
                };
            } else {
                //Only other context possible is another group which is not a pack - check if current user is memeber - if so then insert to hook definitions
                if (group::is_group_member(context.clone(), user_name_address.clone())? == true) | (group::is_group_owner(context.clone(), user_name_address.clone())? == true) {
                    hook_definitions.push(FunctionDescriptor{name: "local_time_to_expression", parameters: FunctionParameters::TimeToExpression{tag: "expression", direction: "forward", expression_address: address.clone(), context: context.clone()}});
                    hook_definitions.push(FunctionDescriptor{name: "create_query_points", parameters: FunctionParameters::CreateQueryPoints{query_points: query_params.clone(), context: context.clone(), privacy: app_definitions::Privacy::Shared, query_type: "Standard".to_string(), expression: address.clone()}});   
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
    let user_name_address = user::get_user_username_address_by_agent_address()?;
    let user_pack;
    match user::get_user_pack(user_name_address.clone())?.pack{
        Some(pack) => {user_pack = pack.address;},
        None => return Err(ZomeApiError::from("User has no packs".to_string()))
    };

    let channels = utils::get_links_and_load_type::<String, app_definitions::Channel>(&expression, "channel".to_string())?;
    let times = utils::get_links_and_load_type::<String, app_definitions::Time>(&expression, "time".to_string())?;
    let exp_type = utils::get_links_and_load_type::<String, app_definitions::Channel>(&expression, "type".to_string())?;
    
    let mut query_points: Vec<HashMap<String, String>> = channels.iter().map(|channel| hashmap!{"value".to_string() => channel.entry.name.clone(), "type".to_string() => "Channel".to_string()}).collect();
    for time in times{
        match time.entry.time_type{
            app_definitions::TimeType::Year => {query_points.push(hashmap!{"value".to_string() => time.entry.time.clone(), "type".to_string() => "Time:Y".to_string()});},
            app_definitions::TimeType::Month => {query_points.push(hashmap!{"value".to_string() => time.entry.time.clone(), "type".to_string() => "Time:M".to_string()});},
            app_definitions::TimeType::Day => {query_points.push(hashmap!{"value".to_string() => time.entry.time.clone(), "type".to_string() => "Time:D".to_string()});},
            app_definitions::TimeType::Hour => {query_points.push(hashmap!{"value".to_string() => time.entry.time.clone(), "type".to_string() => "Time:H".to_string()});}
        };
    }
    query_points.push(hashmap!{"value".to_string() => exp_type[0].entry.name.clone(), "type".to_string() => "Type".to_string()});
    
    let hook_definitions = vec![FunctionDescriptor{name: "create_query_points", parameters: FunctionParameters::CreateQueryPoints{query_points: query_points.clone(), context: user_pack.clone(), privacy: app_definitions::Privacy::Shared, query_type: "Standard".to_string(), expression: expression.clone()}},
                                FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{tag: "resonation", direction: "both", parent_expression: user_pack, child_expression: expression}}];
    utils::handle_hooks("Resonation".to_string(), hook_definitions)?;
    Ok("Resonation Generated".to_string())
}