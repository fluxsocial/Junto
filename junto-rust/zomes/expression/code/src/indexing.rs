use hdk::{
    error::{
        ZomeApiResult,
        ZomeApiError
    },
    holochain_core_types::{
        entry::Entry
    },
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_json_api::{
        json::JsonString
    }
};

use std::convert::TryInto;
use std::collections::HashMap;

use types::{
    app_definition,
    function_definition::ContextAuthResult
};

///Creates links between expression and its attributes (channels, types, times etc)
pub fn create_post_attributes(indexes: &Vec<HashMap<&str, String>>, expression: &Address) -> ZomeApiResult<&'static str>{
    for index in indexes{
        match index["type"].as_str(){
            "channel" => {
                hdk::debug("Linking entry to channel entry")?;
                let entry = Entry::App("attribute".into(), app_definition::Attribute{value: index["value"].clone(), 
                                attribute_type: app_definition::AttributeType::Channel}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(expression, &address, "channels", &index["value"])?;
            },

            "type" => {
                hdk::debug("Linking type to expression")?;
                let entry = Entry::App("attribute".into(), app_definition::Attribute{value: index["value"].clone(), 
                                attribute_type: app_definition::AttributeType::Type}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(expression, &address, "expression_type", &index["value"])?;
            },

            _ => {

            }
        };
    };
    Ok("ok")
}

///Creates index between post and expression. Also adds attributes to context.
pub fn create_post_index(indexes: &Vec<HashMap<&str, String>>, context: &Address, 
                            expression: &Address, index_string: &str, link_type: &str) -> ZomeApiResult<&'static str>{
    hdk::debug(format!("Creating post index with string: {} and type {}", index_string, link_type))?;
    hdk::api::link_entries(context, expression, link_type, index_string)?;
    
    //Code below is used to enable a given context to see which index points exist on in their context - useful for searching within a context
    hdk::debug("Creating entries for each index in each context and linking expression")?;
    for index in indexes{
        match index["type"].as_str(){
            "channel" => {
                let entry = Entry::App("attribute".into(), app_definition::Attribute{value: index["value"].clone(), 
                                attribute_type: app_definition::AttributeType::Channel}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(context, &address, "channel", &index["value"])?;
            },

            "type" => {
                let entry = Entry::App("attribute".into(), app_definition::Attribute{value: index["value"].clone(), 
                                attribute_type: app_definition::AttributeType::Type}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(context, &address, "expression_type", &index["value"])?;
            },

            "time:y" => {
                let entry = Entry::App("attribute".into(), app_definition::Attribute{value: index["value"].clone(), 
                                        attribute_type: app_definition::AttributeType::Year}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(context, &address, "time", &index["value"])?;
            },

            "time:m" => {
                let entry = Entry::App("attribute".into(), app_definition::Attribute{value: index["value"].clone(), 
                                        attribute_type: app_definition::AttributeType::Month}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(context, &address, "time", &index["value"])?;
            },

            "time:d" => {
                let entry = Entry::App("attribute".into(), app_definition::Attribute{value: index["value"].clone(), 
                                        attribute_type: app_definition::AttributeType::Day}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(context, &address, "time", &index["value"])?;
            },

            "time:h" => {
                let entry = Entry::App("attribute".into(), app_definition::Attribute{value: index["value"].clone(), 
                                        attribute_type: app_definition::AttributeType::Hour}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(context, &address, "time", &index["value"])?;
            },

            "user" => {}, //nothing currently needs to be done for user

            _ => {
                return Err(ZomeApiError::from("That index type does not exist".to_string()))
            }
        };
    };
    Ok("ok")
}

///Checks if username_address can access context at given context address
///Returns privacy of context or err if cannot access the given context
pub fn run_context_auth(context: &Address, username_address: &Address) -> ZomeApiResult<Option<ContextAuthResult>>{
    match hdk::utils::get_as_type::<app_definition::Collection>(context.clone()) {
        Ok(context_entry) => {
            hdk::debug("Context type collection, running auth")?;
            //check that current user making post is owner of den they are trying to post into
            let is_collection_owner = hdk::call(hdk::THIS_INSTANCE, "collection", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                                "is_collection_owner", JsonString::from(json!({"collection": context, "user": username_address})))?;
            let is_collection_owner: bool = is_collection_owner.try_into()?;
            if is_collection_owner == false{
                Err(ZomeApiError::from("You are attempting to access a collection which you do not own".to_string()))
            } else {
                Ok(Some(ContextAuthResult::Collection(context_entry)))
            }
        },
        Err(_err) => {
            hdk::debug("Context type group, running auth")?;
            let context_entry = hdk::utils::get_as_type::<app_definition::Group>(context.clone()).ok();
            match context_entry{
                Some(context_entry) => {
                    if context_entry.privacy != app_definition::Privacy::Public {
                        let is_group_owner = hdk::call(hdk::THIS_INSTANCE, "group", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                                        "is_group_owner", JsonString::from(json!({"group": context.clone(), "user": username_address.clone()})))?;
                        let is_group_owner: bool = is_group_owner.try_into()?;

                        let is_group_member = hdk::call(hdk::THIS_INSTANCE, "collection", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                                        "is_group_member", JsonString::from(json!({"group": context.clone(), "user": username_address.clone()})))?;
                        let is_group_member: bool = is_group_member.try_into()?;

                        if (is_group_owner == false) & (is_group_member == false){
                            return Err(ZomeApiError::from("You are attempting to access a group you are not permitted to interact with".to_string()))
                        };
                    };
                    Ok(Some(ContextAuthResult::Group(context_entry)))
                },
                None => Ok(None)
            }
        }
    }
}