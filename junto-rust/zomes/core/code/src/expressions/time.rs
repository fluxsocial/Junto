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

pub fn time_to_expression(tag: &'static str, direction: &'static str, expression_address: &Address, context: &Address) -> ZomeApiResult<String> {
    let mut timestamps: Vec<Address> = vec![];
    let iso_timestamp;
    match hdk::get_entry_result(expression_address, GetEntryOptions {headers: true, ..Default::default()},)?.result {
        GetEntryResultType::Single(result) => {
            iso_timestamp = serde_json::to_string(&result.headers[0]).map_err(|err| ZomeApiError::from(err.to_string()))?;
            hdk::debug(iso_timestamp.clone())?;
            timestamps = create_timestamps(context, &iso_timestamp)?;
        },  
        GetEntryResultType::All(_entry_history) => {
            return Err(ZomeApiError::from("EntryResultType not of enum variant Single".to_string()))
        }
    };
    if timestamps.clone().len() == 0{
        return Err(ZomeApiError::from("Timestamps not found on header".to_string()))
    };

    for timestamp in &timestamps{
        utils::link_expression(tag, direction, timestamp, expression_address)?;
    };

    Ok("Expression linked to global time object(s)".to_string())  
}

//Create timestamp functions should not use datatime but instead should use the timestamp in the entry header for the expression that will be linked to the timestamp
//Create and link current timestamps (year, month, day) to given parent address - returns vector of timestamps
pub fn create_timestamps(parent: &Address, iso_timestamp: &String) -> ZomeApiResult<Vec<Address>> {
    let timestamps = vec![Entry::App("time".into(), app_definitions::Time{time: iso_timestamp[0..4].to_string(), parent: parent.clone(), time_type: app_definitions::TimeType::Year}.into()),
                        Entry::App("time".into(), app_definitions::Time{time: iso_timestamp[5..7].to_string(), parent: parent.clone(), time_type: app_definitions::TimeType::Month}.into()),
                        Entry::App("time".into(), app_definitions::Time{time: iso_timestamp[8..10].to_string(), parent: parent.clone(), time_type: app_definitions::TimeType::Day}.into()),
                        Entry::App("time".into(), app_definitions::Time{time: iso_timestamp[11..13].to_string(), parent: parent.clone(), time_type: app_definitions::TimeType::Hour}.into())];
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

pub fn get_time_address(time: app_definitions::Time) -> ZomeApiResult<Address> {
    Ok(hdk::api::entry_address(&Entry::App("time".into(), time.into()))?)
}