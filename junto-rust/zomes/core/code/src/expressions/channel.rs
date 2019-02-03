use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        entry::Entry, 
        cas::content::Address
    }
};

use super::utils;
use super::definitions;

pub fn commit_den(entry: &Entry, user: &Address) -> Result<Address, String>{
    let pack_address;
    match hdk::commit_entry(&entry){
        Ok(address) => {
            pack_address = address.clone();
            match utils::handle_hooks("Channel".to_string(), &address, Some(&user)){
                Ok(_result) => {},
                Err(hdk_err) => return Err(hdk_err.into()),
            }
        },
        Err(_hdk_err) => return Err("There was an error commiting den entry".to_string())
    }
    Ok(pack_address)
}

pub fn create_den(user: &Address) -> ZomeApiResult<serde_json::Value> {
    //Create den(s) (multiple dens as signified by definitions::app_definitions data) and link to user with required tags as defined by definitons data
    let user_entry = utils::get_as_type::<definitions::app_definitions::User>(user.clone())?;
    let private_den = definitions::app_definitions::Channel{
        parent: user.clone(),
        name: (user_entry.first_name.clone() + "'s Den").to_string(),
        privacy: definitions::app_definitions::Privacy::private
    };
    let shared_den = definitions::app_definitions::Channel{
        parent: user.clone(),
        name: (user_entry.first_name.clone()  + "'s Den").to_string(),
        privacy: definitions::app_definitions::Privacy::shared
    };
    let public_den = definitions::app_definitions::Channel{
        parent: user.clone(),
        name: (user_entry.first_name.clone()  + "'s Den").to_string(),
        privacy: definitions::app_definitions::Privacy::public
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

pub fn link_user_channel(tag: &'static str, direction: &'static str, channel: &Address, user: &Address) -> ZomeApiResult<String>{
    //Should check that channel privacy type != public and then make link to user with whatever tag specified
    let channel_entry = utils::get_as_type::<definitions::app_definitions::Channel>(channel.clone())?;
    if channel_entry.privacy == definitions::app_definitions::Privacy::public{
        return Err(ZomeApiError::from("Linking of channel to a user can only occur on non public channels".to_string()));
    }
    if (direction == "reverse") | (direction == "both"){
        hdk::link_entries(&user, &channel, tag)?;
    }
    if (direction == "forward") | (direction == "both"){
        hdk::link_entries(&channel, &user, tag)?;
    }
    Ok("User channel links created".to_string())
}

