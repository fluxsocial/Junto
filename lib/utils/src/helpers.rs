use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    holochain_core_types::{
        entry::Entry, 
        entry::AppEntryValue,
        link::LinkMatch
    },
    holochain_persistence_api::{
        cas::content::Address,
    },
};

use std::convert::TryFrom;
use std::convert::TryInto;
use std::collections::HashSet;
use std::hash::Hash;
use types::{
    app_definition,
    function_definition::{
        FunctionDescriptor,
        FunctionParameters,
        HooksResultTypes,
        EntryAndAddress,
        EntryAndAddressResult,
        UserDens
    }
};

use super::time;

//This is a helper function which allows us to easily and dynamically handle all functions calls that need to happen
pub fn handle_hooks(hooks: Vec<FunctionDescriptor>) -> ZomeApiResult<Vec<HooksResultTypes>> {
    //First we get all hook functions which can be run on given expression types
    let mut hook_result_outputs = vec![];
    for hook_descriptor in hooks{ //iterate over hook function names provided in function call
        match hook_descriptor.name{ //Match function names
            "time_to_expression" => {
                match hook_descriptor.parameters{
                    FunctionParameters::TimeToExpression {link_type, tag, direction, expression_address} => {
                        hdk::debug("Running time_to_expression")?;
                        let time_addresses = time::time_to_expression(link_type, tag, direction, &expression_address)?;
                        hdk::debug("Ran time_to_expression")?;
                        hook_result_outputs.push(HooksResultTypes::TimeToExpression(time_addresses));
                    },
                    _ => return Err(ZomeApiError::from("time_to_expresssion expects the LocalTimeToExpression enum value to be present".to_string()))
                }
            },
            "create_pack" => {
                match hook_descriptor.parameters{
                    FunctionParameters::CreatePack {username_address, first_name} =>{
                        hdk::debug("Running create_pack")?;
                        let pack = hdk::call(hdk::THIS_INSTANCE, "group", Address::from(hdk::PUBLIC_TOKEN.to_string()), "create_pack", FunctionParameters::CreatePack{username_address, first_name}.into())?;
                        let pack: EntryAndAddress<types::app_definition::Group> = pack.try_into()?;
                        hdk::debug(format!("Ran create_pack, pack is: {:?}", pack.clone()))?;
                        hook_result_outputs.push(HooksResultTypes::CreatePack(pack));
                    },
                    _ => return Err(ZomeApiError::from("create_pack expectes the CreatePack enum value to be present".to_string()))
                }
            },
            "create_den" => {
                match hook_descriptor.parameters{
                    FunctionParameters::CreateDen {username_address, first_name} =>{
                        hdk::debug("Running create_den")?;
                        let dens = hdk::call(hdk::THIS_INSTANCE, "collection", Address::from(hdk::PUBLIC_TOKEN.to_string()), "create_den",
                                                FunctionParameters::CreateDen{username_address, first_name}.into())?;
                        let dens: UserDens = dens.try_into()?;
                        hdk::debug(format!("Ran create_den, dens: {:?}", dens.clone()))?;
                        hook_result_outputs.push(HooksResultTypes::CreateDen(dens));
                    },
                    _ => return Err(ZomeApiError::from("create_den expectes the CreateDen enum value to be present".to_string()))
                }
            },
            "link_expression" => {
                match hook_descriptor.parameters{
                    FunctionParameters::LinkExpression {link_type, tag, direction, parent_expression, child_expression} =>{
                        hdk::debug("Running link_expression")?;
                        let link_result = link_expression(link_type, tag, direction, &parent_expression, &child_expression)?;
                        hdk::debug("Ran link_expression")?;
                        hook_result_outputs.push(HooksResultTypes::LinkExpression(link_result));
                    },
                    _ => return Err(ZomeApiError::from("link_expression expects the LinkExpression enum value to be present".to_string()))
                }
            },
            "create_post_index" => {
                match hook_descriptor.parameters{
                    FunctionParameters::CreatePostIndex {indexes, context, expression, index_string, link_type} =>{
                        hdk::debug("Running create_post_index")?;
                        let query_point_result = hdk::call(hdk::THIS_INSTANCE, "query", Address::from(hdk::PUBLIC_TOKEN.to_string()), "create_post_index",
                                                            FunctionParameters::CreatePostIndex{indexes, context, expression, index_string, link_type}.into())?;
                        let query_point_result: String = query_point_result.try_into()?;
                        hdk::debug("Ran create_post_index")?;
                        hook_result_outputs.push(HooksResultTypes::CreatePostIndex(query_point_result));
                    },
                    _ => return Err(ZomeApiError::from("create_post_index expects the CreatePostIndex enum value to be present".to_string()))
                }
            },
            &_ => {
                return Err(ZomeApiError::from("Specified function does not exist".to_string()))
            }
        };
    };
    Ok(hook_result_outputs) //success
}

//Link two expression objects together in a given direction
pub fn link_expression(link_type: &str, tag: &str, direction: &str, parent_expression: &Address, child_expression: &Address) -> ZomeApiResult<&'static str>{
    hdk::debug("Linking expressions")?;
    if (direction == "reverse") | (direction == "both"){
        hdk::debug(format!("Linking expression: {} (child) to: {} (parent) with tag: {} and link_type: {}", child_expression, parent_expression, tag, link_type))?;
        hdk::link_entries(child_expression, parent_expression, link_type, tag)?;
    }
    if (direction == "forward") | (direction == "both"){
        hdk::debug(format!("Linking expression: {} (parent) to: {} (child) with tag: {} and link_type: {}", parent_expression, child_expression, tag, link_type))?;
        hdk::link_entries(parent_expression, child_expression, link_type, tag)?;
    }
    Ok("Links between expressions made with specified tag")
}

pub fn get_links_and_load(
    base: &Address,
    link_type: LinkMatch<&str>,
    tag: LinkMatch<&str>
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
pub fn get_links_and_load_type<R: TryFrom<AppEntryValue>>(base: &Address, link_type: LinkMatch<&str>, tag: LinkMatch<&str>) -> ZomeApiResult<EntryAndAddressResult<R>> {
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

pub fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}