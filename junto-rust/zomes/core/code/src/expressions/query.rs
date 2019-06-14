use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        cas::content::Address,
        entry::Entry, 
        json::JsonString,
    }
};

use regex::Regex;
use std::convert::TryFrom;

//Our modules for holochain actins
use super::definitions::{
    app_definitions,
    function_definitions::{
        QueryTarget,
        QueryOptions,
        EntryAndAddress,
        QueryType
    }
};

use super::utils;
use super::dos;
use super::random;
use super::channel;

///Function to handle the getting of expression for a given perspective and query point(s)
///for example: perspective: dos & query_points: [2018<timestamp>, holochain<tag>, dht<tag>, eric<user>]
//TODO: Switch to normal Entry (JsonString as returned from get_entry & get_links) for EntryAndAddress across the whole application
//TODO/ORNOT: Support target_type of User
pub fn get_expression(perspective: String, query_points: Vec<String>, query_options: QueryOptions, target_type: QueryTarget, query_type: QueryType, dos: i32, seed: String) -> ZomeApiResult<JsonString> {
    let query_string = query_vec_to_string(query_points)?;
    hdk::debug(format!("Getting expressions with generated query string: {}", query_string))?;
    match perspective.as_ref() {
        "random" => {
            // match target_type {
            //     QueryTarget::ExpressionPost => {
            //         let expressions = get_expression::<app_definitions::ExpressionPost>(query_root, query_points, context, query_options, target_type, _query_type)?;
            //         Ok(JsonString::from(expressions))
            //     },
            //     QueryTarget::User => {
            //         let expressions = get_expression::<app_definitions::UserName>(query_root, query_points, context, query_options, target_type, _query_type)?;
            //         Ok(JsonString::from(expressions))
            //     }
            // }
            Ok(JsonString::from("random query"))
        },

        "dos" => {
            if dos < 1 || dos > 6{return Err(ZomeApiError::from("DOS not within bounds 1 -> 6".to_string()))};
            match target_type {
                QueryTarget::ExpressionPost => {
                    let expressions = dos::dos_query::<app_definitions::ExpressionPost>(query_string , query_options, target_type, query_type, dos, seed)?;
                    let mut out = vec![];
                    for exp in expressions{
                        match hdk::get_entry(&exp)? {
                            Some(Entry::App(_, entry_value)) => {
                                let entry = app_definitions::ExpressionPost::try_from(&entry_value).map_err(|_err| ZomeApiError::from("Links retreived from DOS query were not of type expression post".to_string()))?;
                                out.push(EntryAndAddress{entry: entry, address: exp});
                            },
                            Some(_) => return Err(ZomeApiError::from("Group address was not an app entry".to_string())),
                            None => return Err(ZomeApiError::from("No group entry at specified address".to_string()))
                        };
                    };
                    Ok(JsonString::from(out))
                },
                QueryTarget::User => {
                    let expressions = dos::dos_query::<app_definitions::UserName>(query_string , query_options, target_type, query_type, dos, seed)?;
                    let mut out = vec![];
                    for exp in expressions{
                        match hdk::get_entry(&exp)? {
                            Some(Entry::App(_, entry_value)) => {
                                let entry = app_definitions::UserName::try_from(&entry_value).map_err(|_err| ZomeApiError::from("Links retreived from DOS query were not of type username".to_string()))?;
                                out.push(EntryAndAddress{entry: entry, address: exp});
                            },
                            Some(_) => return Err(ZomeApiError::from("Group address was not an app entry".to_string())),
                            None => return Err(ZomeApiError::from("No group entry at specified address".to_string()))
                        };
                    };
                    Ok(JsonString::from(out))
                }
            }
        },

        _ => { //TODO: Add maximum post retrieval here - perhaps dont return over 50 posts - and posts should either be selected randomly or by a pagination query?
            hdk::debug("Attempting a perspective query")?;
            let perspective_address = Address::from(perspective);
            let perspective_users = channel::get_perspectives_users(perspective_address)?;
            let mut out = vec![];
        
            for user in perspective_users{
                let mut expressions = utils::get_links_and_load_type::<app_definitions::ExpressionPost>(&user.address, Some("expression_post".to_string()), Some(query_string.clone()))?;
                out.append(&mut expressions);
            };
            Ok(JsonString::from(out))
        }
    }
}

///Converts a query_vec to a query_string in the following format: tag1<tag>/tag2<tag>/tag3<tag>/tag4<tag>/user<user>/type<type>/time:y<time>/time:m<time>/time:d<time>/time:h<time> 
pub fn query_vec_to_string(query_points: Vec<String>) -> ZomeApiResult<String> {
    let re = Regex::new(r"(.*<*>)$").unwrap(); //regex to check that each query point is of syntax: value<type>
    let re_tag = Regex::new(r"(.*<tag>)$").unwrap(); //regex to match each query point is not optimal - should instead for match on whole joined string rather than on each item in vec
    let re_user = Regex::new(r"(.*<user>)$").unwrap();
    let re_type = Regex::new(r"(.*<type>)$").unwrap();
    let re_time = Regex::new(r"(.*<time:.>)$").unwrap();
    let mut tags = vec![];
    let mut user = vec![];
    let mut r#type = vec![];
    let mut times = vec![];

    for query_point in query_points{
        if re.is_match(&query_point) == false { //check that query point has type and value and of correct format
            return Err(ZomeApiError::from(format!("Invalid format for query point: {}", query_point)))
        };
        query_point.to_lowercase();
        if re_tag.is_match(&query_point){
            tags.push(query_point.clone())
        };
        if re_user.is_match(&query_point){
            user.push(query_point.clone())
        };
        if re_type.is_match(&query_point){
            r#type.push(query_point.clone())
        };
        if re_time.is_match(&query_point){
            times.push(query_point.clone())
        };
    };
    tags.sort_by(|a, b| b.cmp(&a));
    if tags.len() == 0 {for _ in 1..5{tags.push("*".to_string())};};
    if user.len() == 0 {user.push("*".to_string())};
    if r#type.len() == 0 {r#type.push("*".to_string())};

    if (user.len() > 1) | (r#type.len() > 1) {return Err(ZomeApiError::from(String::from("Invalid Query String")))};
    if tags.len() < 4 {for _ in tags.len()..5{tags.push("*".to_string());};};
    times = utils::sort_time_vector(times);
    Ok(format!("{}/{}/{}/{}", tags.join("/"), user[0], r#type[0], times.join("/")))
}