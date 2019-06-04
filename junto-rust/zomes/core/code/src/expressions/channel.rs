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

//Commits den entry to DHT and runs necassary hooks
pub fn commit_den(entry: &Entry, user: &Address) -> ZomeApiResult<Address> {
    let address = hdk::commit_entry(&entry)?;
    //Build vector describing hook functions which should run to correctly link this data
    let hook_definitions = vec![FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "channel", tag: "den", direction: "reverse", parent_expression: address.clone(), child_expression: user.clone()}},
                                FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "auth", tag: "owner", direction: "forward", parent_expression: address.clone(), child_expression: user.clone()}}];

    utils::handle_hooks("Channel".to_string(), hook_definitions)?;
    Ok(address)
}

//Create den(s) (multiple dens as signified by app_definitions data) and link to user with required tags as defined by definitons data
pub fn create_den(username_address: &Address, first_name: String) -> ZomeApiResult<UserDens> {
    hdk::debug("Creating dens")?;
    let private_den = app_definitions::Channel{ //Create private den
        parent: username_address.clone(),
        name: (first_name.clone() + "'s Den").to_string(),
        privacy: app_definitions::Privacy::Private,
        channel_type: app_definitions::ChannelType::Den
    };
    let shared_den = app_definitions::Channel{ //Create shared den - den viewable by people in your pack
        parent: username_address.clone(),
        name: (first_name.clone()  + "'s Den").to_string(),
        privacy: app_definitions::Privacy::Shared,
        channel_type: app_definitions::ChannelType::Den
    };
    let public_den = app_definitions::Channel{ //Create public den - personal expression place viewable by everyone
        parent: username_address.clone(),
        name: (first_name.clone()  + "'s Den").to_string(),
        privacy: app_definitions::Privacy::Public,
        channel_type: app_definitions::ChannelType::Den
    };
    let private_entry = Entry::App("channel".into(), private_den.clone().into());
    let shared_entry = Entry::App("channel".into(), shared_den.clone().into());
    let public_entry = Entry::App("channel".into(), public_den.clone().into());

    let private_den_address = commit_den(&private_entry, &username_address)?;
    let shared_den_address = commit_den(&shared_entry, &username_address)?;
    let public_den_address = commit_den(&public_entry, &username_address)?;

    Ok(UserDens{private_den: EntryAndAddress{address: private_den_address, entry: private_den}, 
                        shared_den: EntryAndAddress{address: shared_den_address, entry: shared_den}, 
                        public_den: EntryAndAddress{address: public_den_address, entry: public_den}})
}

pub fn is_den_owner(den: Address, user: Address) -> ZomeApiResult<bool>{
    let den_owner_results = utils::get_links_and_load_type::<app_definitions::UserName>(&den, Some("auth".to_string()), Some("owner".to_string()))?;
    Ok(den_owner_results[0].address == user)
}

pub fn get_channel_address(channel: app_definitions::Channel) -> ZomeApiResult<Address> {
    hdk::api::entry_address(&Entry::App("channel".into(), channel.into()))
}

pub fn create_perspective(name: String) -> ZomeApiResult<EntryAndAddress<app_definitions::Channel>>{
    hdk::debug("Creating user perspective")?;
    let current_user = user::get_user_username_by_agent_address()?;
    let perspective_entry = app_definitions::Channel{name: name, parent: current_user.address.clone(), privacy: app_definitions::Privacy::Private, channel_type: app_definitions::ChannelType::Perspective};
    let perspective_address = hdk::api::commit_entry(&Entry::App("channel".into(), perspective_entry.clone().into()))?;
    hdk::api::link_entries(&current_user.address, &perspective_address, "perspective", "")?;
    Ok(EntryAndAddress{address: perspective_address, entry: perspective_entry})
}

pub fn add_user_to_perspective(perspective: Address, target_user: Address) -> ZomeApiResult<Address>{
    let _perspective_entry = utils::get_and_check_perspective(&perspective)?;
    let _user_entry = hdk::api::get_entry(&target_user)?.ok_or(ZomeApiError::from("No such target user".to_string()))?;
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