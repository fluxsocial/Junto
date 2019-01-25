//Holochain core imports
use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    holochain_core_types::{
        cas::content::Address, 
        entry::Entry, 
        error::HolochainError, 
        json::JsonString,
    }
};
//Datetime imports
use chrono::{DateTime, Utc};

//Our module(s) imports
use super::definitions;

//Handle hooked objects that need to be created/linked for a given data type
pub fn handle_hooks(expression_type: String, parent_address: &Address) -> Result<String, ZomeApiError> {
    let hook_items = definitions::get_user_definitions().hooks;
    Ok("Hooks created".to_string())
}

//Create and link current timestamps (year, month, day) to given parent address
pub fn create_timestamps(parent: Address) -> ZomeApiResult<String> {
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
        get_timestamp(&timestamp, &parent)
            .map(|return_timestamp: Option<Entry>| {
                match return_timestamp {
                    Some(entry) => {
                        match hdk::entry_address(&entry){
                            Ok(address) => {
                                timestamp_hashs.push(address);
                            },
                            Err(hdk_err) => return Err(hdk_err)
                        };
                        Ok(())
                    },
                    None => {
                        let time = definitions::Time {
                            timestamp: timestamp.clone(),
                            parent: parent.clone()
                        };
                        let entry = Entry::App("time".into(), time.into());
                        match hdk::commit_entry(&entry){
                            Ok(address) => {
                                timestamp_hashs.push(address);
                            },
                            Err(hdk_err) => return Err(hdk_err)
                        };
                        Ok(())
                    }
                }
            })
            .map_err(|err: ZomeApiError<>| return ZomeApiError::from(err.to_string()));
    }
    //println!("{:?}", &timestamps);
    for address in timestamp_hashs{
        hdk::link_entries(&parent, &address, "time")?;
    } 

    Ok("Timestamps created and linked to parent object".to_string())
}

//Get timestamp entry by timestamp string w/ parent address
pub fn get_timestamp(timestamp: &String, parent: &Address) -> ZomeApiResult<Option<Entry>> {
    let time = definitions::Time {
        timestamp: timestamp.clone(),
        parent: parent.clone()
    };
    let entry = Entry::App("time".into(), time.into());
    hdk::entry_address(&entry)
        .map(|address: Address| hdk::get_entry(&address))
        .and_then(|result: ZomeApiResult<Option<Entry>>| result)
        .or_else(|err: ZomeApiError<>| Err(err))
}

// pub fn handle_contextual_links(expression_type: String, parentAddress: Address) -> Result<String, ZomeApiError> {
// }