//Module to handle all group related operations
use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        entry::Entry, 
        cas::content::Address,
        json::JsonString
    },
    api::DNA_ADDRESS
};

use std::convert::TryFrom;

use super::utils;
use super::definitions::{
    app_definitions,
    function_definitions::{
        FunctionDescriptor,
        FunctionParameters,
        GroupMembers,
        GetLinksLoadResult
    }
};
use super::user;
use super::channel;

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
            let hook_definitions = vec![FunctionDescriptor{name: "time_to_expression", parameters: FunctionParameters::TimeToExpression{tag: "group", direction: "reverse", expression_address: address.clone(), context: Address::from(DNA_ADDRESS.to_string())}},
                                        FunctionDescriptor{name: "time_to_expression", parameters: FunctionParameters::TimeToExpression{tag: "pack", direction: "reverse", expression_address: address.clone(), context: Address::from(DNA_ADDRESS.to_string())}},
                                        FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{tag: "pack", direction: "reverse", parent_expression: address.clone(), child_expression: username_address.clone()}},
                                        FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{tag: "owner", direction: "forward", parent_expression: address.clone(), child_expression: username_address.clone()}}];

            match utils::handle_hooks("Group".to_string(), hook_definitions){
                Ok(_result) => {},
                Err(hdk_err) => return Err(hdk_err.into()),
            }
        },
        Err(_hdk_err) => return Err(ZomeApiError::from("Error occured on committing pack entry".to_string()))
    };
    channel::create_collective_channel(&pack_address)?;
    Ok(json!({"pack_address": pack_address}))
}

pub fn add_pack_member(username_address: Address) -> ZomeApiResult<JsonString>{
    let current_user_username = user::get_user_username_address_by_agent_address()?;
    let group = user::get_user_pack(current_user_username)?.pack;   
    match group {
        Some(group_obj) => Ok(add_member_to_group(username_address.clone(), group_obj.address.clone())?),
        None => Err(ZomeApiError::from("No pack entries on current user".to_string()))
    }
}

pub fn add_member_to_group(username_address: Address, group: Address) -> ZomeApiResult<JsonString>{
    let current_user_username = user::get_user_username_address_by_agent_address()?;
    let group_owner = is_group_owner(group.clone(), current_user_username.clone())?;
    if group_owner == true{
        let group_member = is_group_member(group.clone(), username_address.clone())?;
        if group_member == false{
            hdk::api::link_entries(&group, &username_address, "member".to_string())?;
            Ok(json!({ "message": "User added to group" }).into())
        } else {
            Err(ZomeApiError::from("User submitted is already a member of given group".to_string()))
        }
    } else {
        Err(ZomeApiError::from("You are not the owner of given group and thus cannot ad/remove members".to_string()))
    }
}

pub fn remove_group_member(username_address: Address, group: Address) -> ZomeApiResult<JsonString>{
    let current_user_username = user::get_user_username_address_by_agent_address()?;
    let group_owner = is_group_owner(group.clone(), current_user_username.clone())?;
    if group_owner == true{
        let group_member = is_group_member(group.clone(), username_address.clone())?;
        if group_member == true{
            hdk::api::remove_link(&group, &username_address, "member".to_string())?;
            Ok(json!({ "message": "User removed from group" }).into())
        } else {
            Err(ZomeApiError::from("User submitted is not a group member of given group".to_string()))
        }
    } else {
        Err(ZomeApiError::from("You are not the owner of given group and thus cannot ad/remove members".to_string()))
    }
}

pub fn is_group_member(group: Address, user: Address) -> ZomeApiResult<bool>{
    let group_entry = hdk::api::get_entry(&group)?;
    match group_entry {
        Some(Entry::App(_, entry_value)) => {
            match app_definitions::Group::try_from(&entry_value){
                Ok(_entry) => {
                    match utils::get_links_and_load_type::<String, app_definitions::UserName>(&group, "member".to_string()){
                        Ok(member_vec) => {
                            for member in member_vec {
                                if member.address == user{
                                    return Ok(true)
                                };
                            };
                            return Ok(false)
                        },
                        Err(err) => return Err(err)
                    };
                },
                Err(_err) => return Err(ZomeApiError::from("Specified group address is not of type Group".to_string()))
            }
        },
        Some(_) => return Err(ZomeApiError::from("Context address was not an app entry".to_string())),
        None => return Err(ZomeApiError::from("No context entry at specified address".to_string()))
    }
}

pub fn get_group_members(group: Address) -> ZomeApiResult<GroupMembers> {
    //only viewable by group members or owner
    let group_entry = hdk::api::get_entry(&group)?;
    match group_entry {
        Some(Entry::App(_, entry_value)) => {
            match app_definitions::Group::try_from(&entry_value){
                Ok(_entry) => {
                    let current_user_username = user::get_user_username_address_by_agent_address()?;
                    if is_group_owner(group.clone(), current_user_username.clone())? == false && is_group_member(group.clone(), current_user_username.clone())? == false {
                        return Err(ZomeApiError::from("You are not an owner or member of this group and thus are not allowed to view given information".to_string()))
                    };
                    match utils::get_links_and_load_type::<String, app_definitions::UserName>(&group, "member".to_string()){
                        Ok(member_vec) => {
                            return Ok(GroupMembers{members: member_vec})
                        },
                        Err(err) => return Err(err)
                    };
                },
                Err(_err) => return Err(ZomeApiError::from("Specified group address is not of type Group".to_string()))
            }
        },
        Some(_) => return Err(ZomeApiError::from("Context address was not an app entry".to_string())),
        None => return Err(ZomeApiError::from("No context entry at specified address".to_string()))
    }
}

pub fn is_group_owner(group: Address, user: Address) -> ZomeApiResult<bool>{
    let group_entry = hdk::api::get_entry(&group)?;
    match group_entry {
        Some(Entry::App(_, entry_value)) => {
            match app_definitions::Group::try_from(&entry_value){
                Ok(entry) => {
                    return Ok(entry.owner == user) 
                },
                Err(_err) => return Err(ZomeApiError::from("Specified group address is not of type Group".to_string()))
            }
        },
        Some(_) => return Err(ZomeApiError::from("Context address was not an app entry".to_string())),
        None => return Err(ZomeApiError::from("No context entry at specified address".to_string()))
    }
}

// pub fn search_groups(query_string: String) -> ZomeApiResult<Vec<GetLinksLoadResult<app_definitions::Group>>>{

// }