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

use multihash::Hash;
use std::convert::TryFrom;
use regex::Regex;

//Our modules for holochain actins
use super::definitions::{
    app_definitions,
    function_definitions::{
        FunctionDescriptor,
        FunctionParameters,
        QueryTarget,
        QueryOptions,
        GetLinksLoadElement,
        ExpressionResults,
        QueryType
    }
};

use super::utils;
use super::user;
use super::channel;
use super::group;

//handles conversion of JSON from app call to rust types to call "get_expression" - also converts received entries back to JSON
pub fn handle_get_expression(query_root: Address, query_string: String, query_options: QueryOptions, 
                             context: Address, target_type: QueryTarget, query_type: QueryType) -> ZomeApiResult<JsonString>{
    match target_type {
        QueryTarget::ExpressionPost => {
            let expressions = get_expression::<app_definitions::ExpressionPost>(query_root, query_string, query_options, context, target_type)?;
            Ok(JsonString::from(expressions))
        },
        QueryTarget::User => {
            let expressions = get_expression::<app_definitions::UserName>(query_root, query_string, query_options, context, target_type)?;
            Ok(JsonString::from(expressions))
        }
    }
}

//Function to handle the getting of expression with a given query root and query string
//for example: query_root: Channel: Technology, query_string: 2018<timestamp>:holochain<channel>:dht<channel>:eric<user>
//this would search for all posts in the channel Technology, which where posted in 2018 and also contain the channels Holochain & Dht by the user Eric
pub fn get_expression<T: TryFrom<AppEntryValue>>(query_root: Address, query_string: String, 
        query_options: QueryOptions, context: Address, target_type: QueryTarget) -> ZomeApiResult<ExpressionResults<T>> where T: Clone {
    let mut expression_results = vec![];
    if context.to_string() == hdk::api::DNA_ADDRESS.to_string(){ //global context
        match target_type{
            QueryTarget::User => {
                let queries = query_string.split(":").collect::<Vec<_>>(); //Split query string into individual query parameters
                let re = Regex::new(r"(.*<user>)").unwrap(); //create regex to match for user query string
                let mut has_user_query = None;
                for query in queries{ //iterate over query parameters and check if user query is present
                    if re.is_match(query){
                        has_user_query = Some(query.split("<user>").collect::<Vec<_>>()[0]);
                    };
                };
                match has_user_query{ //match user query
                    Some(query) => { //user query is present - this means we will do a search for the user - disregarding any other query parameters - otherwise the query wont return correct results
                        expression_results = utils::get_links_and_load_type::<String, T>(&query_root, query.to_string())?;
                    },
                    None => { //no user query is present - thus users will be found based on expressions
                        let expression_post_results = utils::get_links_and_load_type::<String, app_definitions::ExpressionPost>(&query_root, query_string.to_string())?;
                        //let mut expression_results = vec![];
                        for expression in expression_post_results{
                            let user = utils::get_links_and_load_type::<String, T>(&expression.address, "owner".to_string())?;
                            expression_results.push(user[0].clone());
                        };
                    }
                };
            },
            QueryTarget::ExpressionPost => {
                expression_results = utils::get_links_and_load_type::<String, T>(&query_root, query_string.to_string())?;
            }
        };
    } else {
        let privacy: app_definitions::Privacy;
        let context_entry = hdk::get_entry(&context)?;
        match context_entry {
            Some(Entry::App(_, entry_value)) => {
                match app_definitions::Channel::try_from(&entry_value){
                    Ok(entry) => {
                        if entry.channel_type != app_definitions::ChannelType::Den{
                            return Err(ZomeApiError::from("When context is a channel it must be of type den".to_string()))
                        };
                        privacy = entry.privacy;
                        if privacy == app_definitions::Privacy::Private {
                            let current_user_hash = user::get_user_username_address_by_agent_address()?;
                            if channel::is_den_owner(context.clone(), current_user_hash.clone())? == false{
                                return Err(ZomeApiError::from("You are attempting to get results from a private channel which you do not own".to_string()))
                            };
                        } else if privacy == app_definitions::Privacy::Shared {
                            //check that user is in pack and thus a shared member of their shared den
                            let den_owner_links = utils::get_links_and_load_type::<String, app_definitions::UserName>(&context, "owner".to_string())?;
                            let den_owner_pack_links = utils::get_links_and_load_type::<String, app_definitions::Group>(&den_owner_links[0].address, "pack".to_string())?;
                            let current_user_hash = user::get_user_username_address_by_agent_address()?;
                            if group::is_group_member(den_owner_pack_links[0].address.clone(), current_user_hash.clone())? == false{
                                return Err(ZomeApiError::from("You are attempting to access a shared channel (den). In order to access expressions from this channel you must be in the owners group".to_string()))
                            };
                        };
                    },
                    Err(_err) => {
                        match app_definitions::Group::try_from(&entry_value){
                            Ok(entry) => {
                                privacy = entry.privacy;
                                if privacy != app_definitions::Privacy::Public {
                                    let current_user_hash = user::get_user_username_address_by_agent_address()?;
                                    if (group::is_group_owner(context.clone(), current_user_hash.clone())? == false) | (group::is_group_member(context.clone(), current_user_hash.clone())? == false){
                                        return Err(ZomeApiError::from("You are attempting to post an expression into a group you are not permitted to interact with".to_string()))
                                    };
                                }; 
                            },
                            Err(_err) => {return Err(ZomeApiError::from("Context address was not a channel, group (den or pack)".to_string()))}
                        };
                    }
                }
            },
            Some(_) => {return Err(ZomeApiError::from("Context address was not an app entry".to_string()))},
            None => return Err(ZomeApiError::from("No context entry at specified address".to_string()))
        };
        //context checking here to see if they are allowed to view posts at given context/privacy
        expression_results = handle_local_query::<T>(context, query_string, privacy, target_type)?;
    }
    Ok(ExpressionResults{expressions: expression_results})
}

//handle local query will just use simple getting of links per query in query string and then cross reference results
pub fn handle_local_query<T: TryFrom<AppEntryValue>>(context: Address, query_string: String, privacy: app_definitions::Privacy,
                          target_type: QueryTarget) -> ZomeApiResult<Vec<GetLinksLoadElement<T>>> where T: Clone {
    let re = Regex::new(r"(<channel>|<user>|<time:y>|<time:m>|<time:d>|<time:h>|<type>)").unwrap();
    let caps = utils::catch_query_string_types(&re, &query_string);
    let value_query_string = &re.replace_all(query_string.as_ref(), "");
    let query_values = value_query_string.split(":").collect::<Vec<_>>();
    if query_values.len() != caps.len(){
        return Err(ZomeApiError::from("Invalid query string".to_string()))
    };
    let mut expression_results = vec![];

    for (i, cap) in caps.iter().enumerate(){
        match cap.to_lowercase().as_ref(){ //Make queries for each value/type
            "<channel>" => {
                let entry = Entry::App("channel".into(), app_definitions::Channel{parent: context.clone(), name: query_values[i].to_string(), 
                                                            privacy: privacy.clone(), channel_type: app_definitions::ChannelType::Tag}.into());
                let address = hdk::entry_address(&entry)?;
                expression_results.push(utils::get_links_and_load_type::<String, T>(&address, "expression".to_string())?);
            },
            "<user>" => {
                let entry = Entry::App("username".into(), app_definitions::UserName{username: query_values[i].to_string()}.into());
                let address = hdk::entry_address(&entry)?;
                expression_results.push(utils::get_links_and_load_type::<String, T>(&address, "expression".to_string())?);
            },
            "<type>" => {
                let entry = Entry::App("channel".into(), app_definitions::Channel{parent: context.clone(), name: query_values[i].to_string(), 
                                                            privacy: privacy.clone(), channel_type: app_definitions::ChannelType::Type}.into());
                let address = hdk::entry_address(&entry)?;
                expression_results.push(utils::get_links_and_load_type::<String, T>(&address, "expression".to_string())?);
            },
            "<time:y>" => {
                let entry = Entry::App("time".into(), app_definitions::Time{parent: context.clone(), time: query_values[i].to_string(), 
                                                        time_type: app_definitions::TimeType::Year}.into());
                let address = hdk::entry_address(&entry)?;
                expression_results.push(utils::get_links_and_load_type::<String, T>(&address, "expression".to_string())?);
            },
            "<time:m>" => {
                let entry = Entry::App("time".into(), app_definitions::Time{parent: context.clone(), time: query_values[i].to_string(), 
                                                        time_type: app_definitions::TimeType::Month}.into());
                let address = hdk::entry_address(&entry)?;
                expression_results.push(utils::get_links_and_load_type::<String, T>(&address, "expression".to_string())?);
            },
            "<time:d>" => {
                let entry = Entry::App("time".into(), app_definitions::Time{parent: context.clone(), time: query_values[i].to_string(), 
                                                        time_type: app_definitions::TimeType::Day}.into());
                let address = hdk::entry_address(&entry)?;
                expression_results.push(utils::get_links_and_load_type::<String, T>(&address, "expression".to_string())?);
            },
            "<time:h>" => {
                let entry = Entry::App("time".into(), app_definitions::Time{parent: context.clone(), time: query_values[i].to_string(), 
                                                        time_type: app_definitions::TimeType::Hour}.into());
                let address = hdk::entry_address(&entry)?;
                expression_results.push(utils::get_links_and_load_type::<String, T>(&address, "expression".to_string())?);
            },
            &_ => {
                return Err(ZomeApiError::from("Invalid query type".to_string()))
            }
        };
    };

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