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
use super::group;
use super::user;
use super::collection;
use super::perspective;

pub fn create_post_attributes(indexes: &Vec<HashMap<String, String>>, expression: &Address) -> ZomeApiResult<String>{
    //Creates links between expression and its global attributes (channels, types, times etc)
    for index in indexes{
        match index["type"].as_ref(){
            "channel" => {
                hdk::debug("Linking entry to channel entry")?;
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].to_string(), 
                                attribute_type: app_definitions::AttributeType::Channel}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&expression, &address, "channels", &index["value"])?;
            },

            "type" => {
                hdk::debug("Linking type to expression")?;
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].to_string(), 
                                attribute_type: app_definitions::AttributeType::Type}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&expression, &address, "expression_type", &index["value"])?;
            },

            "time:y" => {
                hdk::debug("Linking time:y to expression")?; 
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].to_string(), 
                                        attribute_type: app_definitions::AttributeType::Year}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&expression, &address, "created_at", "year")?;
            },

            "time:m" => {
                hdk::debug("Linking time:m to expression")?; 
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].to_string(), 
                                        attribute_type: app_definitions::AttributeType::Month}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&expression, &address, "created_at", "month")?;
            },

            "time:d" => {
                hdk::debug("Linking time:d to expression")?; 
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].to_string(), 
                                        attribute_type: app_definitions::AttributeType::Day}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&expression, &address, "created_at", "day")?;
            },

            "time:h" => {
                hdk::debug("Linking time:h to expression")?; 
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].to_string(), 
                                        attribute_type: app_definitions::AttributeType::Hour}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&expression, &address, "created_at", "hour")?;
            },

            "user" => {
                //nothing currently needs to be done for user - expression -> owner link has already been done in handle_post_expression
            },

            _ => {
                return Err(ZomeApiError::from("That index parameter type does not exist".to_string()))
            }
        };
    };
    Ok("ok".to_string())
}

pub fn create_post_index(indexes: Vec<HashMap<String, String>>, context: &Address, privacy: &app_definitions::Privacy, 
                            expression: &Address, index_string: String, link_type: String) -> ZomeApiResult<String>{
    let current_user_hash = user::get_user_username_by_agent_address()?.address;
    //The auth here does not protect application - instead just for correct API calls
    //if someone wants to post expression somewhere they are not allowed the function should say that and not just silently fail in validation
    match hdk::utils::get_as_type::<app_definitions::Collection>(context.clone()) {
        Ok(context_entry) => {
            hdk::debug("Context type collection, running auth")?;
            //check that current user making post is owner of den they are trying to post into
            if collection::is_den_owner(context.clone(), current_user_hash.clone())? == false{
                return Err(ZomeApiError::from("You are attempting to get results from a private collection which you do not own".to_string()))
            };
            //make link on collection context
            hdk::api::link_entries(&context, expression, link_type, index_string)?;
        },
        Err(_err) => {
            hdk::debug("Context type group, running auth")?;
            let context_entry = hdk::utils::get_as_type::<app_definitions::Group>(context.clone()).map_err(|_err| ZomeApiError::from("Context address was not a collection, group or dna address (global context)".to_string()))?;
            if context_entry.privacy != app_definitions::Privacy::Public {
                if (group::is_group_owner(context.clone(), current_user_hash.clone())? == false) & (group::is_group_member(context.clone(), current_user_hash.clone())? == false){
                    return Err(ZomeApiError::from("You are attempting to post an expression into a group you are not permitted to interact with".to_string()))
                };
            };
            //make link on group context
            hdk::api::link_entries(&context, expression, link_type, index_string)?;
        }
    };
    
    //Code below is used to allow a given context to see which index points posts exist on in their context
    //TODO might make more sense just to link to global entry - why do we need more entries for the same thing? - we are no longer linking from these entries so there is no scaling considerations
    hdk::debug("Creating entries for each index in each context and linking expression")?;
    for index in indexes{
        match index["type"].as_ref(){
            "channel" => {
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].to_string(), 
                                attribute_type: app_definitions::AttributeType::Channel}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&context, &address, "channel", &index["value"])?;
            },

            "type" => {
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].to_string(), 
                                attribute_type: app_definitions::AttributeType::Type}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&context, &address, "expression_type", &index["value"])?;
            },

            "time:y" => {
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].to_string(), 
                                        attribute_type: app_definitions::AttributeType::Year}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&context, &address, "time", &index["value"])?;
            },

            "time:m" => {
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].to_string(), 
                                        attribute_type: app_definitions::AttributeType::Month}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&context, &address, "time", &index["value"])?;
            },

            "time:d" => {
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].to_string(), 
                                        attribute_type: app_definitions::AttributeType::Day}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&context, &address, "time", &index["value"])?;
            },

            "time:h" => {
                let entry = Entry::App("attribute".into(), app_definitions::Attribute{value: index["value"].to_string(), 
                                        attribute_type: app_definitions::AttributeType::Hour}.into()).into();
                let address = hdk::commit_entry(&entry)?;
                hdk::api::link_entries(&context, &address, "time", &index["value"])?;
            },

            "user" => {
                //nothing currently needs to be done for user - expression -> owner link has already been done in handle_post_expression
            },

            _ => {
                return Err(ZomeApiError::from("That index type does not exist".to_string()))
            }
        };
    };
    Ok("ok".to_string())
}