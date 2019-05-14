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
use regex::Regex;

//Our module(s) imports
use super::group;
use super::channel;
use super::time;
use super::indexing;
use super::definitions::{
    app_definitions,
    function_definitions::{
        FunctionDescriptor,
        FunctionParameters,
        GetLinksLoadResult,
        GetLinksLoadElement
    }
};

//Handle hooked objects that need to be created/linked for a given data type
//This is essentially a helper function which allows us to easily and dynamically handle all links/objects that need to be created upon an entry or link
pub fn handle_hooks(expression_type: String, hooks: Vec<FunctionDescriptor>) -> Result<String, ZomeApiError> {
    //First we get all hook functions which can be run on given expression types
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
    let hook_functions: Vec<&'static str> = hook_items.into_iter().map(|hook_map: HashMap<&'static str, &'static str>| { //Likely this neesds to be refactored - hooks hashmap could probably just be a vector of strings instead
        match hook_map.get("function"){
            Some(value) => value.clone(),
            None => {""}
        }
    }).collect();

    if hook_functions.len() > 0{
        for hook_descriptor in hooks.iter(){ //iterate over hook function names provided in function call
            if hook_functions.contains(&hook_descriptor.name){ //Check that is allowed on expression type
                match &hook_descriptor.name{ //Match function names
                    &"time_to_expression" => {
                        match &hook_descriptor.parameters{
                            FunctionParameters::TimeToExpression {tag, direction, expression_address, context} => {
                                time::time_to_expression(tag, direction, &expression_address, &context)?;
                                hdk::debug("Ran time_to_expression function")?;
                            },
                            _ => return Err(ZomeApiError::from("time_to_expresssion expects the LocalTimeToExpression enum value to be present".to_string()))
                        }
                    },
                    &"create_pack" => {
                        match &hook_descriptor.parameters{
                            FunctionParameters::CreatePack {username_address, first_name} =>{
                                group::create_pack(username_address, first_name.to_string())?;
                                hdk::debug("Ran create_pack function")?;
                            },
                            _ => return Err(ZomeApiError::from("create_pack expectes the CreatePack enum value to be present".to_string()))
                        }
                    },
                    &"create_den" => {
                        match &hook_descriptor.parameters{
                            FunctionParameters::CreateDen {username_address, first_name} =>{
                                channel::create_den(username_address, first_name.to_string())?;
                                hdk::debug("Ran create_den function")?;
                            },
                            _ => return Err(ZomeApiError::from("create_den expectes the CreateDen enum value to be present".to_string()))
                        }
                    },
                    &"link_expression" => {
                        match &hook_descriptor.parameters{
                            FunctionParameters::LinkExpression {tag, direction, parent_expression, child_expression} =>{
                                link_expression(tag, direction, &parent_expression, &child_expression)?;
                                hdk::debug("Ran link_expression function")?;
                            },
                            _ => return Err(ZomeApiError::from("link_expression expects the LinkExpression enum value to be present".to_string()))
                        }
                    },
                    &"create_query_points" => {
                        match &hook_descriptor.parameters{
                            FunctionParameters::CreateQueryPoints {query_points, context, privacy, query_type, expression} =>{
                                indexing::create_query_points(query_points.to_vec(), context, privacy, query_type, expression)?;
                                hdk::debug("Ran create_query_points function")?;
                            },
                            _ => return Err(ZomeApiError::from("create_query_points expects the CreateQueryPoints enum value to be present".to_string()))
                        }
                    },
                    &_ => {
                        return Err(ZomeApiError::from("Specified function does not exist".to_string()))
                    }
                }
            } else {
                return Err(ZomeApiError::from("Specified hook function is not present in expression hook definitions".to_string()))
            }
        };
    }
    Ok("Hooks created".to_string()) //success
}

pub fn catch_query_string_types(re: &Regex, query_string: &String) -> Vec<String> {
    let mut caps: Vec<String> = vec![];
    for cap in re.captures_iter(&query_string.as_ref()){
        caps.push(cap[0].to_string());
    };
    caps
}

//Get entry as a given entry type
// pub fn get_as_type<R: TryFrom<AppEntryValue>> (address: HashString) -> ZomeApiResult<R> {
//     let get_result = hdk::get_entry(&address)?;
//     let entry = get_result.ok_or(ZomeApiError::Internal("No entry at this address".into()))?;
//     match entry {
//         Entry::App(_, entry_value) => {
//             R::try_from(entry_value.to_owned())
//                 .map_err(|_| ZomeApiError::Internal(
//                     "Could not convert get_links result to requested type".to_string())
//                 )
//         },
//         _ => Err(ZomeApiError::Internal(
//             "get_links did not return an app entry".to_string())
//         )
//     }
// }

//Link two expression objects together in a given direction
pub fn link_expression(tag: &'static str, direction: &'static str, parent_expression: &Address, child_expression: &Address) -> ZomeApiResult<String>{
    hdk::debug("Linking expressions")?;
    if (direction == "reverse") | (direction == "both"){
        hdk::debug(format!("Linking expression: {} to: {} with tag: {}", child_expression.to_string(), parent_expression.to_string(), tag))?;
        hdk::link_entries(&child_expression, &parent_expression, tag)?;
    }
    if (direction == "forward") | (direction == "both"){
        hdk::debug(format!("Linking expression: {} to: {} with tag: {}", parent_expression.to_string(), child_expression.to_string(), tag))?;
        hdk::link_entries(&parent_expression, &child_expression, tag)?;
    }
    Ok("Links between expressions made with specified tag".to_string())
}

pub fn get_links_and_load<S: Into<String>>(
    base: &HashString,
    tag: S
) -> ZomeApiResult<GetLinksLoadResult<Entry>>  {
	let get_links_result = hdk::get_links(base, tag)?;

	Ok(get_links_result.addresses()
	.iter()
	.map(|address| {
		hdk::get_entry(&address.to_owned())
		.map(|entry: Option<Entry>| {
			GetLinksLoadElement{
				address: address.to_owned(),
				entry: entry.unwrap()
			}
		})
	})
	.filter_map(Result::ok)
	.collect())
}

//This function has now been implemented in the HDK - but its still useful as it can return the address as well as the entry
pub fn get_links_and_load_type<S: Into<String>, R: TryFrom<AppEntryValue>>(base: &HashString, tag: S) -> ZomeApiResult<GetLinksLoadResult<R>> {
	let link_load_results = get_links_and_load(base, tag)?;

	Ok(link_load_results
	.iter()
	.map(|get_links_result| {

		match get_links_result.entry.clone() {
			Entry::App(_, entry_value) => {
				let entry = R::try_from(entry_value)
				.map_err(|_| ZomeApiError::Internal(
					"Could not convert get_links result to requested type".to_string())
				)?;

	            Ok(GetLinksLoadElement::<R>{
	                entry: entry, 
	                address: get_links_result.address.clone()
	            })
			},
			_ => Err(ZomeApiError::Internal(
				"get_links did not return an app entry".to_string())
			)
		}
	})
	.filter_map(Result::ok)
	.collect())
}