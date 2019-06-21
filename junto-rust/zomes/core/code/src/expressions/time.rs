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

use super::utils;
use super::definitions::{
    app_definitions
};

pub fn time_to_expression(link_type: String, tag: String, direction: String, expression_address: &Address) -> ZomeApiResult<Vec<Address>> {
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
        utils::link_expression(link_type.clone(), tag.clone(), direction.clone(), timestamp, expression_address)?;
    };

    Ok(timestamps)  
}

//Create timestamp functions should not use datatime but instead should use the timestamp in the entry header for the expression that will be linked to the timestamp
//Create and link current timestamps (year, month, day) to given parent address - returns vector of timestamps
pub fn create_timestamps(iso_timestamp: &String) -> ZomeApiResult<Vec<Address>> {
    let timestamps = vec![Entry::App("attribute".into(), app_definitions::Attribute{value: iso_timestamp[0..5].to_string(), attribute_type: app_definitions::AttributeType::Year}.into()),
                        Entry::App("attribute".into(), app_definitions::Attribute{value: iso_timestamp[6..8].to_string(), attribute_type: app_definitions::AttributeType::Month}.into()),
                        Entry::App("attribute".into(), app_definitions::Attribute{value: iso_timestamp[9..11].to_string(), attribute_type: app_definitions::AttributeType::Day}.into()),
                        Entry::App("attribute".into(), app_definitions::Attribute{value: iso_timestamp[12..14].to_string(), attribute_type: app_definitions::AttributeType::Hour}.into())];
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