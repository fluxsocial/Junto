use hdk::{
    error::{
        ZomeApiResult,
        ZomeApiError
    },
    holochain_core_types::{
        entry::Entry, 
        link::LinkMatch
    },
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_json_api::{
        json::JsonString
    }
};

use std::convert::TryFrom;
use std::convert::TryInto;

use utils;
use types::{
    app_definition,
    function_definition::{
        FunctionDescriptor,
        FunctionParameters,
        GroupMembers,
        EntryAndAddress
    }
};

//Creates a user "group" - more specifically in this case a pack
pub fn create_pack(username_address: Address, first_name: String) -> ZomeApiResult<EntryAndAddress<app_definition::Group>> {
    hdk::debug("Creating pack")?;
    let pack = app_definition::Group{ //Create default pack data
        name: (first_name + "'s Pack").to_string(),
        owner: username_address.clone(),
        privacy: app_definition::Privacy::Shared 
    };
    let entry = Entry::App("group".into(), pack.clone().into());
    let address = hdk::commit_entry(&entry)?;
    let hook_definitions = vec![FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "group", tag: "pack", direction: "reverse", parent_expression: address.clone(), child_expression: username_address.clone()}},
                                FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "group_auth", tag: "owner", direction: "forward", parent_expression: address.clone(), child_expression: username_address.clone()}}];

    let _hook_result = utils::helpers::handle_hooks(hook_definitions)?;
    Ok(EntryAndAddress{entry: pack, address: address})
}

pub fn add_pack_member(username_address: Address) -> ZomeApiResult<JsonString>{
    let current_user_username = hdk::call(hdk::THIS_INSTANCE, "user", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                                "get_user_username_by_agent_address", JsonString::from(""))?;
    let current_user_username: EntryAndAddress<app_definition::UserName> = current_user_username.try_into()?;
    let group = get_user_pack(current_user_username.address)?;   
    add_member_to_group(username_address.clone(), group.address.clone())
}

pub fn add_member_to_group(username_address: Address, group: Address) -> ZomeApiResult<JsonString>{
    let current_user_username = hdk::call(hdk::THIS_INSTANCE, "user", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                                "get_user_username_by_agent_address", JsonString::from(""))?;
    let current_user_username: EntryAndAddress<app_definition::UserName> = current_user_username.try_into()?;
    let group_owner = is_group_owner(group.clone(), current_user_username.address)?;
    if group_owner == true{
        let group_member = is_group_member(group.clone(), username_address.clone())?;
        if group_member == false{
            hdk::api::link_entries(&group, &username_address, "group_auth".to_string(), "member".to_string())?;
            hdk::api::link_entries(&username_address, &group, "auth".to_string(), "member".to_string())?;
            Ok(json!({ "message": "User added to group" }).into())
        } else {
            Err(ZomeApiError::from("User submitted is already a member of given group".to_string()))
        }
    } else {
        Err(ZomeApiError::from("You are not the owner of given group and thus cannot ad/remove members".to_string()))
    }
}

pub fn remove_group_member(username_address: Address, group: Address) -> ZomeApiResult<JsonString>{
    let current_user_username = hdk::call(hdk::THIS_INSTANCE, "user", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                                "get_user_username_by_agent_address", JsonString::from(""))?;
    let current_user_username: EntryAndAddress<app_definition::UserName> = current_user_username.try_into()?;
    let group_owner = is_group_owner(group.clone(), current_user_username.address)?;
    if group_owner == true{
        let group_member = is_group_member(group.clone(), username_address.clone())?;
        if group_member == true{
            hdk::api::remove_link(&group, &username_address, "group_auth".to_string(), "member".to_string())?;
            hdk::api::remove_link(&username_address, &group, "auth".to_string(), "member".to_string())?;
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
            let _entry = app_definition::Group::try_from(&entry_value).map_err(|_err| ZomeApiError::from("Specified group address is not of type Group".to_string()))?; //will return error here if cannot ser entry to group
            let member_vec = utils::helpers::get_links_and_load_type::<app_definition::UserName>(&group, LinkMatch::Exactly("group_auth"), LinkMatch::Exactly("member"))?;
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
            let _entry = app_definition::Group::try_from(&entry_value).map_err(|_err| ZomeApiError::from("Specified group address is not of type Group".to_string()))?; //will return error here if cannot ser entry to group
            let current_user_username = hdk::call(hdk::THIS_INSTANCE, "user", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                                        "get_user_username_by_agent_address", JsonString::from(""))?;
            let current_user_username: EntryAndAddress<app_definition::UserName> = current_user_username.try_into()?;
            if is_group_owner(group.clone(), current_user_username.address.clone())? == false && is_group_member(group.clone(), current_user_username.address)? == false {
                return Err(ZomeApiError::from("You are not an owner or member of this group and thus are not allowed to view given information".to_string()))
            };
            let member_vec = utils::helpers::get_links_and_load_type::<app_definition::UserName>(&group, LinkMatch::Exactly("group_auth"), LinkMatch::Exactly("member"))?;
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
            let entry = app_definition::Group::try_from(&entry_value).map_err(|_err| ZomeApiError::from("Specified group address is not of type Group".to_string()))?;
            Ok(entry.owner == user) 
        },
        Some(_) => return Err(ZomeApiError::from("Context address was not an app entry".to_string())),
        None => return Err(ZomeApiError::from("No context entry at specified address".to_string()))
    }
}

pub fn get_user_pack(username_address: Address) -> ZomeApiResult<EntryAndAddress<app_definition::Group>>{
    let pack_links = utils::helpers::get_links_and_load_type::<app_definition::Group>(&username_address, LinkMatch::Exactly("group"), LinkMatch::Exactly("pack"))?;
    hdk::debug(format!("Pack links on username: {}", pack_links.len().to_string()))?;
    if pack_links.len() > 1{
        return Err(ZomeApiError::from("Pack links on user greater than 1".to_string()))
    } else if pack_links.len() == 0{
        return Err(ZomeApiError::from("No pack links on user".to_string()))
    }
    Ok(pack_links[0].clone())
}

pub fn get_user_member_packs(username_address: Address) -> ZomeApiResult<Vec<EntryAndAddress<app_definition::Group>>>{
    let pack_links = utils::helpers::get_links_and_load_type::<app_definition::Group>(&username_address, LinkMatch::Exactly("auth"), LinkMatch::Exactly("member"))?;
    let mut packs: Vec<EntryAndAddress<app_definition::Group>> = vec![];
    for pack in pack_links{
        packs.push(pack.clone());
    };
    Ok(packs)
}

// pub fn search_groups(query_string: String) -> ZomeApiResult<EntryAndAddressResult<app_definition::Group>>>{

// }