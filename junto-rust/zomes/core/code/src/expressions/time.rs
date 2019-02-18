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

use super::definitions;

pub fn global_time_to_expression(tag: &'static str, direction: &String, expression_address: &Address) -> ZomeApiResult<String> {    
    //Check that current times exist and then link expression address to times
    //Get/create timestamps
    let timestamps: Vec<Address>;
    match create_timestamps(HashString::encode_from_str(&DNA_ADDRESS.to_string(), Hash::SHA2256)){
        Ok(result) => timestamps = result,
        Err(_hdk_err) => return Err(ZomeApiError::from("There was an error with creating/getting of timestamps".to_string()))
    };

    if (direction == "reverse") | (direction == "both"){
        for timestamp in &timestamps{
            hdk::link_entries(&timestamp, &expression_address, tag)?;
        }
    }
    if (direction == "forward") | (direction == "both"){
        for timestamp in &timestamps{
            hdk::link_entries(&expression_address, &timestamp, tag)?;
        }
    }
    Ok("Expression linked to global time object(s)".to_string())
}

pub fn local_time_to_expression(tag: &'static str, direction: &String, expression_address: &Address, context: &Address) -> ZomeApiResult<String> {
    //does the same as global_time_to_expression but accepts a context address which allows us to check for time(s) inside expression channels such as dens or packs
    let timestamps: Vec<Address>;
    match create_timestamps(context){
        Ok(result) => timestamps = result,
        Err(_hdk_err) => return Err(ZomeApiError::from("There was an error with creating/getting of timestamps".to_string()))
    };

    if (direction == "reverse") | (direction == "both"){
        for timestamp in &timestamps{
            hdk::link_entries(&timestamp, &expression_address, tag)?;
        }
    }
    if (direction == "forward") | (direction == "both"){
        for timestamp in &timestamps{
            hdk::link_entries(&expression_address, &timestamp, tag)?;
        }
    }
    Ok("Expression linked to global time object(s)".to_string())
}

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

//Create and link current timestamps (year, month, day) to given parent address
//will return vector of each timestamp
pub fn create_timestamps(parent: Address) -> ZomeApiResult<Vec<Address>> {
    let timestamps: Vec<String> = get_current_timestamps();
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
                        let time = definitions::app_definitions::Time {
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
    //println!("{:?}", &timestamps);
    for address in &timestamp_hashs{
        hdk::link_entries(&parent, &address, "time")?;
    } 

    Ok(timestamp_hashs)
}

//Get timestamp entry by timestamp string w/ parent address
pub fn get_timestamp(timestamp: &String, parent: &Address) -> ZomeApiResult<Option<Entry>> {
    let time = definitions::app_definitions::Time {
        timestamp: timestamp.clone(),
        parent: parent.clone()
    };
    let entry = Entry::App("time".into(), time.into());
    hdk::entry_address(&entry)
        .map(|address: Address| hdk::get_entry(&address))
        .and_then(|result: ZomeApiResult<Option<Entry>>| result)
        .or_else(|err: ZomeApiError<>| Err(err))
}
