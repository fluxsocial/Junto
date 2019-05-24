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
        EntryAndAddress,
        UserPack
    }
};
use super::user;
use super::channel;

//Creates a user "group" - more specifically in this case a pack
pub fn create_pack(username_address: &Address, first_name: String) -> ZomeApiResult<UserPack> {
    hdk::debug("Creating pack")?;
    let pack = app_definitions::Group{ //Create default pack data
        parent: username_address.clone(),
        name: (first_name + "'s Pack").to_string(),
        owner: username_address.clone(),
        privacy: app_definitions::Privacy::Shared 
    };
    let entry = Entry::App("group".into(), pack.clone().into());
    let address = hdk::commit_entry(&entry)?;
    let hook_definitions = vec![FunctionDescriptor{name: "time_to_expression", parameters: FunctionParameters::TimeToExpression{link_type: "group", tag: "pack", direction: "forward", expression_address: address.clone(), context: Address::from(DNA_ADDRESS.to_string())}},
                                FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "group", tag: "pack", direction: "reverse", parent_expression: address.clone(), child_expression: username_address.clone()}},
                                FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "auth", tag: "owner", direction: "forward", parent_expression: address.clone(), child_expression: username_address.clone()}}];

    let _hook_result = utils::handle_hooks("Group".to_string(), hook_definitions)?;
    channel::create_collective_channel(&address)?;
    Ok(UserPack{pack: EntryAndAddress{entry: pack, address: address}})
}

pub fn add_pack_member(username_address: Address) -> ZomeApiResult<JsonString>{
    let current_user_username = user::get_user_username_address_by_agent_address()?;
    let group = user::get_user_pack(current_user_username)?.pack;   
    add_member_to_group(username_address.clone(), group.address.clone())
}

pub fn add_member_to_group(username_address: Address, group: Address) -> ZomeApiResult<JsonString>{
    let current_user_username = user::get_user_username_address_by_agent_address()?;
    let group_owner = is_group_owner(group.clone(), current_user_username.clone())?;
    if group_owner == true{
        let group_member = is_group_member(group.clone(), username_address.clone())?;
        if group_member == false{
            hdk::api::link_entries(&group, &username_address, "auth".to_string(), "member".to_string())?;
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
            hdk::api::remove_link(&group, &username_address, "auth".to_string(), "member".to_string())?;
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
            let _entry = app_definitions::Group::try_from(&entry_value).map_err(|_err| ZomeApiError::from("Specified group address is not of type Group".to_string()))?; //will return error here if cannot ser entry to group
            let member_vec = utils::get_links_and_load_type::<app_definitions::UserName>(&group, Some("auth".to_string()), Some("member".to_string()))?;
            for member in member_vec {
                if member.address == user{
                    return Ok(true)
                }
            }
            Ok(false)
        },
        Some(_) => Err(ZomeApiError::from("Group address was not an app entry".to_string())),
        None => Err(ZomeApiError::from("No group entry at specified address".to_string()))
    }
}

pub fn get_group_members(group: Address) -> ZomeApiResult<GroupMembers> {
    //only viewable by group members or owner
    let group_entry = hdk::api::get_entry(&group)?;
    match group_entry {
        Some(Entry::App(_, entry_value)) => {
            let _entry = app_definitions::Group::try_from(&entry_value).map_err(|_err| ZomeApiError::from("Specified group address is not of type Group".to_string()))?; //will return error here if cannot ser entry to group
            let current_user_username = user::get_user_username_address_by_agent_address()?;
            if is_group_owner(group.clone(), current_user_username.clone())? == false && is_group_member(group.clone(), current_user_username.clone())? == false {
                return Err(ZomeApiError::from("You are not an owner or member of this group and thus are not allowed to view given information".to_string()))
            };
            let member_vec = utils::get_links_and_load_type::<app_definitions::UserName>(&group, Some("auth".to_string()), Some("member".to_string()))?;
            Ok(GroupMembers{members: member_vec})
        },
        Some(_) => Err(ZomeApiError::from("Context address was not an app entry".to_string())),
        None => Err(ZomeApiError::from("No context entry at specified address".to_string()))
    }
}

pub fn is_group_owner(group: Address, user: Address) -> ZomeApiResult<bool>{
    let group_entry = hdk::api::get_entry(&group)?;
    match group_entry {
        Some(Entry::App(_, entry_value)) => {
            let entry = app_definitions::Group::try_from(&entry_value).map_err(|_err| ZomeApiError::from("Specified group address is not of type Group".to_string()))?;
            Ok(entry.owner == user) 
        },
        Some(_) => return Err(ZomeApiError::from("Context address was not an app entry".to_string())),
        None => return Err(ZomeApiError::from("No context entry at specified address".to_string()))
    }
}

// pub fn search_groups(query_string: String) -> ZomeApiResult<EntryAndAddressResult<app_definitions::Group>>>{

// }