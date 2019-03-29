//Module to handle all group related operations
use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        entry::Entry, 
        cas::content::Address,
        json::JsonString
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

use super::user;

//Creates a user "group" - more specifically in this case a pack
pub fn create_pack(user: &Address) -> ZomeApiResult<serde_json::Value> {
    let user_entry = user::get_user_profile()?.entry;
    let pack = app_definitions::Group{ //Create default pack data
        parent: user.clone(),
        name: (user_entry.first_name + "'s Pack").to_string(),
        owner: user.clone(),
        private: app_definitions::Privacy::Shared 
    };
    let entry = Entry::App("group".into(), pack.into());
    let pack_address: Address;
    match hdk::commit_entry(&entry){
        Ok(address) => {
            pack_address = address.clone();
            let hook_definitions = vec![FunctionDescriptor{name: "global_time_to_expression", parameters: FunctionParameters::GlobalTimeToExpression{tag: "group", direction: "reverse", expression_address: address.clone()}},
                                        FunctionDescriptor{name: "global_time_to_expression", parameters: FunctionParameters::GlobalTimeToExpression{tag: "pack", direction: "reverse", expression_address: address.clone()}},
                                        FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{tag: "pack", direction: "reverse", parent_expression: address.clone(), child_expression: user.clone()}},
                                        FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{tag: "owner", direction: "forward", parent_expression: address.clone(), child_expression: user.clone()}}];

            match utils::handle_hooks("Group".to_string(), hook_definitions){
                Ok(_result) => {},
                Err(hdk_err) => return Err(hdk_err.into()),
            }
        },
        Err(_hdk_err) => return Err(ZomeApiError::from("Error occured on committing pack entry".to_string()))
    };
    Ok(json!({"pack_address": pack_address}))
}

pub fn add_to_pack(user: &Address) -> JsonString{
    //get current users pack
    //create link on pack and add member @address
    //add link on members address w/ link: pack_member
    json!({ "message": "Ok" }).into()
}