//Holochain core imports
use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    holochain_core_types::{
        cas::content::Address, entry::Entry, dna::entry_types::Sharing, error::HolochainError, json::JsonString,
    }
};
//Datetime imports
use chrono::{DateTime, Utc};

//Our module(s) imports
use super::definitions;

//Handle hooked objects that need to be created/linked for a given data type
pub fn handle_hooks(expression_type: String, parentAddress: Address) -> Result<String, ZomeApiError> {
    let hook_items = definitions::USER_EXPRESSION_LINK_DEFINITIONS.hooks;
    Ok("Hooks created".to_string())
}

//Create and link current timestamps (year, month, day) to given parent address
pub fn create_timestamps(parent: Address) -> Result<String, ZomeApiError> {
    let now: DateTime<Utc> = Utc::now();
    let year = now.format("%Y").to_string();
    let month_y = now.format("%b %Y").to_string();
    let month = now.format("%b").to_string();
    let day_m_y = now.format("%b %e %Y").to_string();
    let day = now.format("%b").to_string();
    let timestamps = vec![year, month_y, month, day_m_y, day];
    let mut timestamp_hashs = vec![];

    //Iterate over timestamp objects and check that they exist
    for timestamp in timestamps{
        match get_timestamp(&timestamp, &parent){
            Ok(entry) => {
                match entry{
                    Some(entry) => timestamp_hashs.push(hdk::entry_address(&entry)), //Check entry and see how we can extract address
                    None => {//create timestamp with link
                        let time = definitions::Time {
                            timestamp: timestamp,
                            parent: parent
                        };
                        let entry = Entry::App("time".into(), time.into());
                        match hdk::commit_entry(&entry){
                            Ok(address) => timestamp_hashs.push(hdk::entry_address(&entry)),
                            Err(hdk_err) => return Err(ZomeApiError::from(hdk_err.into()))
                        } //Check that this is successful otherwise return error from main function scope
                    }
                }
            },
            Err(hdk_err) => return Err(ZomeApiError::from(hdk_err.into())) //Return error from main function scope
        }
    }

    for address in timestamp_hashs{
        // let hash = Entry::App("time".into(), timestamp.into());
        hdk::link_entries(&parent, &address, "time")?;
    } 

    Ok("Timestamps created and linked to parent object".to_string())
}

//Get timestamp entry by timestamp string w/ parent address
pub fn get_timestamp(timestamp: &String, parent: &Address) -> ZomeApiResult<Option<definitions::Time>> {
    let time = definitions::Time {
        timestamp: *timestamp,
        parent: *parent
    };
    let entry = Entry::App("time".into(), time.into());
    match hdk::entry_address(&entry){
        Ok(address) => hdk::get_entry(&address),
        Err(hdk_err) => ZomeApiResult::from(Err(hdk_err))
    }
}

// pub fn handle_contextual_links(expression_type: String, parentAddress: Address) -> Result<String, ZomeApiError> {
// }