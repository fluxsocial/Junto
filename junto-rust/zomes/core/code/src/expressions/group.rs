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

pub fn create_pack(user: &Address) -> ZomeApiResult<serde_json::Value> {
    //Create pack and link to user with required tags as defined by definitions::app_definitions data?
    let user_entry = utils::get_as_type::<definitions::app_definitions::User>(user.clone())?;
    let pack = definitions::app_definitions::Group{
        parent: user.clone(),
        name: (user_entry.first_name + "'s Pack").to_string(),
        owner: user.clone(),
        private: definitions::app_definitions::Privacy::shared 
    };
    let entry = Entry::App("group".into(), pack.into());
    let pack_address;
    match hdk::commit_entry(&entry){
        Ok(address) => {
            pack_address = address.clone();
            match utils::handle_hooks("Group".to_string(), &address, Some(&user)){
                Ok(_result) => {},
                Err(hdk_err) => return Err(hdk_err.into()),
            }
        },
        Err(hdk_err) => return Err(ZomeApiError::from("Error occured on committing pack entry".to_string()))
    };
    Ok(json!({"pack_address": pack_address}))
}

pub fn pack_link(tag: &'static str, direction: &'static str, pack: &Address, expression: &Address) -> ZomeApiResult<String>{
    if (direction == "reverse") | (direction == "both"){
        hdk::link_entries(&expression, &pack, tag)?;
    }
    if (direction == "forward") | (direction == "both"){
        hdk::link_entries(&pack, &expression, tag)?;
    }
    Ok("Pack links made for owner of pack".to_string())
}