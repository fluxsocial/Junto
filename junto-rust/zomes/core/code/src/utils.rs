//Holochain core imports
use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    holochain_core_types::{
        cas::content::Address, 
        entry::Entry, 
        entry::AppEntryValue,
        error::HolochainError, 
        json::JsonString,
        hash::HashString
    }
};
use std::collections::HashMap;
use std::convert::TryFrom;

//Datetime imports
use chrono::{DateTime, Utc};

//Our module(s) imports
use super::definitions;
use super::user;

//Handle hooked objects that need to be created/linked for a given data type
pub fn handle_hooks(expression_type: String, parent_address: &Address, child_address: Option<&Address>) -> Result<String, ZomeApiError> {
    let hook_items: Vec<HashMap<&'static str, &'static str>>;
    match expression_type.as_ref(){
        "User" => hook_items = definitions::get_user_definitions().hooks,
        "Channel" => hook_items = definitions::get_channel_definitions().hooks,
        "ExpressionPost" => hook_items = definitions::get_post_expression_definitions().hooks,
        "Group" => hook_items = definitions::get_group_definitions().hooks,
        "Time" => hook_items = definitions::get_time_definitions().hooks,
        "Resonation" => hook_items = definitions::get_resonation_definitions().hooks,
        _ => return Err(ZomeApiError::from("Expression type does not exist".to_string()))
    }
    if hook_items.len() > 0{
        for hook_definition in hook_items{
            match hook_definition.get("function"){
                Some(&"global_time_to_expression") =>  {
                    user::global_time_to_expression(&hook_definition.get("tag").unwrap(), &hook_definition.get("direction").unwrap().to_string(), 
                                                    &parent_address)
                        .map_err(|err: ZomeApiError<>| err);
                },
                Some(&"create_pack") => {
                    user::create_pack(&parent_address)
                        .map_err(|err: ZomeApiError<>| err);
                },
                Some(&"create_den") => {
                    user::create_den(&parent_address)
                        .map_err(|err: ZomeApiError<>| err);
                },
                Some(&"pack_link") => {
                    match child_address{
                        Some(child_value) => {
                            user::pack_link(&hook_definition.get("tag").unwrap(), &hook_definition.get("direction").unwrap(), 
                                            parent_address, child_value)
                                .map_err(|err: ZomeApiError<>| err);
                        },
                        None => return Err(ZomeApiError::from("Child address must be specified for pack link".to_string()))
                    }
                },
                Some(&"link_user_channel") =>{
                    match child_address {
                        Some(child_value) => {
                            user::link_user_channel(&hook_definition.get("tag").unwrap(), &hook_definition.get("direction").unwrap(), 
                                                    &parent_address, child_value)
                                .map_err(|err: ZomeApiError<>| err);
                        },
                        None => return Err(ZomeApiError::from("Child address must be specified for pack link".to_string()))
                    }
                },
                None => {},
                _ => {}
            }
        }
    }
    Ok("Hooks created".to_string())
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
    for address in &timestamp_hashs{
        hdk::link_entries(&parent, &address, "time")?;
    } 

    Ok(timestamp_hashs)
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

pub fn get_as_type<R: TryFrom<AppEntryValue>> (address: HashString) -> ZomeApiResult<R> {
    let get_result = hdk::get_entry(&address)?;
    let entry = get_result.ok_or(ZomeApiError::Internal("No entry at this address".into()))?;
    match entry {
        Entry::App(_, entry_value) => {
            R::try_from(entry_value.to_owned())
                .map_err(|_| ZomeApiError::Internal(
                    "Could not convert get_links result to requested type".to_string())
                )
        },
        _ => Err(ZomeApiError::Internal(
            "get_links did not return an app entry".to_string())
        )
    }
}

// pub fn handle_contextual_links(expression_type: String, parentAddress: Address) -> Result<String, ZomeApiError> {
// }