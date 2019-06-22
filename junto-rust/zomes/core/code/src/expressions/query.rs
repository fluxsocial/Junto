use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        cas::content::Address,
        entry::Entry, 
        json::JsonString,
        hash::HashString
    }
};

use regex::Regex;
use std::convert::TryFrom;
use itertools::Itertools;
use multihash::Hash;

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
use super::perspective;

///Function to handle the getting of expression for a given perspective and query point(s)
///for example: perspective: dos & query_points: [2018<timestamp>, holochain<channel>, dht<channel>, eric<channel>]
//TODO: Switch to normal Entry (JsonString as returned from get_entry & get_links) for EntryAndAddress across the whole application
pub fn get_expression(perspective: String, attributes: Vec<String>, query_options: QueryOptions, target_type: QueryTarget, 
                        query_type: QueryType, dos: u32, seed: String) -> ZomeApiResult<JsonString> {
    let index_strings = attributes_to_index_string(attributes)?;
    hdk::debug(format!("Getting expressions with generated query string(s): {:?}", index_strings))?;
    match perspective.as_ref() {
        "random" => {
            let seed = HashString::encode_from_str(&seed, Hash::SHA2256);
            hdk::debug(format!("Seed addresss: {}", seed.to_string()))?;
            let current_bit_prefix = random::get_current_bit_prefix()?;
            let bit_prefix_bucket_id = random::hash_prefix(Address::from(seed), current_bit_prefix); //get and id for bucket to make query from using seed passed into function
            hdk::debug(format!("Making random query with bit prefix: {}", bit_prefix_bucket_id))?;
            let bit_prefix_bucket = hdk::entry_address(&Entry::App("bucket".into(), app_definitions::Bucket{id: bit_prefix_bucket_id}.into()))?;
            let mut results = vec![];
            for index_string in &index_strings{
                results.append(&mut hdk::get_links(&bit_prefix_bucket, Some(String::from("expression_post")), Some(index_string.clone()))?.addresses());
            };
            match target_type{
                QueryTarget::ExpressionPost => {
                    let mut out = vec![];
                    for result in results{
                        match hdk::get_entry(&result)?{
                            Some(Entry::App(_, entry_value)) => {
                                let entry = app_definitions::ExpressionPost::try_from(&entry_value).map_err(|_err| ZomeApiError::from("Links retreived from random query were not of type expression post".to_string()))?;
                                let expression_data = utils::get_expression_attributes(EntryAndAddress{entry: entry, address: result})?;
                                out.push(expression_data);
                            },
                            Some(_) => {},
                            None => {}
                        };
                    };
                    Ok(JsonString::from(out))
                },
                QueryTarget::User => {
                    let mut out = vec![];
                    for result in results{
                        out.push(utils::get_links_and_load_type::<app_definitions::UserName>(&result, Some(String::from("auth")), Some(String::from("owner")))?[0].clone());
                    };
                    Ok(JsonString::from(out.into_iter().unique().collect::<Vec<_>>()))
                }
            }
        },

        "dos" => {
            if dos < 1 || dos > 6{return Err(ZomeApiError::from("DOS not within bounds 1 -> 6".to_string()))};
            let mut expressions = dos::dos_query(index_strings, query_options, query_type, dos, seed)?;
            expressions = expressions.into_iter().unique().collect::<Vec<_>>(); //ensure all posts returned are unique
            match target_type{
                QueryTarget::ExpressionPost => {
                    let mut out = vec![];
                    for expression in expressions{
                        match hdk::get_entry(&expression)?{
                            Some(Entry::App(_, entry_value)) => {
                                let entry = app_definitions::ExpressionPost::try_from(&entry_value).map_err(|_err| ZomeApiError::from("Links retreived from DOS query were not of type expression post".to_string()))?;
                                let expression_data = utils::get_expression_attributes(EntryAndAddress{entry: entry, address: expression})?;
                                out.push(expression_data);
                            },
                            Some(_) => return Err(ZomeApiError::from("Group address was not an app entry".to_string())),
                            None => return Err(ZomeApiError::from("No group entry at specified address".to_string()))
                        };
                    };
                    Ok(JsonString::from(out))
                },
                QueryTarget::User => {
                    let mut out = vec![];
                    for expression in expressions{
                        out.push(utils::get_links_and_load_type::<app_definitions::UserName>(&expression, Some(String::from("auth")), Some(String::from("owner")))?[0].clone());
                    };
                    Ok(JsonString::from(out.into_iter().unique().collect::<Vec<_>>()))
                }
            }
        },

        _ => { //TODO: Add maximum post retrieval here - perhaps dont return over 50 posts - and posts should either be selected randomly or by a pagination query?
            hdk::debug("Attempting a perspective query")?;
            let perspective_address = Address::from(perspective);
            let perspective_users = perspective::get_perspectives_users(perspective_address)?;
            let mut out = vec![];
        
            for user in perspective_users{
                let mut expressions = vec![];
                for index_string in &index_strings{
                    expressions.append(&mut utils::get_links_and_load_type::<app_definitions::ExpressionPost>(&user.address, Some("expression_post".to_string()), Some(index_string.clone()))?);
                };
                let mut expressions = expressions.into_iter().map(|expression| utils::get_expression_attributes(expression)).collect::<Result<Vec<_>,_>>()?;
                out.append(&mut expressions);
            };
            Ok(JsonString::from(out))
        }
    }
}

///Converts an attributes vector to a index_string in the following format: 
///tag1<channel>/tag2<channel>/tag3<channel>/tag4<channel>/user<user>/type<type>/time:y<time>/time:m<time>/time:d<time>/time:h<time> 
pub fn attributes_to_index_string(attributes: Vec<String>) -> ZomeApiResult<Vec<String>> {
    let re = Regex::new(r"(.*<*>)$").unwrap(); //regex to check that each attribute point is of syntax: value<type>
    let re_channel = Regex::new(r"(.*<channel>)$").unwrap(); //regex to match each attribute point is not optimal - should instead for match on whole joined string rather than on each item in vec
    let re_user = Regex::new(r"(.*<user>)$").unwrap();
    let re_type = Regex::new(r"(.*<type>)$").unwrap();
    let re_time = Regex::new(r"(.*<time:.>)$").unwrap();
    let mut channels = vec![];
    let mut user = vec![];
    let mut r#type = vec![];
    let mut times = vec![];

    for mut attribute in attributes{
        attribute = attribute.to_lowercase();
        if re.is_match(&attribute) == false { //check that attribute has type and value and of correct format
            return Err(ZomeApiError::from(format!("Invalid format for attribute: {}", attribute)))
        };
        attribute.to_lowercase();
        if re_channel.is_match(&attribute){
            channels.push(attribute.clone())
        };
        if re_user.is_match(&attribute){
            user.push(attribute.clone())
        };
        if re_type.is_match(&attribute){
            r#type.push(attribute.clone())
        };
        if re_time.is_match(&attribute){
            times.push(attribute.clone())
        };
    };
    channels.sort_by(|a, b| b.cmp(&a));
    if channels.len() == 0 {for _ in 1..5{channels.push("*".to_string())};};
    if user.len() == 0 {user.push("*".to_string())};
    if r#type.len() == 0 {r#type.push("*".to_string())};
    if times.len() > 4 {return Err(ZomeApiError::from("Invalid query string".to_string()))};
    if (user.len() > 1) | (r#type.len() > 1) {return Err(ZomeApiError::from(String::from("Invalid query string")))};
    let channels = get_channel_combinations(channels.clone())?;
    times = utils::sort_time_vector(times);

    let out = channels.into_iter().map(|channel_combination| format!("{}/{}/{}/{}", channel_combination.join("/"), user[0], r#type[0], times.join("/"))).collect::<Vec<String>>();
    Ok(out)
}

pub fn get_channel_combinations(mut channels: Vec<String>) -> ZomeApiResult<Vec<Vec<String>>> {
    let mut out = vec![];

    match channels.len(){
        4 => out.push(channels.clone()),
        3 => {
            let mut current_tag_combination = vec!["*".to_string()];
            current_tag_combination.append(&mut channels.clone());
            out.push(current_tag_combination);
            channels.append(&mut vec!["*".to_string()]);
            out.push(channels.clone());
        },
        2 => {
            let mut current_tag_combination = vec!["*".to_string()];
            current_tag_combination.append(&mut channels.clone());
            current_tag_combination.append(&mut vec!["*".to_string()]);
            out.push(current_tag_combination);
            current_tag_combination = vec!["*".to_string(), "*".to_string()];
            current_tag_combination.append(&mut channels.clone());
            out.push(current_tag_combination);
            channels.append(&mut vec!["*".to_string(), "*".to_string()]);
            out.push(channels.clone());
        },
        1 => {
            out.push(vec![channels[0].clone(), "*".to_string(), "*".to_string(), "*".to_string()]);
            out.push(vec!["*".to_string(), channels[0].clone(), "*".to_string(), "*".to_string()]);
            out.push(vec!["*".to_string(), "*".to_string(), channels[0].clone(), "*".to_string()]);
            out.push(vec!["*".to_string(), "*".to_string(), "*".to_string(), channels[0].clone()]);
        },
        0 => {
            out.push(vec!["*".to_string(), "*".to_string(), "*".to_string(), "*".to_string()]);
        },
        _ => {
            return Err(ZomeApiError::from("Invalid attribute string".to_string()))
        }
    };
    Ok(out)
}