use hdk::{
    holochain_core_types::{
        cas::content::Address,
        entry::Entry, 
        json::JsonString
    }
};

//Our modules for holochain actins
use super::definitions;
use super::utils;
use super::channel;

pub fn handle_post_expression(expression: definitions::app_definitions::ExpressionPost, channels: Vec<String>) -> JsonString{
    let entry = Entry::App("expression_post".into(), expression.into());
    match hdk::commit_entry(&entry){
        Ok(address) => {},
        Err(hdk_err) => hdk_err.into()
    };
    //get user den
    //get user pack
    //get packs a user is a part of
    //check that channel(s) exist in each of the above expression locals
    //if not create needed channels 
    //check for time in each expression locals
    //create contextual links of times & channels in each expression locals
    json!({"message": "ok"}).into()
}

pub fn handle_resonation(expression: Address, resonation: definitions::app_definitions::Resonation) -> JsonString{
    json!({"message": "ok"}).into()
}