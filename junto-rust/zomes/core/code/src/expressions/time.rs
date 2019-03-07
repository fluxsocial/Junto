use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    api::{
        DNA_ADDRESS,
        get_entry
    },
    holochain_core_types::{
        entry::Entry,
        cas::content::Address,
        hash::HashString
    }
};

//Datetime imports
use multihash::Hash;

use super::utils;
use super::definitions::{
    app_definitions
};

//Creates timestamp entrys on the global DHT if they do not exist and then links given expression to the timestamps
pub fn global_time_to_expression(tag: &'static str, direction: &'static str, expression_address: &Address) -> ZomeApiResult<String> {    
    let mut timestamps: Vec<Address> = vec![];
    let expression_entry = get_entry(expression_address)?;
    match expression_entry{
        Some(Entry::ChainHeader(header)) => {
            let iso_timestamp = serde_json::to_string(header.timestamp());
            match iso_timestamp{
                Ok(iso_timestamp) => {
                    match create_timestamps(&HashString::encode_from_str(&DNA_ADDRESS.to_string(), Hash::SHA2256), iso_timestamp){
                        Ok(result) => timestamps = result,
                        Err(hdk_err) => return Err(ZomeApiError::from(hdk_err.to_string()))
                    };
                },
                Err(hdk_err) => return Err(ZomeApiError::from(hdk_err.to_string()))
            }
        },
        Some(_) => {},
        None => return Err(ZomeApiError::from("No such expression at expression_address".to_string()))
    }

    for timestamp in &timestamps{
        utils::link_expression(tag, direction, timestamp, expression_address);
    }

    Ok("Expression linked to global time object(s)".to_string())
}

//Creates timestamp entrys on the global DHT but with expression parent specified (context) - this allows us to create unique timestamp objects for each expression "center" such as a user or group
pub fn local_time_to_expression(tag: &'static str, direction: &'static str, expression_address: &Address, context: &Address) -> ZomeApiResult<String> {
    let mut timestamps: Vec<Address> = vec![];
    let expression_entry = get_entry(expression_address)?;
    match expression_entry{
        Some(Entry::ChainHeader(header)) => {
            let iso_timestamp = serde_json::to_string(header.timestamp());
            match iso_timestamp{
                Ok(iso_timestamp) => {
                    match create_timestamps(context, iso_timestamp){
                        Ok(result) => timestamps = result,
                        Err(hdk_err) => return Err(ZomeApiError::from(hdk_err.to_string()))
                    };
                },
                Err(hdk_err) => return Err(ZomeApiError::from(hdk_err.to_string()))
            }
        },
        Some(_) => {},
        None => return Err(ZomeApiError::from("No such expression at expression_address".to_string()))
    }

    for timestamp in &timestamps{
        utils::link_expression(tag, direction, timestamp, expression_address);
    }

    Ok("Expression linked to global time object(s)".to_string())
}

//Create timestamp functions should not use datatime but instead should use the timestamp in the entry header for the expression that will be linked to the timestamp
//Create and link current timestamps (year, month, day) to given parent address - returns vector of timestamps
pub fn create_timestamps(parent: &Address, iso_timestamp: String) -> ZomeApiResult<Vec<Address>> {
    let timestamps = vec![iso_timestamp[0..4].to_string(), iso_timestamp[5..7].to_string(), iso_timestamp[8..10].to_string(), iso_timestamp[11..13].to_string()]; //Slice timestamp into vector as year, month, day, hour
    let mut timestamp_hashs = vec![];

    for (i, timestamp) in timestamps.iter().enumerate(){
        match i{
            0 => {
                let hash = save_timestamp(timestamp, "year".to_string(), &parent);
                match hash{
                    Ok(hash) => timestamp_hashs.push(hash),
                    Err(hdk_err) => return Err(hdk_err)
                }
            },
            1 => {
                let hash = save_timestamp(timestamp, "month".to_string(), &parent);
                match hash{
                    Ok(hash) => timestamp_hashs.push(hash),
                    Err(hdk_err) => return Err(ZomeApiError::from(hdk_err.to_string()))
                }
            },
            2 => {
                let hash = save_timestamp(timestamp, "day".to_string(), &parent);
                match hash{
                    Ok(hash) => timestamp_hashs.push(hash),
                    Err(hdk_err) => return Err(ZomeApiError::from(hdk_err.to_string()))
                }
            },
            3 => {
                let hash = save_timestamp(timestamp, "hour".to_string(), &parent);
                match hash{
                    Ok(hash) => timestamp_hashs.push(hash),
                    Err(hdk_err) => return Err(ZomeApiError::from(hdk_err.to_string()))
                }
            },
            _ => {}
        }
    };

    for address in &timestamp_hashs{
        hdk::link_entries(&parent, &address, "time")?;
    };

    Ok(timestamp_hashs)
}

pub fn save_timestamp(timestamp: &String, time_type: String, parent: &Address) -> ZomeApiResult<Address>{
    //Check that timestamp object exists if not create it
    match get_timestamp(&timestamp, &parent, &time_type){
        Ok(Some(entry)) => {
            match Some(entry){
                Some(entry) => {
                    match hdk::entry_address(&entry){
                        Ok(address) => return Ok(address),
                        Err(hdk_err) => return Err(hdk_err)
                    };
                },
                None => { //This code is a little messy and should be refactored - currently we are matching for the same thing twice - need to try and figure out how to refactor this TODO
                    let time = app_definitions::Time {
                        time: timestamp.clone(),
                        parent: parent.clone(),
                        time_type: time_type.clone()
                    };
                    let entry = Entry::App("time".into(), time.into());
                    match hdk::commit_entry(&entry){
                        Ok(address) => return Ok(address),
                        Err(hdk_err) => return Err(hdk_err)
                    };
                }
            } 
        },
        Ok(None) => {
            let time = app_definitions::Time {
                time: timestamp.clone(),
                parent: parent.clone(),
                time_type: time_type.clone()
            };
            let entry = Entry::App("time".into(), time.into());
            match hdk::commit_entry(&entry){
                Ok(address) => return Ok(address),
                Err(hdk_err) => return Err(hdk_err)
            };
        },
        Err(hdk_err) => return Err(hdk_err)
    }
}

//Get timestamp entry by timestamp string w/ parent address
pub fn get_timestamp(timestamp: &String, parent: &Address, time_type: &String) -> ZomeApiResult<Option<Entry>> {
    let time = app_definitions::Time {
        time: timestamp.clone(),
        parent: parent.clone(),
        time_type: time_type.clone()
    };
    let entry = Entry::App("time".into(), time.into());
    hdk::entry_address(&entry)
        .map(|address: Address| hdk::get_entry(&address))
        .and_then(|result: ZomeApiResult<Option<Entry>>| result)
        .or_else(|err: ZomeApiError<>| Err(err))
}
