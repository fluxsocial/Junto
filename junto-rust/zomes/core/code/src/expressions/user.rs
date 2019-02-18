use hdk::{
    holochain_core_types::{
        cas::content::Address,
        entry::Entry, 
        json::JsonString
    }
};

//Our modules for holochain actions
use super::utils;
use super::definitions::{
    app_definitions,
    function_definitions::{
        FunctionDescriptor,
        FunctionParameters
    }
};

//Function to create user and all necassary expression centers for the user
pub fn handle_create_user(user_data: app_definitions::User) -> JsonString {
    let entry = Entry::App("user".into(), user_data.into());
    match hdk::commit_entry(&entry) {
        Ok(address) => {
            //Build hook definitions to link user to timestamps and create pack/den
            let hook_definitions = vec![FunctionDescriptor{name: "global_time_to_expression", parameters: FunctionParameters::GlobalTimeToExpression{tag: "user", direction: "reverse", expression_address: address.clone()}},
                            FunctionDescriptor{name: "create_pack", parameters: FunctionParameters::CreatePack{user: address.clone()}},
                            FunctionDescriptor{name: "create_den", parameters: FunctionParameters::CreateDen{user: address.clone()}}];

            match utils::handle_hooks("User".to_string(), hook_definitions){
                Ok(result) => json!({"user_address": address, "data": result}).into(), //Here no actual results are being returned from handle_hooks function just a string - TODO
                Err(hdk_err) => hdk_err.into(),
            }
        }
        Err(hdk_err) => hdk_err.into(),
    }
    //Might have to handle contextual links here - investigate
}

//Returns user JsonObject from a given address
pub fn handle_get_user(user: Address) -> JsonString {
    match hdk::get_entry(&user){
        Ok(result) => json!({ "user": result }).into(),
        Err(hdk_err) => hdk_err.into()
    }
}