use hdk::{
    holochain_core_types::{
        entry::Entry, 
        json::JsonString
    }
};

//Our modules for holochain actins
use super::definitions;
use super::utils;

//Public functions for user data "object"
pub fn handle_create_user(user_data: definitions::app_definitions::User) -> JsonString {
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

// pub fn get_user(user: &Address) -> ZomeApiResult<Entry> {
     //Ok(Entry)
// }