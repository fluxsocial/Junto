use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        entry::Entry, 
        cas::content::Address
    }
};

use super::utils;
use super::definitions::{
    app_definitions,
    function_definitions::{
        FunctionDescriptor,
        FunctionParameters
    }
};

pub fn commit_den(entry: &Entry, user: &Address) -> Result<Address, String> {
    let pack_address;
    match hdk::commit_entry(&entry){
        Ok(address) => {
            pack_address = address.clone();
            let hook_definitions = vec![FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{tag: "den", direction: "reverse", parent_expression: address.clone(), child_expression: user.clone()}},
                                        FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{tag: "owner", direction: "forward", parent_expression: address.clone(), child_expression: user.clone()}}];

            match utils::handle_hooks("Channel".to_string(), hook_definitions){
                Ok(_result) => {},
                Err(hdk_err) => return Err(hdk_err.into()),
            }
        },
        Err(_hdk_err) => return Err("There was an error commiting den entry".to_string())
    }
    Ok(pack_address)
}

pub fn create_den(user: &Address) -> ZomeApiResult<serde_json::Value> {
    //Create den(s) (multiple dens as signified by app_definitions data) and link to user with required tags as defined by definitons data
    let user_entry = utils::get_as_type::<app_definitions::User>(user.clone())?;
    let private_den = app_definitions::Channel{
        parent: user.clone(),
        name: (user_entry.first_name.clone() + "'s Den").to_string(),
        privacy: app_definitions::Privacy::Private
    };
    let shared_den = app_definitions::Channel{
        parent: user.clone(),
        name: (user_entry.first_name.clone()  + "'s Den").to_string(),
        privacy: app_definitions::Privacy::Shared
    };
    let public_den = app_definitions::Channel{
        parent: user.clone(),
        name: (user_entry.first_name.clone()  + "'s Den").to_string(),
        privacy: app_definitions::Privacy::Public
    };
    let private_entry = Entry::App("channel".into(), private_den.into());
    let shared_entry = Entry::App("channel".into(), shared_den.into());
    let public_entry = Entry::App("channel".into(), public_den.into());
    let private_den_address: Address;
    let shared_den_address: Address;
    let public_den_address: Address;
    match commit_den(&private_entry, &user){
        Ok(address) => private_den_address = address,
        Err(err) => return Err(ZomeApiError::from(err))
    };

    match commit_den(&shared_entry, &user){
        Ok(address) => shared_den_address = address,
        Err(err) => return Err(ZomeApiError::from(err))
    };

    match commit_den(&public_entry, &user){
        Ok(address) => public_den_address = address,
        Err(err) => return Err(ZomeApiError::from(err))
    };

    Ok(json!({"private_den_address": private_den_address, "shared_den_address": shared_den_address, "public_den_address": public_den_address}))
}

pub fn channel_exists(channel: String, parent: Address, privacy: bool) -> ZomeApiResult<String> {
    Ok("ok".to_string())
}

pub fn create_channel(channel: String, parent: Address, privacy: bool) -> ZomeApiResult<String> {
    Ok("ok".to_string())
}

pub fn create_channels(channels: Vec<app_definitions::Channel>) -> ZomeApiResult<String> {
    Ok("ok".to_string())
}
