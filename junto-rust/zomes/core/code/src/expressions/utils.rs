//Holochain core imports
use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    holochain_core_types::{
        cas::content::Address, 
        entry::Entry, 
        entry::AppEntryValue,
        hash::HashString
    }
};
use std::collections::HashMap;
use std::convert::TryFrom;

//Our module(s) imports
use super::definitions;
use super::user;
use super::group;
use super::channel;
use super::time;

//Handle hooked objects that need to be created/linked for a given data type
pub fn handle_hooks(expression_type: String, parent_address: &Address, child_address: Option<&Address>, 
                    context: Option<&Address>, channels: Option<Vec<definitions::app_definitions::Channel>>) -> Result<String, ZomeApiError> {
    let hook_items: Vec<HashMap<&'static str, &'static str>>;
    match expression_type.as_ref(){
        "User" => hook_items = definitions::app_definitions::get_user_definitions().hooks,
        "Channel" => hook_items = definitions::app_definitions::get_channel_definitions().hooks,
        "ExpressionPost" => hook_items = definitions::app_definitions::get_post_expression_definitions().hooks,
        "Group" => hook_items = definitions::app_definitions::get_group_definitions().hooks,
        "Time" => hook_items = definitions::app_definitions::get_time_definitions().hooks,
        "Resonation" => hook_items = definitions::app_definitions::get_resonation_definitions().hooks,
        _ => return Err(ZomeApiError::from("Expression type does not exist".to_string()))
    }
    if hook_items.len() > 0{
        for hook_definition in hook_items{
            match hook_definition.get("function"){
                Some(&"global_time_to_expression") =>  {
                    time::global_time_to_expression(&hook_definition.get("tag").unwrap(), &hook_definition.get("direction").unwrap().to_string(), 
                                                    &parent_address)
                        .map_err(|err: ZomeApiError<>| err);
                },
                Some(&"local_time_to_expression") => {
                    match context {
                        Some(context_address) => {
                            time::global_time_to_expression(&hook_definition.get("tag").unwrap(), &hook_definition.get("direction").unwrap().to_string(), 
                                                    &parent_address, &context_address)
                                .map_err(|err: ZomeApiError<>| err);
                        },
                        None => return Err(ZomeApiError::from("Context address must be specified for local_time_to_expression".to_string()))
                    }
                },
                Some(&"create_pack") => {
                    group::create_pack(&parent_address)
                        .map_err(|err: ZomeApiError<>| err);
                },
                Some(&"create_den") => {
                    channel::create_den(&parent_address)
                        .map_err(|err: ZomeApiError<>| err);
                },
                Some(&"pack_link") => {
                    match child_address{
                        Some(child_value) => {
                            group::pack_link(&hook_definition.get("tag").unwrap(), &hook_definition.get("direction").unwrap(), 
                                            parent_address, child_value)
                                .map_err(|err: ZomeApiError<>| err);
                        },
                        None => return Err(ZomeApiError::from("Child address must be specified for pack link".to_string()))
                    }
                },
                Some(&"link_user_channel") =>{
                    match child_address {
                        Some(child_value) => {
                            channel::link_user_channel(&hook_definition.get("tag").unwrap(), &hook_definition.get("direction").unwrap(), 
                                                    &parent_address, child_value)
                                .map_err(|err: ZomeApiError<>| err);
                        },
                        None => return Err(ZomeApiError::from("Child address must be specified for pack link".to_string()))
                    }
                },
                None => {},
                _ => {}
            }
        }
    }
    Ok("Hooks created".to_string())
}

pub fn get_as_type<R: TryFrom<AppEntryValue>> (address: HashString) -> ZomeApiResult<R> {
    let get_result = hdk::get_entry(&address)?;
    let entry = get_result.ok_or(ZomeApiError::Internal("No entry at this address".into()))?;
    match entry {
        Entry::App(_, entry_value) => {
            R::try_from(entry_value.to_owned())
                .map_err(|_| ZomeApiError::Internal(
                    "Could not convert get_links result to requested type".to_string())
                )
        },
        _ => Err(ZomeApiError::Internal(
            "get_links did not return an app entry".to_string())
        )
    }
}

// pub fn handle_contextual_links(expression_type: String, parentAddress: Address) -> Result<String, ZomeApiError> {
// }