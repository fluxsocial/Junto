use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    api::DNA_ADDRESS,
    holochain_core_types::{
        entry::Entry,
        cas::content::Address,
        hash::HashString
    }
};

//Datetime imports
use chrono::{DateTime, Utc};
use multihash::Hash;

use super::utils;
use super::definitions::{
    app_definitions
};

//Creates timestamp entrys on the global DHT if they do not exist and then links given expression to the timestamps
pub fn global_time_to_expression(tag: &'static str, direction: &'static str, expression_address: &Address) -> ZomeApiResult<String> {    
    let timestamps: Vec<Address>;
    match create_timestamps(&HashString::encode_from_str(&DNA_ADDRESS.to_string(), Hash::SHA2256)){
        Ok(result) => timestamps = result,
        Err(_hdk_err) => return Err(ZomeApiError::from("There was an error with creating/getting of timestamps".to_string()))
    };

    for timestamp in &timestamps{
        utils::link_expression(tag, direction, timestamp, expression_address);
    }

    Ok("Expression linked to global time object(s)".to_string())
}

//Creates timestamp entrys on the global DHT but with expression parent specified (context) - this allows us to create unique timestamp objects for each expression "center" such as a user or group
pub fn local_time_to_expression(tag: &'static str, direction: &'static str, expression_address: &Address, context: &Address) -> ZomeApiResult<String> {
    let timestamps: Vec<Address>;
    match create_timestamps(context){
        Ok(result) => timestamps = result,
        Err(_hdk_err) => return Err(ZomeApiError::from("There was an error with creating/getting of timestamps".to_string()))
    };

    for timestamp in &timestamps{
        utils::link_expression(tag, direction, timestamp, expression_address);
    }

    Ok("Expression linked to global time object(s)".to_string())
}

//Get current times
pub fn get_current_timestamps() -> Vec<String>{
    let now: DateTime<Utc> = Utc::now();
    let year = now.format("%Y").to_string();
    let month_y = now.format("%b %Y").to_string();
    let month = now.format("%b").to_string();
    let day_m_y = now.format("%b %e %Y").to_string();
    let day = now.format("%b").to_string();
    let timestamps = vec![year, month_y, month, day_m_y, day];
    timestamps
}

//Create and link current timestamps (year, month, day) to given parent address - returns vector of timestamps
pub fn create_timestamps(parent: &Address) -> ZomeApiResult<Vec<Address>> {
    let timestamps: Vec<String> = get_current_timestamps();
    let mut timestamp_hashs = vec![];

    //Iterate over timestamp objects and check that they exist - if not create them
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
                        let time = app_definitions::Time {
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
            .map_err(|err: ZomeApiError<>| ZomeApiError::Internal("There was an error getting timestamps".to_string()));
    }

    for address in &timestamp_hashs{
        hdk::link_entries(&parent, &address, "time")?;
    } 

    Ok(timestamp_hashs)
}

//Get timestamp entry by timestamp string w/ parent address
pub fn get_timestamp(timestamp: &String, parent: &Address) -> ZomeApiResult<Option<Entry>> {
    let time = app_definitions::Time {
        timestamp: timestamp.clone(),
        parent: parent.clone()
    };
    let entry = Entry::App("time".into(), time.into());
    hdk::entry_address(&entry)
        .map(|address: Address| hdk::get_entry(&address))
        .and_then(|result: ZomeApiResult<Option<Entry>>| result)
        .or_else(|err: ZomeApiError<>| Err(err))
}
