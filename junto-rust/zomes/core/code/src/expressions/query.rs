use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        cas::content::Address,
        entry::Entry, 
        json::JsonString,
        entry::AppEntryValue
    }
};

use std::convert::TryFrom;
use regex::Regex;

//Our modules for holochain actins
use super::definitions::{
    app_definitions,
    function_definitions::{
        QueryTarget,
        QueryOptions,
        EntryAndAddress,
        ExpressionResults,
        QueryType
    }
};

use super::utils;
use super::user;
use super::channel;
use super::group;

//handles conversion of JSON from app call to rust types to call "get_expression" - also converts received entries back to JSON
pub fn handle_get_expression(query_root: Address, query_points: Vec<String>, context: Address,  
    query_options: QueryOptions, target_type: QueryTarget, _query_type: QueryType) -> ZomeApiResult<JsonString>{
    match target_type {
        QueryTarget::ExpressionPost => {
            let expressions = get_expression::<app_definitions::ExpressionPost>(query_root, query_points, context, query_options, target_type, _query_type)?;
            Ok(JsonString::from(expressions))
        },
        QueryTarget::User => {
            let expressions = get_expression::<app_definitions::UserName>(query_root, query_points, context, query_options, target_type, _query_type)?;
            Ok(JsonString::from(expressions))
        }
    }
}

//Function to handle the getting of expression with a given query root and query points
//for example: query_root: Channel: Technology & query_points: [2018<timestamp>, holochain<channel>, dht<channel>, eric<user>]
//this would search for all posts in the channel Technology, which where posted in 2018 and also contain the channels Holochain & Dht by the user Eric
pub fn get_expression<T: TryFrom<AppEntryValue>>(query_root: Address, mut query_points: Vec<String>, context: Address,  
    _query_options: QueryOptions, target_type: QueryTarget, _query_type: QueryType) -> ZomeApiResult<ExpressionResults<T>> where T: Clone {
    let mut expression_results = vec![];

    let re = Regex::new(r"(.*<*>)$").unwrap(); //regex to check that each query point is of syntax: value<type>
    let mut query_points_split: Vec<Vec<&str>> = vec![];
    for query_point in &query_points{
        let split_check: Vec<&str> = query_point.split("<").collect(); //split value and type
        if re.is_match(query_point) == false { //check that query point has type and value and of correct format
            return Err(ZomeApiError::from(format!("Invalid format for query point: {}", query_point)))
        };
        split_check[0].to_lowercase(); //put query points to lowercase to match how they are saved into DHT
        split_check[1].to_lowercase();
        query_points_split.push(split_check.clone());
    };
    
    query_points_split.sort_by(|a, b| b[0].cmp(&a[0])); //sort by value of query parameter(s)
    query_points = query_points_split.iter().map(|query_point| query_point.join("<")).collect(); //collect query points back into original string(s)

    if context.to_string() == hdk::api::DNA_ADDRESS.to_string(){ //global context
        let query_string = query_points.join(":");
        match target_type{
            QueryTarget::User => {
                let re = Regex::new(r"(.*<user>)").unwrap(); //create regex to match for user query string
                let mut has_user_query = None;
                for query_point in &query_points{ //iterate over query parameters and check if user query is present
                    if re.is_match(query_point){
                        has_user_query = Some(query_point.split("<user>").collect::<Vec<_>>()[0]);
                    };
                };
                match has_user_query{ //match user query
                    Some(query) => { //user query is present - this means we will do a search for the user - disregarding any other query parameters - otherwise the query wont return correct results
                        expression_results = utils::get_links_and_load_type::<T>(&query_root, Some("username".to_string()), Some(query.to_string()))?;
                    },
                    None => { //no user query is present - thus users will be found based on expressions
                        let expression_post_results = utils::get_links_and_load_type::<app_definitions::ExpressionPost>(&query_root, Some("expression_post".to_string()), Some(query_string.to_string()))?;
                        for expression in expression_post_results{
                            let user = utils::get_links_and_load_type::<T>(&expression.address, Some("auth".to_string()), Some("owner".to_string()))?;
                            expression_results.push(user[0].clone());
                        };
                    }
                };
            },
            QueryTarget::ExpressionPost => {
                expression_results = utils::get_links_and_load_type::<T>(&query_root, Some("expression_post".to_string()), Some(query_string.to_string()))?;
            }
        };
    } else { //context is a local context - first do local context auth checking. TODO: abstract out to own function
        let privacy: app_definitions::Privacy;
        let current_user_hash = user::get_user_username_by_agent_address()?.address;
        match hdk::utils::get_as_type::<app_definitions::Channel>(context.clone()){
            Ok(context_entry) => {
                if context_entry.channel_type != app_definitions::ChannelType::Den{
                    return Err(ZomeApiError::from("When context is a channel it must be of type den".to_string()))
                };
                privacy = context_entry.privacy;
                if privacy == app_definitions::Privacy::Private {
                    if channel::is_den_owner(context.clone(), current_user_hash.clone())? == false{
                        return Err(ZomeApiError::from("You are attempting to get results from a private channel which you do not own".to_string()))
                    };
                } else if privacy == app_definitions::Privacy::Shared {
                    //check that user is in pack and thus a shared member of their shared den
                    let den_owner_links = utils::get_links_and_load_type::<app_definitions::UserName>(&context, Some("auth".to_string()), Some("owner".to_string()))?;
                    let den_owner_pack_links = utils::get_links_and_load_type::<app_definitions::Group>(&den_owner_links[0].address, Some("group".to_string()), Some("pack".to_string()))?;
                    if group::is_group_member(den_owner_pack_links[0].address.clone(), current_user_hash.clone())? == false{
                        return Err(ZomeApiError::from("You are attempting to access a shared channel (den). In order to access expressions from this channel you must be in the owners group".to_string()))
                    };
                };
            },
            Err(_err) => {
                let context_entry = hdk::utils::get_as_type::<app_definitions::Group>(context.clone()).map_err(|_err| ZomeApiError::from("Context address was not a channel, group or dna address (global context)".to_string()))?;
                privacy = context_entry.privacy;
                if privacy != app_definitions::Privacy::Public {
                    if (group::is_group_owner(context.clone(), current_user_hash.clone())? == false) | (group::is_group_member(context.clone(), current_user_hash.clone())? == false){
                        return Err(ZomeApiError::from("You are attempting to post an expression into a group you are not permitted to interact with".to_string()))
                    };
                }; 
            }
        };
        expression_results = handle_local_query::<T>(context, query_points, privacy, target_type)?;
    };
    Ok(ExpressionResults{expressions: expression_results})
}

//handle local query will just use simple getting of links per query in query string and then cross reference results
pub fn handle_local_query<T: TryFrom<AppEntryValue>>(context: Address, query_points: Vec<String>, privacy: app_definitions::Privacy,
                          _target_type: QueryTarget) -> ZomeApiResult<Vec<EntryAndAddress<T>>> where T: Clone {
    let mut expression_results = vec![];

    for query_point in query_points{
        let query_split: Vec<&str> = query_point.split("<").collect();
        match query_split[1][0..query_split.len()-1].as_ref(){
            "channel" => {
                let entry = Entry::App("channel".into(), app_definitions::Channel{parent: context.clone(), name: query_split[0].to_string(), 
                                                            privacy: privacy.clone(), channel_type: app_definitions::ChannelType::Tag}.into());
                let address = hdk::entry_address(&entry)?;
                expression_results.push(utils::get_links_and_load_type::<T>(&address, Some("expression_post".to_string()), Some("expression".to_string()))?);
            },
            "user" => {
                let entry = Entry::App("username".into(), app_definitions::UserName{username: query_split[0].to_string()}.into());
                let address = hdk::entry_address(&entry)?;
                expression_results.push(utils::get_links_and_load_type::<T>(&address, Some("expression_post".to_string()), Some("expression".to_string()))?);
            },
            "type" => {
                let entry = Entry::App("channel".into(), app_definitions::Channel{parent: context.clone(), name: query_split[0].to_string(), 
                                                            privacy: privacy.clone(), channel_type: app_definitions::ChannelType::Type}.into());
                let address = hdk::entry_address(&entry)?;
                expression_results.push(utils::get_links_and_load_type::<T>(&address, Some("expression_post".to_string()), Some("expression".to_string()))?);
            },
            "time:y" => {
                let entry = Entry::App("time".into(), app_definitions::Time{parent: context.clone(), time: query_split[0].to_string(), 
                                                        time_type: app_definitions::TimeType::Year}.into());
                let address = hdk::entry_address(&entry)?;
                expression_results.push(utils::get_links_and_load_type::<T>(&address, Some("expression_post".to_string()), Some("expression".to_string()))?);
            },
            "time:m" => {
                let entry = Entry::App("time".into(), app_definitions::Time{parent: context.clone(), time: query_split[0].to_string(), 
                                                        time_type: app_definitions::TimeType::Month}.into());
                let address = hdk::entry_address(&entry)?;
                expression_results.push(utils::get_links_and_load_type::<T>(&address, Some("expression_post".to_string()), Some("expression".to_string()))?);
            },
            "time:d" => {
                let entry = Entry::App("time".into(), app_definitions::Time{parent: context.clone(), time: query_split[0].to_string(), 
                                                        time_type: app_definitions::TimeType::Day}.into());
                let address = hdk::entry_address(&entry)?;
                expression_results.push(utils::get_links_and_load_type::<T>(&address, Some("expression_post".to_string()), Some("expression".to_string()))?);
            },
            "time:h" => {
                let entry = Entry::App("time".into(), app_definitions::Time{parent: context.clone(), time: query_split[0].to_string(), 
                                                        time_type: app_definitions::TimeType::Hour}.into());
                let address = hdk::entry_address(&entry)?;
                expression_results.push(utils::get_links_and_load_type::<T>(&address, Some("expression_post".to_string()), Some("expression".to_string()))?);
            },
            &_ => {
                return Err(ZomeApiError::from("Invalid query type".to_string()))
            }        
        }
    }

    if expression_results.len() == 0 {
        Ok(vec![])
    } else {
        //add ability for "or" querying
        let mut out = vec![]; //Most likely more effecient method than this - this will do for now
        let start_comparison = &expression_results[0];
        for expression in start_comparison{ //look over each expressions in first query parameter
            out.push(expression.clone()); //push current expression to out vector
            for expression_set in expression_results[1..].into_iter(){ //loop over following expression querys
                if expression_set.contains(&expression) == false{ //check if expression exists inside next expression query
                    out.pop(); //expression does not exist - in the case of an "and" query this should be removed from out vector
                };
            };  
        };
        Ok(out)
    }
}