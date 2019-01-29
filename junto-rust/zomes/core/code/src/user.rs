use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    error::ZomeApiError,
    api::DNA_ADDRESS,
    holochain_core_types::{
        entry::Entry, 
        dna::entry_types::Sharing, 
        error::HolochainError, 
        json::JsonString,
        cas::content::Address,
        hash::HashString
    },
    holochain_wasm_utils::api_serialization::get_entry::{
        GetEntryOptions,
        StatusRequestKind
    }
};

use multihash::Hash;

//Our modules for holochain actins
use super::definitions;
use super::utils;

//Public functions for user data "object"
pub fn handle_create_user(user_data: definitions::User) -> JsonString {
    let entry = Entry::App("user".into(), user_data.into());
    match hdk::commit_entry(&entry) {
        Ok(address) => match utils::handle_hooks("User".to_string(), &address, None) {
            Ok(result) => json!({"user_address": address, "data": result}).into(),
            Err(hdk_err) => hdk_err.into(),
        }
        Err(hdk_err) => hdk_err.into(),
    }
    //Then we have to handle any hooks/contextual links specified in definitions - functions are in utils.rs currently
}

pub fn global_time_to_expression(tag: &'static str, direction: &String, expression_address: &Address) -> ZomeApiResult<String> {    
    //Check that current times exist and then link expression address to times
    //Get/create timestamps
    let timestamps: Vec<Address>;
    match utils::create_timestamps(HashString::encode_from_str(&DNA_ADDRESS.to_string(), Hash::SHA2256)){
        Ok(result) => timestamps = result,
        Err(hdk_err) => return Err(ZomeApiError::from("There was an error with creating/getting of timesamps".to_string()))
    };

    if (direction == "reverse") | (direction == "both"){
        for timestamp in &timestamps{
            hdk::link_entries(&timestamp, &expression_address, tag)?;
        }
    }
    if (direction == "forward") | (direction == "both"){
        for timestamp in &timestamps{
            hdk::link_entries(&expression_address, &timestamp, tag)?;
        }
    }
    Ok("Expression linked to global time object(s)".to_string())
}

pub fn create_pack(user: &Address) -> ZomeApiResult<serde_json::Value> {
    //Create pack and link to user with required tags as defined by definitions data?
    let user_entry = utils::get_as_type::<definitions::User>(user.clone())?;
    let pack = definitions::Group{
        parent: user.clone(),
        name: (user_entry.first_name + "'s Pack").to_string(),
        owner: user.clone(),
        private: true 
    };
    let entry = Entry::App("group".into(), pack.into());
    let pack_address;
    match hdk::commit_entry(&entry){
        Ok(address) => {
            pack_address = address.clone();
            match utils::handle_hooks("Group".to_string(), &address, Some(&user)){
                Ok(result) => {},
                Err(hdk_err) => return Err(hdk_err.into()),
            }
        },
        Err(hdk_err) => return Err(ZomeApiError::from("Error occured on committing pack entry".to_string()))
    };
    Ok(json!({"pack_address": pack_address}))
}

pub fn create_den(user: &Address) -> ZomeApiResult<String> {
    //Create den(s) (multiple dens as signified by definitions data) and link to user with required tags as defined by definitons data
    Ok("ok".to_string())
}

pub fn pack_link(tag: &'static str, direction: &'static str, pack: &Address, expression: &Address) -> ZomeApiResult<String>{
     Ok("ok".to_string())
}

// pub fn get_user(user: &Address) -> ZomeApiResult<Entry> {
     //Ok(Entry)
// }

//Entry Definition(s)

//Definition for user object
pub fn user_definition() -> ValidatingEntryType {
    entry!(
        name: "user",
        description: "User Object Entry",
        sharing: Sharing::Public,
        native_type: definitions::User,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_user: definitions::User, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}