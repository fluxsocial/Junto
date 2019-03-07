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
                    &"global_time_to_expression" => {
                        match &hook_descriptor.parameters { //ensure we have the correct parameters for each function
                            FunctionParameters::GlobalTimeToExpression {tag, direction, expression_address} => { //unpack enum into the relevant variables
                                time::global_time_to_expression(tag, direction, &expression_address)?; //call function
                            },
                            _ => return Err(ZomeApiError::from("Global time to expression expects the GlobalTimeToExpression enum value to be present".to_string())) //GlobalTimeToExpression parameters must be present for this function to be ran
                        }
                    },
                    &"local_time_to_expression" => {
                        match &hook_descriptor.parameters {
                            FunctionParameters::LocalTimeToExpression {tag, direction, expression_address, context} => {
                                time::local_time_to_expression(tag, direction, &expression_address, &context)?;
                            },
                            _ => return Err(ZomeApiError::from("local_time_to_expression expects the LocalTimeToExpression enum value to be present".to_string()))
                        }
                    },
                    &"create_pack" => {
                        match &hook_descriptor.parameters{
                            FunctionParameters::CreatePack {user} =>{
                                group::create_pack(&user)?;
                            },
                            _ => return Err(ZomeApiError::from("create_pack expectes the CreatePack enum value to be present".to_string()))
                        }
                    },
                    &"create_den" => {
                        match &hook_descriptor.parameters{
                            FunctionParameters::CreateDen {user} =>{
                                channel::create_den(&user)?;
                            },
                            _ => return Err(ZomeApiError::from("create_den expectes the CreateDen enum value to be present".to_string()))
                        }
                    },
                    &"link_expression" => {
                        match &hook_descriptor.parameters{
                            FunctionParameters::LinkExpression {tag, direction, parent_expression, child_expression} =>{
                                link_expression(tag, direction, &parent_expression, &child_expression)?;
                            },
                            _ => return Err(ZomeApiError::from("link_expression expects the LinkExpression enum value to be present".to_string()))
                        }
                    },
                    &"create_channels" => {
                        match &hook_descriptor.parameters{
                            FunctionParameters::CreateChannels {channels, parent, privacy} =>{
                                channel::create_channels(channels, parent, privacy)?;
                            },
                            _ => return Err(ZomeApiError::from("link_expression expects the LinkExpression enum value to be present".to_string()))
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

//This will handle our queryable link structure - contextual links name likely to change. 
//
//This function will enable the dynamic querying we envison to happen. 
//We should be able to query and expression in relation to another - so that means for example: being able to query a given group by any amount of channels and times
// pub fn handle_contextual_links(expression_type: String, parent_address: Address) -> Result<String, ZomeApiError> {
// }

//Get entry as a given entry type
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

//Link two expression objects together in a given direction
pub fn link_expression(tag: &'static str, direction: &'static str, parent_expression: &Address, child_expression: &Address) -> ZomeApiResult<String>{
    if (direction == "reverse") | (direction == "both"){
        hdk::link_entries(&child_expression, &parent_expression, tag)?;
    }
    if (direction == "forward") | (direction == "both"){
        hdk::link_entries(&parent_expression, &child_expression, tag)?;
    }
    Ok("Links between expressions made with specified tag".to_string())
}

pub fn get_links_and_load<S: Into<String>>(
    base: &HashString,
    tag: S
) -> ZomeApiResult<app_definitions::GetLinksLoadResult<Entry>>  {
	let get_links_result = hdk::get_links(base, tag)?;

	Ok(get_links_result.addresses()
	.iter()
	.map(|address| {
		hdk::get_entry(&address.to_owned())
		.map(|entry: Option<Entry>| {
			app_definitions::GetLinksLoadElement{
				address: address.to_owned(),
				entry: entry.unwrap()
			}
		})
	})
	.filter_map(Result::ok)
	.collect())
}

pub fn get_links_and_load_type<S: Into<String>, R: TryFrom<AppEntryValue>>(base: &HashString, tag: S) -> ZomeApiResult<app_definitions::GetLinksLoadResult<R>> {
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

	            Ok(app_definitions::GetLinksLoadElement::<R>{
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

pub fn sort_alphabetically(mut vector: Vec<String>) -> Vec<String>{
    vector.sort_by(|a, b| b.cmp(a));
    vector
}