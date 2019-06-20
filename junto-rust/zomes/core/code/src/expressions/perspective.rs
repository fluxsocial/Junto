//Module to handle all channel related operations
use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        entry::Entry, 
        cas::content::Address
    }
};

use super::utils;
// use super::group;
use super::user;
use super::definitions::{
    app_definitions,
    function_definitions::{
        FunctionDescriptor,
        FunctionParameters,
        UserDens,
        EntryAndAddress
    }
};

pub fn create_perspective(name: String) -> ZomeApiResult<EntryAndAddress<app_definitions::Perspective>>{
    hdk::debug("Creating user perspective")?;
    let current_user = user::get_user_username_by_agent_address()?;
    let perspective_entry = app_definitions::Perspective{name: name, parent: current_user.address.clone()};
    let perspective_address = hdk::api::commit_entry(&Entry::App("perspective".into(), perspective_entry.clone().into()))?;
    hdk::api::link_entries(&current_user.address, &perspective_address, "perspective", "")?;
    Ok(EntryAndAddress{address: perspective_address, entry: perspective_entry})
}

pub fn add_user_to_perspective(perspective: Address, target_user: Address) -> ZomeApiResult<Address>{
    let _perspective_entry = utils::get_and_check_perspective(&perspective)?;
    let _user_entry = hdk::api::get_entry(&target_user)?.ok_or(ZomeApiError::from("No such target user".to_string()))?;
    let current_user = user::get_user_username_by_agent_address()?;
    hdk::api::link_entries(&target_user, &current_user.address, "follower", "")?;
    hdk::api::link_entries(&current_user.address, &target_user, "following", "")?;
    hdk::api::link_entries(&perspective, &target_user, "user_perspective", "")
}

pub fn get_perspectives_users(perspective: Address) -> ZomeApiResult<Vec<EntryAndAddress<app_definitions::UserName>>> {
    let perspective_entry = utils::get_and_check_perspective(&perspective)?;
    let current_user = user::get_user_username_by_agent_address()?;
    if perspective_entry.parent == current_user.address{
        let perspective_users = utils::get_links_and_load_type::<app_definitions::UserName>(&perspective, Some("user_perspective".to_string()), None)?;
        Ok(perspective_users)
    } else {
        Err(ZomeApiError::from("That is not your perspective".to_string()))
    }
}