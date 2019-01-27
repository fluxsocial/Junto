use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{
        entry::Entry, 
        dna::entry_types::Sharing, 
        error::HolochainError, 
        json::JsonString,
        cas::content::Address
    }
};

//Our modules for holochain actins
use super::definitions;
use super::utils;

//Public functions for user data "object"
pub fn handle_create_user(user_data: definitions::User) -> JsonString {
    let entry = Entry::App("user".into(), user_data.into());
    match hdk::commit_entry(&entry) {
        Ok(address) => match utils::handle_hooks("User".to_string(), &address) {
            Ok(result) => json!({"user_address": address, "data": result}).into(),
            Err(hdk_err) => hdk_err.into(),
        }
        Err(hdk_err) => hdk_err.into(),
    }
    //Then we have to handle any hooks/contextual links specified in definitions - functions are in utils.rs currently
}

pub fn time_to_user(tag: &String, direction: &String, user: &Address) -> ZomeApiResult<String> {    
    //Check that current times exist and then link user to times
    let timestamps: Vec<String> = utils::get_current_timestamps();
    Ok("ok".to_string())
}

pub fn create_pack(user: &Address) -> ZomeApiResult<String> {
    //Create pack and link to user with required tags as defined by definitions data?
    Ok("ok".to_string())
}

pub fn create_den(user: &Address) -> ZomeApiResult<String> {
    //Create den(s) (multiple dens as signified by definitions data) and link to user with required tags as defined by definitons data
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