use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_persistence_api::{
        cas::content::Address,
    },
    holochain_wasm_utils::api_serialization::get_entry::{
        GetEntryResultType,
        GetEntryOptions
    }
};

use std::collections::HashMap;

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