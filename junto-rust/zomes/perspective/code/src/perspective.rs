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
    },
};

use std::convert::TryFrom;
use std::convert::TryInto;

use utils;
use types::{
    app_definition,
    function_definition::EntryAndAddress
};

pub fn create_perspective(username_address: Address, name: String) -> ZomeApiResult<EntryAndAddress<app_definition::Perspective>>{
    hdk::debug("Creating user perspective")?;
    let perspective_entry = app_definition::Perspective{name: name, parent: username_address.clone()};
    let perspective_address = hdk::api::commit_entry(&Entry::App("perspective".into(), perspective_entry.clone().into()))?;
    hdk::api::link_entries(&username_address, &perspective_address, "perspective", "")?;
    Ok(EntryAndAddress{address: perspective_address, entry: perspective_entry})
}

pub fn add_user_to_perspective(perspective: Address, target_user: Address) -> ZomeApiResult<Address>{
    let _perspective_entry = get_and_check_is_perspective(&perspective)?;
    let _user_entry = hdk::api::get_entry(&target_user)?.ok_or(ZomeApiError::from("No such target user".to_string()))?;
    let current_user = hdk::call(hdk::THIS_INSTANCE, "user", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                                "get_user_username_by_agent_address", JsonString::from(""))?;
    let current_user: EntryAndAddress<app_definition::UserName> = current_user.try_into()?;
    hdk::api::link_entries(&target_user, &current_user.address, "follower", "")?;
    hdk::api::link_entries(&current_user.address, &target_user, "following", "")?;
    hdk::api::link_entries(&perspective, &target_user, "user_perspective", "")
}

pub fn get_perspectives_users(perspective: Address) -> ZomeApiResult<Vec<EntryAndAddress<app_definition::UserName>>> {
    let perspective_entry = get_and_check_is_perspective(&perspective)?;
    let current_user = hdk::call(hdk::THIS_INSTANCE, "user", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                                "get_user_username_by_agent_address", JsonString::from(""))?;
    let current_user: EntryAndAddress<app_definition::UserName> = current_user.try_into()?;
    if perspective_entry.parent == current_user.address{
        let perspective_users = utils::helpers::get_links_and_load_type::<app_definition::UserName>(&perspective, LinkMatch::Exactly("user_perspective"), LinkMatch::Any)?;
        Ok(perspective_users)
    } else {
        Err(ZomeApiError::from("That is not your perspective".to_string()))
    }
}

pub fn get_and_check_is_perspective(perspective: &Address) -> ZomeApiResult<app_definition::Perspective>{
    let entry = hdk::api::get_entry(perspective)?;
    match entry {
        Some(Entry::App(_, entry_value)) => {
            let perspective_entry = app_definition::Perspective::try_from(&entry_value).map_err(|_err| ZomeApiError::from("Specified perspective address is not of type Perspective".to_string()))?; //will return error here if cannot ser entry to group
            Ok(perspective_entry)
        },
        Some(_) => Err(ZomeApiError::from("Context address was not an app entry".to_string())),
        None => Err(ZomeApiError::from("No perspective entry at specified address".to_string()))
    }
}