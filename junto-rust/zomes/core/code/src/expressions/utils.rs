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
use super::user;
use super::group;
use super::channel;
use super::time;
use super::definitions::{
    app_definitions,
    function_definitions::{
        FunctionDescriptor,
        FunctionParameters
    }
};

// //Handle hooked objects that need to be created/linked for a given data type
pub fn handle_hooks(expression_type: String, hooks: Vec<FunctionDescriptor>) -> Result<String, ZomeApiError> {
    let hook_items: Vec<HashMap<&'static str, &'static str>>;
    match expression_type.as_ref(){
        "User" => hook_items = app_definitions::get_user_definitions().hooks,
        "Channel" => hook_items = app_definitions::get_channel_definitions().hooks,
        "ExpressionPost" => hook_items = app_definitions::get_post_expression_definitions().hooks,
        "Group" => hook_items = app_definitions::get_group_definitions().hooks,
        "Time" => hook_items = app_definitions::get_time_definitions().hooks,
        "Resonation" => hook_items = app_definitions::get_resonation_definitions().hooks,
        _ => return Err(ZomeApiError::from("Expression type does not exist".to_string()))
    }
    let hook_functions: Vec<&'static str> = hook_items.into_iter().map(|hook_map: HashMap<&'static str, &'static str>| {
        match hook_map.get("function"){
            Some(value) => value.clone(),
            None => {""}
        }
    }).collect();

    if hook_functions.len() > 0{
        for hook_descriptor in hooks.iter(){
            if hook_functions.contains(&hook_descriptor.name){
                match &hook_descriptor.name{
                    &"global_time_to_expression" => {
                        match &hook_descriptor.parameters {
                            FunctionParameters::GlobalTimeToExpression {tag, direction, expression_address} => {
                                time::global_time_to_expression(tag, direction, &expression_address)
                                    .map_err(|err: ZomeApiError<>| err);
                            },
                            _ => return Err(ZomeApiError::from("Global time to expression expects the GlobalTimeToExpression enum value to be present".to_string()))
                        }
                    },
                    &"local_time_to_expression" => {
                        match &hook_descriptor.parameters {
                            FunctionParameters::LocalTimeToExpression {tag, direction, expression_address, context} => {
                                time::local_time_to_expression(tag, direction, &expression_address, &context)
                                    .map_err(|err: ZomeApiError<>| err);
                            },
                            _ => return Err(ZomeApiError::from("local_time_to_expression expects the LocalTimeToExpression enum value to be present".to_string()))
                        }
                    },
                    &"create_pack" => {
                        match &hook_descriptor.parameters{
                            FunctionParameters::CreatePack {user} =>{
                                group::create_pack(&user)
                                    .map_err(|err: ZomeApiError<>| err);
                            },
                            _ => return Err(ZomeApiError::from("create_pack expectes the CreatePack enum value to be present".to_string()))
                        }
                    },
                    &"create_den" => {
                        match &hook_descriptor.parameters{
                            FunctionParameters::CreateDen {user} =>{
                                channel::create_den(&user)
                                    .map_err(|err: ZomeApiError<>| err);
                            },
                            _ => return Err(ZomeApiError::from("create_den expectes the CreateDen enum value to be present".to_string()))
                        }
                    },
                    &"link_expression" => {
                        match &hook_descriptor.parameters{
                            FunctionParameters::LinkExpression {tag, direction, parent_expression, child_expression} =>{
                                link_expression(tag, direction, &parent_expression, &child_expression)
                                    .map_err(|err: ZomeApiError<>| err);
                            },
                            _ => return Err(ZomeApiError::from("link_expression expects the LinkExpression enum value to be present".to_string()))
                        }
                    }
                    &_ => {
                        return Err(ZomeApiError::from("Specified function does not exist".to_string()))
                    }
                }
            } else {
                return Err(ZomeApiError::from("Specified hook function is not present in expression hook definitions".to_string()))
            }
        };
    }
    Ok("Hooks created".to_string())
}

// pub fn handle_contextual_links(expression_type: String, parentAddress: Address) -> Result<String, ZomeApiError> {
// }

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

pub fn link_expression(tag: &'static str, direction: &'static str, parent_expression: &Address, child_expression: &Address) -> ZomeApiResult<String>{
    if (direction == "reverse") | (direction == "both"){
        hdk::link_entries(&child_expression, &parent_expression, tag)?;
    }
    if (direction == "forward") | (direction == "both"){
        hdk::link_entries(&parent_expression, &child_expression, tag)?;
    }
    Ok("Links between expressions made with specified tag".to_string())
}