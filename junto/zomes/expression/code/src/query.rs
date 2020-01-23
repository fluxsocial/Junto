use hdk::{
    error::{
        ZomeApiResult,
        ZomeApiError
    },
    holochain_core_types::{
        link::LinkMatch,
        entry::Entry
    },
    holochain_persistence_api::{
        cas::content::Address,
        hash::HashString
    },
    holochain_json_api::{
        json::JsonString
    }
};

use regex::Regex;
use std::convert::TryFrom;
use std::convert::TryInto;
use itertools::Itertools;
use multihash::Hash;

use types::{
    app_definition,
    function_definition::{
        self,
        QueryTarget,
        QueryOptions,
        EntryAndAddress,
        QueryType,
        ExpressionData
    }
};
use utils;

use super::indexing;
use super::dos;

///Function to handle the getting of expression for a given perspective and query point(s)
///for example: perspective: dos & query_points: [2018<timestamp>, holochain<channel>, dht<channel>, eric<channel>]
//TODO: Switch to normal Entry (JsonString as returned from get_entry & get_links) for EntryAndAddress across the whole application
pub fn query_expressions(perspective: String, attributes: Vec<String>, query_options: QueryOptions, target_type: QueryTarget, 
                        query_type: QueryType, dos: u32, seed: String, resonations: bool) -> ZomeApiResult<JsonString> {
    let index_strings = attributes_to_index_string(attributes)?;
    hdk::debug(format!("Getting expressions with generated query string(s): {:?}", index_strings))?;
    match perspective.as_ref() {
        "random" => {
            let seed = HashString::encode_from_str(&seed, Hash::SHA2256);
            hdk::debug(format!("Seed addresss: {}", seed.to_string()))?;
            let current_bit_prefix = hdk::call(hdk::THIS_INSTANCE, "config", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                                        "get_current_bit_prefix", JsonString::from(json!({})))?;
            let current_bit_prefix: ZomeApiResult<u32> = current_bit_prefix.try_into()?;
            let current_bit_prefix = current_bit_prefix?;
            let bit_prefix_bucket_id = utils::helpers::hash_prefix(Address::from(seed), current_bit_prefix); //get and id for bucket to make query from using seed passed into function
            hdk::debug(format!("Making random query with bit prefix: {}", bit_prefix_bucket_id))?;
            let bit_prefix_bucket = hdk::entry_address(&Entry::App("bucket".into(), app_definition::Bucket{id: bit_prefix_bucket_id}.into()))?;
            query_from_address(Some(&bit_prefix_bucket), Some(index_strings), target_type, None, false, Some("bucket_expression_post"))
        },

        "dos" => {
            if dos < 1 || dos > 6 {return Err(ZomeApiError::from("DOS not within bounds 1 -> 6".to_string()))};
            let mut expressions = dos::dos_query(index_strings, query_options, query_type, dos, seed, resonations)?;
            expressions = expressions.into_iter().unique().collect::<Vec<_>>(); //ensure all posts returned are unique
            query_from_address(None, None, target_type, Some(expressions), false, None)
        },

        _ => { //TODO: Add maximum post retrieval here - perhaps dont return over 50 posts - and posts should either be selected randomly or by a pagination query?
            hdk::debug("Making either a group, perspective or collection query")?;
            let current_agent_username = utils::helpers::call_and_get_current_user_username()?;
            let context_address = Address::from(perspective);

            match indexing::run_context_auth(&context_address, &current_agent_username.address){
                Ok(Some(function_definition::ContextAuthResult::Collection(_context_entry))) => {
                    hdk::debug("Making a collection query")?;
                    query_from_address(Some(&context_address), Some(index_strings), target_type, None, false, Some("collection_expression_post"))
                },
                Ok(Some(function_definition::ContextAuthResult::Group(_context_entry))) => {
                    hdk::debug("Making a group query")?;
                    query_from_address(Some(&context_address), Some(index_strings), target_type, None, resonations, Some("group_expression_post"))
                },
                Ok(None) => { 
                    hdk::debug("Making a perspective query")?;
                    let perspective_users = hdk::call(hdk::THIS_INSTANCE, "perspective", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                                        "get_perspectives_users", JsonString::from(json!({"perspective": context_address})))?;
                    let perspective_users: ZomeApiResult<Vec<EntryAndAddress<app_definition::UserName>>> = perspective_users.try_into()?;
                    let mut out = vec![];
                
                    for user in perspective_users?{
                        let mut expressions = vec![];
                        for index_string in &index_strings{
                            if resonations == true{
                                expressions.append(&mut utils::helpers::get_links_and_load_type::<app_definition::ExpressionPost>(&user.address, LinkMatch::Exactly("resonation"), LinkMatch::Regex(index_string))?);
                            } else {
                                expressions.append(&mut utils::helpers::get_links_and_load_type::<app_definition::ExpressionPost>(&user.address, LinkMatch::Exactly("expression_post"), LinkMatch::Regex(index_string))?);
                            }
                        };
                        let mut expressions = expressions.into_iter().map(|expression| get_expression_attributes(expression, true)).collect::<Result<Vec<_>,_>>()?;
                        out.append(&mut expressions);
                    };
                    Ok(JsonString::from(out))
                }
                Err(err) => {
                    hdk::debug("Error invalid auth on perspective")?;
                    Err(err)
                }
            }
        }
    }
}

pub fn get_expression(expression: Address) -> ZomeApiResult<function_definition::ExpressionData>{
    match hdk::get_entry(&expression)? {
        Some(Entry::App(_, entry_value)) => {
            let entry = app_definition::ExpressionPost::try_from(&entry_value).map_err(|_err| ZomeApiError::from("Links retreived from query were not of type expression post".to_string()))?;
            Ok(get_expression_attributes(EntryAndAddress{entry: entry, address: expression}, true)?)
        },
        Some(_) => Err(ZomeApiError::from("Expression address was not an app entry".to_string())),
        None => Err(ZomeApiError::from("No perspective entry at specified address".to_string()))
    }
}

pub fn query_from_address(anchor: Option<&Address>, index_strings: Option<Vec<String>>, target_type: QueryTarget, 
                            results: Option<Vec<Address>>, resonations: bool, link_type: Option<&'static str>) -> ZomeApiResult<JsonString> {
    
    let results = results.unwrap_or_else(|| {
        let mut expressions = vec![];
        for index_string in index_strings.unwrap(){
            if resonations == true{
                expressions.append(&mut hdk::get_links(anchor.unwrap(), LinkMatch::Exactly("resonation"), LinkMatch::Regex(index_string.as_str())).unwrap().addresses());
            } else {
                expressions.append(&mut hdk::get_links(anchor.unwrap(), LinkMatch::Exactly(link_type.unwrap()), LinkMatch::Regex(index_string.as_str())).unwrap().addresses());
            };
        };
        expressions.into_iter().unique().collect::<Vec<_>>() 
    });

    match target_type{
        QueryTarget::ExpressionPost => {
            let mut out = vec![];
            for result in results{
                match hdk::get_entry(&result)?{
                    Some(Entry::App(_, entry_value)) => {
                        let entry = app_definition::ExpressionPost::try_from(&entry_value).map_err(|_err| ZomeApiError::from("Links retreived from query were not of type expression post".to_string()))?;
                        out.push(get_expression_attributes(EntryAndAddress{entry: entry, address: result}, true)?);
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
                out.push(utils::helpers::get_links_and_load_type::<app_definition::UserName>(&result, LinkMatch::Exactly("expression_auth"), LinkMatch::Exactly("owner"))?[0].clone());
            };
            Ok(JsonString::from(out.into_iter().unique().collect::<Vec<_>>()))
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
        if re_channel.is_match(&attribute){
            channels.push(attribute.clone());
        };
        if re_user.is_match(&attribute){
            user.push(attribute.clone());
        };
        if re_type.is_match(&attribute){
            r#type.push(attribute.clone());
        };
        if re_time.is_match(&attribute){
            times.push(attribute.clone());
        };
    };
    channels.sort_by(|a, b| b.cmp(&a));
    if user.len() == 0 {user.push(".+?".to_string())}; //push "any" regex matcher to string
    if r#type.len() == 0 {r#type.push(".+?".to_string())};
    if (user.len() > 1) | (r#type.len() > 1) | (times.len() > 4) {return Err(ZomeApiError::from(String::from("Invalid query string")))};

    let channels = channels.iter().map(|channel| channel.as_str()).collect::<Vec<&str>>();
    let channels = get_channel_combinations(channels)?;
    let mut times = times.iter().map(|time| time.as_str()).collect::<Vec<&str>>();
    times = utils::time::sort_time_vector(times);

    let out = channels.into_iter().map(|channel_combination| format!("/{}/{}/{}/{}/", channel_combination.join("/"), user[0], r#type[0], times.join("/"))).collect::<Vec<String>>();
    Ok(out)
}

//Get possible combinations of channels within the four channel slots. 
//If all slots are not filled it will return combinations of channels with empty slots filled with "any" regex matcher
pub fn get_channel_combinations(mut channels: Vec<&str>) -> ZomeApiResult<Vec<Vec<&str>>> {
    let mut out = vec![];

    match channels.len(){
        4 => out.push(channels.clone()),
        3 => {
            let mut current_tag_combination = vec![".+?"];
            current_tag_combination.append(&mut channels.clone());
            out.push(current_tag_combination);
            channels.append(&mut vec![".+?"]);
            out.push(channels.clone());
        },
        2 => {
            let mut current_tag_combination = vec![".+?"];
            current_tag_combination.append(&mut channels.clone());
            current_tag_combination.append(&mut vec![".+?"]);
            out.push(current_tag_combination);
            current_tag_combination = vec![".+?", ".+?"];
            current_tag_combination.append(&mut channels.clone());
            out.push(current_tag_combination);
            channels.append(&mut vec![".+?", ".+?"]);
            out.push(channels.clone());
        },
        1 => {
            out.push(vec![channels[0].clone(), ".+?", ".+?", ".+?"]);
            out.push(vec![".+?", channels[0].clone(), ".+?", ".+?"]);
            out.push(vec![".+?", ".+?", channels[0].clone(), ".+?"]);
            out.push(vec![".+?", ".+?", ".+?", channels[0].clone()]);
        },
        0 => {
            out.push(vec![".+?", ".+?", ".+?", ".+?"]);
        },
        _ => {
            return Err(ZomeApiError::from("Invalid attribute string".to_string()))
        }
    };
    Ok(out)
}

pub fn get_expression_attributes(expression_data: EntryAndAddress<app_definition::ExpressionPost>, fetch_sub_expressions: bool) -> ZomeApiResult<ExpressionData> {
    let user = utils::helpers::get_links_and_load_type::<app_definition::UserName>(&expression_data.address, LinkMatch::Exactly("expression_auth"), LinkMatch::Exactly("owner"))?;
    let profile = utils::helpers::get_links_and_load_type::<app_definition::User>(&user[0].address, LinkMatch::Exactly("profile"), LinkMatch::Any)?;
    let timestamp = utils::time::get_entries_timestamp(&expression_data.address)?;
    let channels = utils::helpers::get_links_and_load_type::<app_definition::Attribute>(&expression_data.address, LinkMatch::Exactly("channels"), LinkMatch::Any)?;
    let sub_expressions_count = hdk::api::get_links_count(&expression_data.address, LinkMatch::Exactly("expression_sub_expression"), LinkMatch::Any)?.count as f32;
    let mut sub_expressions = vec![];
    if fetch_sub_expressions == true {
        hdk::debug("Getting sub expressions")?;
        sub_expressions = utils::helpers::get_links_and_load_type::<app_definition::ExpressionPost>(&expression_data.address, LinkMatch::Exactly("expression_sub_expression"), LinkMatch::Any)?
                                .into_iter().map(|sub_expression| get_expression_attributes(sub_expression, false)).collect::<Result<Vec<_>,_>>()?;
    };
    let resonations = utils::helpers::get_links_and_load_type::<app_definition::UserName>(&expression_data.address, LinkMatch::Exactly("resonator"), LinkMatch::Any)?;

    Ok(ExpressionData{expression: expression_data, sub_expressions: sub_expressions, sub_expressions_count: sub_expressions_count, author_username: user[0].clone(), author_profile: profile[0].clone(), 
                        resonations: resonations, timestamp: format!("{}-{}-{}-{}", timestamp["year"], timestamp["month"], timestamp["day"], timestamp["hour"]),
                        channels: channels})
}