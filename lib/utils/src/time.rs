use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        entry::Entry,
        cas::content::Address
    },
    holochain_wasm_utils::api_serialization::get_entry::{
        GetEntryResultType,
        GetEntryOptions
    }
};

use std::collections::HashMap;

use types::app_definition;
use super::helpers;

pub fn time_to_expression(link_type: &str, tag: &str, direction: &str, expression_address: &Address) -> ZomeApiResult<Vec<Address>> {
    let timestamps = 
        match hdk::get_entry_result(expression_address, GetEntryOptions {headers: true, ..Default::default()},)?.result {
            GetEntryResultType::Single(result) => {
                let iso_timestamp = serde_json::to_string(&result.headers[0].timestamp()).map_err(|err| ZomeApiError::from(err.to_string()))?;
                create_timestamps(&iso_timestamp)?
            },  
            GetEntryResultType::All(_entry_history) => {
                return Err(ZomeApiError::from("EntryResultType not of enum variant Single".to_string()))
            }
        };

    if timestamps.clone().len() == 0{
        return Err(ZomeApiError::from("Timestamps not found on header".to_string()))
    };
    for timestamp in &timestamps{
        helpers::link_expression(link_type, tag, direction, timestamp, expression_address)?;
    };

    Ok(timestamps)  
}

//Create and link current timestamps (year, month, day) to given parent address - returns vector of timestamps
pub fn create_timestamps(iso_timestamp: &String) -> ZomeApiResult<Vec<Address>> {
    let timestamps = vec![Entry::App("attribute".into(), app_definition::Attribute{value: iso_timestamp[0..5].to_string(), attribute_type: app_definition::AttributeType::Year}.into()),
                        Entry::App("attribute".into(), app_definition::Attribute{value: iso_timestamp[6..8].to_string(), attribute_type: app_definition::AttributeType::Month}.into()),
                        Entry::App("attribute".into(), app_definition::Attribute{value: iso_timestamp[9..11].to_string(), attribute_type: app_definition::AttributeType::Day}.into()),
                        Entry::App("attribute".into(), app_definition::Attribute{value: iso_timestamp[12..14].to_string(), attribute_type: app_definition::AttributeType::Hour}.into())];
    let mut timestamp_address = vec![];

    for timestamp in timestamps{
        let entry_address = hdk::entry_address(&timestamp)?;
        match hdk::get_entry(&entry_address)? {
            Some(_entry) => {
                timestamp_address.push(entry_address);
            },
            None => {
                hdk::commit_entry(&timestamp)?;
                timestamp_address.push(entry_address);
            }
        };
    };

    Ok(timestamp_address)
}

pub fn get_entries_timestamp(entry: &Address) -> ZomeApiResult<HashMap<&'static str, String>>{
    let mut out = HashMap::new();
    match hdk::get_entry_result(entry, GetEntryOptions {headers: true, ..Default::default()},)?.result {
        GetEntryResultType::Single(result) => {
            let iso_timestamp = serde_json::to_string(&result.headers[0].timestamp()).map_err(|err| ZomeApiError::from(err.to_string()))?; //TODO: ensure this is the actual header we want to use
            hdk::debug(format!("Got iso timestamp: {:?}", iso_timestamp))?;
            out.insert("year", iso_timestamp[1..5].to_lowercase());
            out.insert("month", iso_timestamp[6..8].to_lowercase());
            out.insert("day", iso_timestamp[9..11].to_lowercase());
            out.insert("hour", iso_timestamp[12..14].to_lowercase());
        },  
        GetEntryResultType::All(_entry_history) => {
            return Err(ZomeApiError::from("EntryResultType not of enum variant Single".to_string()))
        }
    };
    Ok(out)
}

///Sorts vector of times into ordered vector from year -> hour
pub fn sort_time_vector(times: Vec<&str>) -> Vec<&str> {
    let search_times = vec!["time:y>", "time:m>", "time:d>", "time:h>"];
    let mut times_out = vec![];
    let time_types = times.clone().into_iter().map(|time| time.split("<").collect::<Vec<_>>()[1]).collect::<Vec<_>>();
    for search_time in &search_times{
        match time_types.iter().position(|time_type| time_type == search_time){
            Some(index) => {
                times_out.push(times[index].clone())
            },
            None => times_out.push("*")
        }; 
    };
    times_out
}