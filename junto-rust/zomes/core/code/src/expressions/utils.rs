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
use super::indexing;
use super::definitions::{
    app_definitions,
    function_definitions::{
        FunctionDescriptor,
        FunctionParameters,
        EntryAndAddressResult,
        EntryAndAddress,
        HooksResultTypes
    }
};

//Handle hooked objects that need to be created/linked for a given data type
//This is essentially a helper function which allows us to easily and dynamically handle all links/objects that need to be created
pub fn handle_hooks(expression_type: String, hooks: Vec<FunctionDescriptor>) -> ZomeApiResult<Vec<HooksResultTypes>> {
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
    let mut hook_result_outputs = vec![];
    if hook_functions.len() > 0{
        for hook_descriptor in hooks.iter(){ //iterate over hook function names provided in function call
            if hook_functions.contains(&hook_descriptor.name){ //Check that is allowed on expression type
                match &hook_descriptor.name{ //Match function names
                    &"time_to_expression" => {
                        match &hook_descriptor.parameters{
                            FunctionParameters::TimeToExpression {link_type, tag, direction, expression_address, context} => {
                                hdk::debug("Running time_to_expression")?;
                                let time_addresses = time::time_to_expression(link_type, tag, direction, &expression_address, &context)?;
                                hdk::debug("Ran time_to_expression")?;
                                hook_result_outputs.push(HooksResultTypes::TimeToExpression(time_addresses));
                            },
                            _ => return Err(ZomeApiError::from("time_to_expresssion expects the LocalTimeToExpression enum value to be present".to_string()))
                        }
                    },
                    &"create_pack" => {
                        match &hook_descriptor.parameters{
                            FunctionParameters::CreatePack {username_address, first_name} =>{
                                hdk::debug("Running create_pack")?;
                                let pack = group::create_pack(username_address, first_name.to_string())?;
                                hdk::debug(format!("Ran create_pack, pack address is: {:?}", pack.clone()))?;
                                hook_result_outputs.push(HooksResultTypes::CreatePack(pack))
                            },
                            _ => return Err(ZomeApiError::from("create_pack expectes the CreatePack enum value to be present".to_string()))
                        }
                    },
                    &"create_den" => {
                        match &hook_descriptor.parameters{
                            FunctionParameters::CreateDen {username_address, first_name} =>{
                                hdk::debug("Running create_den")?;
                                let dens = channel::create_den(username_address, first_name.to_string())?;
                                hdk::debug(format!("Ran create_den, dens: {:?}", dens.clone()))?;
                                hook_result_outputs.push(HooksResultTypes::CreateDen(dens))
                            },
                            _ => return Err(ZomeApiError::from("create_den expectes the CreateDen enum value to be present".to_string()))
                        }
                    },
                    &"link_expression" => {
                        match &hook_descriptor.parameters{
                            FunctionParameters::LinkExpression {link_type, tag, direction, parent_expression, child_expression} =>{
                                hdk::debug("Running link_expression")?;
                                let link_result = link_expression(link_type, tag, direction, &parent_expression, &child_expression)?;
                                hdk::debug("Ran link_expression")?;
                                hook_result_outputs.push(HooksResultTypes::LinkExpression(link_result))
                            },
                            _ => return Err(ZomeApiError::from("link_expression expects the LinkExpression enum value to be present".to_string()))
                        }
                    },
                    &"create_post_index" => {
                        match &hook_descriptor.parameters{
                            FunctionParameters::CreatePostIndex {query_points, context, privacy, expression, index_string, link_type} =>{
                                hdk::debug("Running create_post_index")?;
                                let query_point_result = indexing::create_post_index(query_points.to_vec(), context, privacy, expression, index_string.to_string(), link_type.to_string())?;
                                hdk::debug("Ran create_post_index")?;
                                hook_result_outputs.push(HooksResultTypes::CreatePostIndex(query_point_result))
                            },
                            _ => return Err(ZomeApiError::from("create_post_index expects the CreateQueryPoints enum value to be present".to_string()))
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
    Ok(hook_result_outputs) //success
}

//Link two expression objects together in a given direction
pub fn link_expression(link_type: &'static str, tag: &'static str, direction: &'static str, parent_expression: &Address, child_expression: &Address) -> ZomeApiResult<String>{
    hdk::debug("Linking expressions")?;
    if (direction == "reverse") | (direction == "both"){
        hdk::debug(format!("Linking expression: {} (child) to: {} (parent) with tag: {} and link_type: {}", child_expression.to_string(), parent_expression.to_string(), tag, link_type))?;
        hdk::link_entries(&child_expression, &parent_expression, link_type, tag)?;
    }
    if (direction == "forward") | (direction == "both"){
        hdk::debug(format!("Linking expression: {} (parent) to: {} (child) with tag: {} and link_type: {}", parent_expression.to_string(), child_expression.to_string(), tag, link_type))?;
        hdk::link_entries(&parent_expression, &child_expression, link_type, tag)?;
    }
    Ok("Links between expressions made with specified tag".to_string())
}

pub fn get_links_and_load(
    base: &HashString,
    link_type: Option<String>,
    tag: Option<String>
) -> ZomeApiResult<EntryAndAddressResult<Entry>>  {
	let get_links_result = hdk::get_links(base, link_type, tag)?;

	Ok(get_links_result.addresses()
	.iter()
	.map(|address| {
		hdk::get_entry(&address.to_owned())
		.map(|entry: Option<Entry>| {
			EntryAndAddress{
				address: address.to_owned(),
				entry: entry.unwrap()
			}
		})
	})
	.filter_map(Result::ok)
	.collect())
}

//This function has now been implemented in the HDK - but its still useful as it can return the address as well as the entry
pub fn get_links_and_load_type<R: TryFrom<AppEntryValue>>(base: &HashString, link_type: Option<String>, tag: Option<String>) -> ZomeApiResult<EntryAndAddressResult<R>> {
	let link_load_results = get_links_and_load(base, link_type, tag)?;

	Ok(link_load_results
	.iter()
	.map(|get_links_result| {

		match get_links_result.entry.clone() {
			Entry::App(_, entry_value) => {
				let entry = R::try_from(entry_value)
				.map_err(|_| ZomeApiError::Internal(
					"Could not convert get_links result to requested type".to_string())
				)?;

	            Ok(EntryAndAddress::<R>{
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

pub fn get_and_check_perspective(perspective: &Address) -> ZomeApiResult<app_definitions::Channel>{
    let entry = hdk::api::get_entry(perspective)?;
    match entry {
        Some(Entry::App(_, entry_value)) => {
            let perspective_entry = app_definitions::Channel::try_from(&entry_value).map_err(|_err| ZomeApiError::from("Specified perspective address is not of type Channel".to_string()))?; //will return error here if cannot ser entry to group
            if perspective_entry.channel_type != app_definitions::ChannelType::Perspective{
                Err(ZomeApiError::from("Channel is not of type perspective".to_string()))
            } else {
                Ok(perspective_entry)
            }
        },
        Some(_) => Err(ZomeApiError::from("Context address was not an app entry".to_string())),
        None => Err(ZomeApiError::from("No perspective entry at specified address".to_string()))
    }
}