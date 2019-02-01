use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    api::DNA_ADDRESS,
    holochain_core_types::{
        cas::content::Address,
        hash::HashString
    }
};

use multihash::Hash;
use super::utils;

pub fn global_time_to_expression(tag: &'static str, direction: &String, expression_address: &Address) -> ZomeApiResult<String> {    
    //Check that current times exist and then link expression address to times
    //Get/create timestamps
    let timestamps: Vec<Address>;
    match utils::create_timestamps(HashString::encode_from_str(&DNA_ADDRESS.to_string(), Hash::SHA2256)){
        Ok(result) => timestamps = result,
        Err(hdk_err) => return Err(ZomeApiError::from("There was an error with creating/getting of timesamps".to_string()))
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