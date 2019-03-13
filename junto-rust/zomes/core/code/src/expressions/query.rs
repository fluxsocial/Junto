//Module to handle all channel related operations
use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        entry::Entry, 
        cas::content::Address
    },
    api::get_entry
};

use std::collections::HashMap;

use super::definitions::{
    app_definitions,
    function_definitions::{
        FunctionDescriptor,
        FunctionParameters
    }
};

fn get_channel_entry(channel: app_definitions::Channel) -> ZomeApiResult<Option<Entry>>{
    let entry = Entry::App("channel".into(), channel}.into());
    let address = hdk::entry_address(&entry)?;
    hdk::get_entry(&address)?
}

fn get_time_entry(time: app_definitions::Time) -> ZomeApiResult<Option<Entry>>{
    let entry = Entry::App("time".into(), time}.into());
    let address = hdk::entry_address(&entry)?;
    hdk::get_entry(&address)?
}

pub fn create_query_points(query_points: &Vec<HashMap<String, String>>, context: &Address, privacy: &app_definitions::Privacy) -> ZomeApiResult<String>{
    for query_param in query_points{
        match query_param["type"].as_ref(){
            "Channel" => {
                let dht_entry = get_channel_entry(app_definitions::Channel{name: query_param["value"].to_string(), 
                                                    parent: context.clone(), privacy: privacy.clone(), channel_type: app_definitions::ChannelType::Tag)?;

                match dht_entry {
                    Some(value) => {},
                    None => {hdk::utils::commit_and_links(&entry, context, "channel")?;}
                };
            },
            "Type" => {
                let entry = Entry::App("channel".into(), app_definitions::Channel{name: query_param["value"].to_string(), 
                                        parent: context.clone(), privacy: privacy.clone(), channel_type: app_definitions::ChannelType::Type}.into());
                let address = hdk::entry_address(&entry)?;
                let dht_entry = hdk::get_entry(&address)?;

                match dht_entry {
                    Some(value) => {},
                    None => {hdk::utils::commit_and_links(&entry, context, "channel")?;}
                };
            },
            "Time:Y" => {
                let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                        parent: context.clone(), time_type: app_definitions::TimeTypes::Year}.into());
                let address = hdk::entry_address(&entry)?;
                let dht_entry = hdk::get_entry(&address)?;

                match dht_entry {
                    Some(value) => {},
                    None => {
                        let address = hdk::utils::commit_and_links(&entry, context, "time")?;
                        hdk::api::link_entries(context, &address, "year")?;
                    }
                };
            },
            "Time:M" => {
                let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                        parent: context.clone(), time_type: app_definitions::TimeTypes::Month}.into());
                let address = hdk::entry_address(&entry)?;
                let dht_entry = hdk::get_entry(&address)?;

                match dht_entry {
                    Some(value) => {},
                    None => {
                        let address = hdk::utils::commit_and_links(&entry, context, "time")?;
                        hdk::api::link_entries(context, &address, "month")?;
                    }
                };
            },
            "Time:D" => {
                let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                        parent: context.clone(), time_type: app_definitions::TimeTypes::Day}.into());
                let address = hdk::entry_address(&entry)?;
                let dht_entry = hdk::get_entry(&address)?;

                match dht_entry {
                    Some(value) => {},
                    None => {
                        let address = hdk::utils::commit_and_links(&entry, context, "time")?;
                        hdk::api::link_entries(context, &address, "day")?;
                    }
                };
            },
            "Time:H" => {
                let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                        parent: context.clone(), time_type: app_definitions::TimeTypes::Hour}.into());
                let address = hdk::entry_address(&entry)?;
                let dht_entry = hdk::get_entry(&address)?;

                match dht_entry {
                    Some(value) => {},
                    None => {
                        let address = hdk::utils::commit_and_links(&entry, context, "time")?;
                        hdk::api::link_entries(context, &address, "hour")?;
                    }
                };
            },
            "User" => {
                //Should look for user with given username value - if it is not found - return error, dont create user
            },
            &_ => {

            }
        };
    };
    //create standard link on above values between query point and expression
    //check query type if == contextual then create contextual links & create_contextual_links, then return ok
    Ok("ok".to_string())
}

pub fn create_contextual_links(query_points: &Vec<HashMap<String, String>>, expression: &Address) -> ZomeApiResult<String>{
    Ok("ok".to_string())
}

pub fn create_expression_links(query_points: &Vec<HashMap<String, String>>, expression: &Address, context: &Address) -> ZomeApiResult<String>{
    Ok("ok".to_string())
}
