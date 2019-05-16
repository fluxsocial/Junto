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
use super::group;
use super::user;
use super::definitions::{
    app_definitions,
    function_definitions::{
        FunctionDescriptor,
        FunctionParameters
    }
};

//Commits den entry to DHT and runs necassary hooks
pub fn commit_den(entry: &Entry, user: &Address) -> ZomeApiResult<Address> {
    let address = hdk::commit_entry(&entry)?;
    //Build vector describing hook functions which should run to correctly link this data
    let hook_definitions = vec![FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{tag: "den", direction: "reverse", parent_expression: address.clone(), child_expression: user.clone()}},
                                FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{tag: "owner", direction: "forward", parent_expression: address.clone(), child_expression: user.clone()}}];

    utils::handle_hooks("Channel".to_string(), hook_definitions)?;
    Ok(address)
}

//Create den(s) (multiple dens as signified by app_definitions data) and link to user with required tags as defined by definitons data
pub fn create_den(username_address: &Address, first_name: String) -> ZomeApiResult<serde_json::Value> {
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
    let private_entry = Entry::App("channel".into(), private_den.into());
    let shared_entry = Entry::App("channel".into(), shared_den.into());
    let public_entry = Entry::App("channel".into(), public_den.into());

    let private_den_address = commit_den(&private_entry, &username_address)?;
    let shared_den_address = commit_den(&shared_entry, &username_address)?;
    let public_den_address = commit_den(&public_entry, &username_address)?;

    Ok(json!({"private_den_address": private_den_address, "shared_den_address": shared_den_address, "public_den_address": public_den_address}))
}

pub fn is_den_owner(den: Address, user: Address) -> ZomeApiResult<bool>{
    let den_owner_results = utils::get_links_and_load_type::<String, app_definitions::UserName>(&den, "owner".to_string())?;
    Ok(den_owner_results[0].address == user)
}

pub fn create_collective_channel(context: &Address) -> ZomeApiResult<Address> {
    let channel_entry: app_definitions::Channel;
    if context == &Address::from(hdk::api::DNA_ADDRESS.to_string()) {
        channel_entry = app_definitions::Channel{parent: context.clone(), name: "Collective".to_string(), 
                                        privacy: app_definitions::Privacy::Public, channel_type: app_definitions::ChannelType::Tag};
    } else { 
        let user_name_address = user::get_user_username_address_by_agent_address()?;
        if (group::is_group_member(context.clone(), user_name_address.clone())? == true) | (group::is_group_owner(context.clone(), user_name_address.clone())? == true) {
            channel_entry = app_definitions::Channel{parent: context.clone(), name: "Collective".to_string(), 
                                        privacy: app_definitions::Privacy::Shared, channel_type: app_definitions::ChannelType::Tag};
        } else {
            return Err(ZomeApiError::from("You are not a member/owner of given channel".to_string()))
        };
    };
    let channel_collective_entry = Entry::App("channel".into(), channel_entry.into());
    let address = hdk::commit_entry(&channel_collective_entry)?;
    Ok(address)
}

pub fn get_channel_address(channel: app_definitions::Channel) -> ZomeApiResult<Address> {
    hdk::api::entry_address(&Entry::App("channel".into(), channel.into()))
}