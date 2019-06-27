use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        entry::Entry, 
        cas::content::Address
    }
};

use std::collections::HashMap;

use super::definitions::app_definitions;

///Creates links between expression and its attributes (channels, types, times etc)
pub fn create_post_attributes(indexes: &Vec<HashMap<&str, String>>, expression: &Address) -> ZomeApiResult<&'static str>{
    for index in indexes{
        match index["type"].as_str(){
            "channel" => {
                hdk::debug("Linking entry to channel entry")?;
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].clone(), 
                                attribute_type: app_definitions::AttributeType::Channel}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(expression, &address, "channels", &index["value"])?;
            },

            "type" => {
                hdk::debug("Linking type to expression")?;
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].clone(), 
                                attribute_type: app_definitions::AttributeType::Type}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(expression, &address, "expression_type", &index["value"])?;
            },

            "time:y" => {
                hdk::debug("Linking time:y to expression")?; 
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].clone(), 
                                        attribute_type: app_definitions::AttributeType::Year}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(expression, &address, "created_at", "year")?;
            },

            "time:m" => {
                hdk::debug("Linking time:m to expression")?; 
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].clone(), 
                                        attribute_type: app_definitions::AttributeType::Month}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(expression, &address, "created_at", "month")?;
            },

            "time:d" => {
                hdk::debug("Linking time:d to expression")?; 
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].clone(), 
                                        attribute_type: app_definitions::AttributeType::Day}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(expression, &address, "created_at", "day")?;
            },

            "time:h" => {
                hdk::debug("Linking time:h to expression")?; 
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].clone(), 
                                        attribute_type: app_definitions::AttributeType::Hour}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(expression, &address, "created_at", "hour")?;
            },

            "user" => {
                //nothing currently needs to be done for user - expression -> owner link has already been done in handle_post_expression
            },

            _ => {
                return Err(ZomeApiError::from("That index parameter type does not exist".to_string()))
            }
        };
    };
    Ok("ok")
}

///Creates index between post and expression. Also adds attributes to context.
pub fn create_post_index(indexes: &Vec<HashMap<&str, String>>, context: &Address, 
                            expression: &Address, index_string: &str, link_type: &str) -> ZomeApiResult<&'static str>{
    hdk::api::link_entries(&context, expression, link_type, index_string)?;
    
    //Code below is used to enable a given context to see which index points exist on in their context - useful for searching within a context
    hdk::debug("Creating entries for each index in each context and linking expression")?;
    for index in indexes{
        match index["type"].as_str(){
            "channel" => {
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].clone(), 
                                attribute_type: app_definitions::AttributeType::Channel}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(context, &address, "channel", &index["value"])?;
            },

            "type" => {
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].clone(), 
                                attribute_type: app_definitions::AttributeType::Type}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(context, &address, "expression_type", &index["value"])?;
            },

            "time:y" => {
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].clone(), 
                                        attribute_type: app_definitions::AttributeType::Year}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(context, &address, "time", &index["value"])?;
            },

            "time:m" => {
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].clone(), 
                                        attribute_type: app_definitions::AttributeType::Month}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(context, &address, "time", &index["value"])?;
            },

            "time:d" => {
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].clone(), 
                                        attribute_type: app_definitions::AttributeType::Day}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(context, &address, "time", &index["value"])?;
            },

            "time:h" => {
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].clone(), 
                                        attribute_type: app_definitions::AttributeType::Hour}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(context, &address, "time", &index["value"])?;
            },

            "user" => {
                //nothing currently needs to be done for user - expression -> owner link has already been done in handle_post_expression
            },

            _ => {
                return Err(ZomeApiError::from("That index type does not exist".to_string()))
            }
        };
    };
    Ok("ok")
}