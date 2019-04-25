//Module to handle all group related operations
use hdk::{
    AGENT_ADDRESS,
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        entry::Entry, 
        cas::content::Address,
        json::JsonString
    }
};

use std::convert::TryFrom;

use super::utils;
use super::definitions::{
    app_definitions,
    function_definitions::{
        FunctionDescriptor,
        FunctionParameters
    }
};
use super::user;

//Creates a user "group" - more specifically in this case a pack
pub fn create_pack(username_address: &Address, first_name: String) -> ZomeApiResult<serde_json::Value> {
    hdk::debug("Creating pack")?;
    let pack = app_definitions::Group{ //Create default pack data
        parent: username_address.clone(),
        name: (first_name + "'s Pack").to_string(),
        owner: username_address.clone(),
        privacy: app_definitions::Privacy::Shared 
    };
    let entry = Entry::App("group".into(), pack.into());
    let pack_address: Address;
    match hdk::commit_entry(&entry){
        Ok(address) => {
            pack_address = address.clone();
            let hook_definitions = vec![FunctionDescriptor{name: "global_time_to_expression", parameters: FunctionParameters::GlobalTimeToExpression{tag: "group", direction: "reverse", expression_address: address.clone()}},
                                        FunctionDescriptor{name: "global_time_to_expression", parameters: FunctionParameters::GlobalTimeToExpression{tag: "pack", direction: "reverse", expression_address: address.clone()}},
                                        FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{tag: "pack", direction: "reverse", parent_expression: address.clone(), child_expression: username_address.clone()}},
                                        FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{tag: "owner", direction: "forward", parent_expression: address.clone(), child_expression: username_address.clone()}}];

            match utils::handle_hooks("Group".to_string(), hook_definitions){
                Ok(_result) => {},
                Err(hdk_err) => return Err(hdk_err.into()),
            }
        },
        Err(_hdk_err) => return Err(ZomeApiError::from("Error occured on committing pack entry".to_string()))
    };
    Ok(json!({"pack_address": pack_address}))
}

pub fn add_to_pack(user: &Address) -> JsonString{
    //get current users pack
    //create link on pack and add member @address
    //add link on members address w/ link: pack_member
    json!({ "message": "Ok" }).into()
}

pub fn is_pack_member(pack: &Address, user: &Address) -> ZomeApiResult<bool>{
    let pack_entry = hdk::api::get_entry(pack)?;
    match pack_entry {
        Some(Entry::App(_, entry_value)) => {
            match app_definitions::Group::try_from(&entry_value){
                Ok(entry) => {
                    match utils::get_links_and_load_type::<String, app_definitions::UserName>(pack, "member".to_string()){
                        Ok(member_vec) =>{
                            for member in member_vec{
                                if member.address == *user{
                                    return Ok(true)
                                };
                            };
                            return Ok(false)
                        },
                        Err(err) => return Err(err)
                    };
                },
                Err(_err) => return Err(ZomeApiError::from("Specified pack address is not of type Group".to_string()))
            }
        },
        Some(_) => return Err(ZomeApiError::from("Context address was not an app entry".to_string())),
        None => return Err(ZomeApiError::from("No context entry at specified address".to_string()))
    }
}

pub fn is_pack_owner(pack: &Address, user: &Address) -> ZomeApiResult<bool>{
    let pack_entry = hdk::api::get_entry(pack)?;
    match pack_entry {
        Some(Entry::App(_, entry_value)) => {
            match app_definitions::Group::try_from(&entry_value){
                Ok(entry) => {
                    return Ok(entry.owner == *user) 
                },
                Err(_err) => return Err(ZomeApiError::from("Specified pack address is not of type Group".to_string()))
            }
        },
        Some(_) => return Err(ZomeApiError::from("Context address was not an app entry".to_string())),
        None => return Err(ZomeApiError::from("No context entry at specified address".to_string()))
    }
}