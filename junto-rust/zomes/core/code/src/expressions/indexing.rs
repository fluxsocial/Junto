//Module to handle all channel related operations
use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        entry::Entry, 
        cas::content::Address,
        hash::HashString
    }
};

use std::collections::HashMap;
use itertools::Itertools;

use super::definitions::app_definitions;
use super::group;
use super::user;
use super::channel;

pub fn create_query_points(query_points: Vec<HashMap<String, String>>, context: &Address, privacy: &app_definitions::Privacy, 
                            query_type: &String, expression: &Address) -> ZomeApiResult<String>{
    let mut addressed_params: Vec<HashMap<String, String>> = query_points.to_vec();
    let mut is_global = true;
    if context != &HashString::from(hdk::api::DNA_ADDRESS.to_string()){
        match hdk::utils::get_as_type::<app_definitions::Channel>(context.clone()) {
            Ok(context_entry) => {
                if context_entry.channel_type != app_definitions::ChannelType::Den{
                    return Err(ZomeApiError::from("When context is a channel it must be of type den - you cannot post into normal public channels".to_string()))
                }
                let current_user_hash = user::get_user_username_address_by_agent_address()?;
                //check that current user making post is owner of den they are trying to post into
                if channel::is_den_owner(context.clone(), current_user_hash.clone())? == false{
                    return Err(ZomeApiError::from("You are attempting to get results from a private channel which you do not own".to_string()))
                }
            },
            Err(_err) => {
                let context_entry = hdk::utils::get_as_type::<app_definitions::Group>(context.clone()).map_err(|_err| ZomeApiError::from("Context address was not a channel, group or dna address (global context)".to_string()))?;
                if context_entry.privacy != app_definitions::Privacy::Public {
                    let current_user_hash = user::get_user_username_address_by_agent_address()?;
                    if (group::is_group_owner(context.clone(), current_user_hash.clone())? == false) | (group::is_group_member(context.clone(), current_user_hash.clone())? == false){
                        return Err(ZomeApiError::from("You are attempting to post an expression into a group you are not permitted to interact with".to_string()))
                    }
                }
            }
        };
        //make link on group channel for all posts
        hdk::api::link_entries(&context, expression, "expression_post", "expression")?;
        is_global = false;
    };

    //Create "traditional" base links between index items and expression for given context - these are links that should be present for every expression post - no matter if it is public or not
    for (i, query_param) in query_points.iter().enumerate(){
        match query_param["type"].as_ref(){
            "channel" => {
                let address = 
                    if is_global == true {
                        let channel_anchor = hdk::commit_entry(&Entry::App("anchor".into(), app_definitions::Anchor{anchor_type: "channel".to_string()}.into()))?;
                        let entry = Entry::App("channel".into(), app_definitions::Channel{name: query_param["value"].to_string(), 
                                        parent: channel_anchor.clone(), privacy: privacy.clone(), channel_type: app_definitions::ChannelType::Tag}.into()).into();
                        let address = hdk::commit_entry(&entry)?;
                        hdk::api::link_entries(&channel_anchor, &address, "channel", &query_param["value"])?;
                        address
                    } else {
                        let entry = Entry::App("channel".into(), app_definitions::Channel{name: query_param["value"].to_string(), 
                                        parent: context.clone(), privacy: privacy.clone(), channel_type: app_definitions::ChannelType::Tag}.into()).into();
                        let address = hdk::commit_entry(&entry)?;
                        hdk::api::link_entries(&context, &address, "channel", &query_param["value"])?;
                        address
                    };

                hdk::api::link_entries(&address, expression, "expression_post", "expression")?;
                hdk::api::link_entries(&address, expression, "expression_post", &query_param["value"])?;
                hdk::api::link_entries(&expression, &address, "expression_channels", &query_param["value"])?;
                addressed_params[i].insert("address".to_string(), address.to_string()); 
            },

            "type" => {
                let address =
                    if is_global == true {
                        let type_anchor = hdk::commit_entry(&Entry::App("anchor".into(), app_definitions::Anchor{anchor_type: "expression_type".to_string()}.into()))?;
                        let entry = Entry::App("channel".into(), app_definitions::Channel{name: query_param["value"].to_string(), 
                                                            parent: type_anchor.clone(), privacy: privacy.clone(), channel_type: app_definitions::ChannelType::Type}.into()).into();
                        let address = hdk::commit_entry(&entry)?;
                        hdk::api::link_entries(&type_anchor, &address, "expression_type", &query_param["value"])?;
                        address

                    } else {
                        let entry = Entry::App("channel".into(), app_definitions::Channel{name: query_param["value"].to_string(), 
                                        parent: context.clone(), privacy: privacy.clone(), channel_type: app_definitions::ChannelType::Type}.into()).into();
                        let address = hdk::commit_entry(&entry)?;
                        hdk::api::link_entries(&context, &address, "expression_type", &query_param["value"])?;
                        address
                    };

                hdk::api::link_entries(&address, expression, "expression_post", "expression")?;
                hdk::api::link_entries(&address, expression, "expression_post", &query_param["value"])?;
                hdk::api::link_entries(expression, &address, "expression_type", &query_param["value"])?;
                addressed_params[i].insert("address".to_string(), address.to_string());
            },

            "time:y" => {
                let address = 
                    if is_global == true {
                        let time_anchor = hdk::commit_entry(&Entry::App("anchor".into(), app_definitions::Anchor{anchor_type: "time".to_string()}.into()))?;   
                        let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                                parent: time_anchor.clone(), time_type: app_definitions::TimeType::Year}.into()).into();
                        let address = hdk::commit_entry(&entry)?;
                        hdk::api::link_entries(&time_anchor, &address, "time", &query_param["value"])?;
                        address

                    } else {
                        let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                                parent: context.clone(), time_type: app_definitions::TimeType::Year}.into()).into();
                        let address = hdk::commit_entry(&entry)?;
                        hdk::api::link_entries(&context, &address, "time", &query_param["value"])?;
                        address
                    };

                hdk::api::link_entries(&address, expression, "expression_post", "expression")?;
                hdk::api::link_entries(&address, expression, "expression_post", &query_param["value"])?;
                hdk::api::link_entries(expression, &address, "time", "year")?;
                addressed_params[i].insert("address".to_string(), address.to_string());
            },

            "time:m" => {
                let address =
                    if is_global == true {
                        let time_anchor = hdk::commit_entry(&Entry::App("anchor".into(), app_definitions::Anchor{anchor_type: "time".to_string()}.into()))?;   
                        let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                                parent: time_anchor.clone(), time_type: app_definitions::TimeType::Month}.into()).into();
                        let address = hdk::commit_entry(&entry)?;
                        hdk::api::link_entries(&time_anchor, &address, "time", &query_param["value"])?;
                        address

                    } else {
                        let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                                parent: context.clone(), time_type: app_definitions::TimeType::Month}.into()).into();
                        let address = hdk::commit_entry(&entry)?;
                        hdk::api::link_entries(&context, &address, "time", &query_param["value"])?;
                        address
                    };

                hdk::api::link_entries(&address, expression, "expression_post", "expression")?;
                hdk::api::link_entries(&address, expression, "expression_post", &query_param["value"])?;
                hdk::api::link_entries(&expression, &address, "time", "month")?;
                addressed_params[i].insert("address".to_string(), address.to_string());
            },

            "time:d" => {
                let address =
                    if is_global == true {
                        let time_anchor = hdk::commit_entry(&Entry::App("anchor".into(), app_definitions::Anchor{anchor_type: "time".to_string()}.into()))?;   
                        let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                                parent: time_anchor.clone(), time_type: app_definitions::TimeType::Day}.into()).into();
                        let address = hdk::commit_entry(&entry)?;
                        hdk::api::link_entries(&time_anchor, &address, "time", &query_param["value"])?;
                        address

                    } else {
                        let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                                parent: context.clone(), time_type: app_definitions::TimeType::Day}.into()).into();
                        let address = hdk::commit_entry(&entry)?;
                        hdk::api::link_entries(&context, &address, "time", &query_param["value"])?;
                        address
                    };

                hdk::api::link_entries(&address, expression, "expression_post", "expression")?;
                hdk::api::link_entries(&address, expression, "expression_post", &query_param["value"])?;
                hdk::api::link_entries(&expression, &address, "time", "day")?;
                addressed_params[i].insert("address".to_string(), address.to_string());
            },

            "time:h" => {
                let address =
                    if is_global == true {
                        let time_anchor = hdk::commit_entry(&Entry::App("anchor".into(), app_definitions::Anchor{anchor_type: "time".to_string()}.into()))?;   
                        let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                                parent: time_anchor.clone(), time_type: app_definitions::TimeType::Hour}.into()).into();
                        let address = hdk::commit_entry(&entry)?;
                        hdk::api::link_entries(&time_anchor, &address, "time", &query_param["value"])?;
                        address

                    } else {
                        let entry = Entry::App("time".into(), app_definitions::Time{time: query_param["value"].to_string(), 
                                                parent: context.clone(), time_type: app_definitions::TimeType::Hour}.into()).into();
                        let address = hdk::commit_entry(&entry)?;
                        hdk::api::link_entries(&context, &address, "time", &query_param["value"])?;
                        address
                    };

                hdk::api::link_entries(&address, expression, "expression_post", "expression")?;
                hdk::api::link_entries(&address, expression, "expression_post", &query_param["value"])?;
                hdk::api::link_entries(&expression, &address, "time", "hour")?;
                addressed_params[i].insert("address".to_string(), address.to_string());
            },

            "user" => {
                let entry = Entry::App("username".into(), app_definitions::UserName{username: query_param["value"].to_string()}.into()).into();
                let address = hdk::entry_address(&entry)?;
                let dht_entry = hdk::get_entry(&address)?;

                match dht_entry{
                    Some(_value) => {
                        hdk::api::link_entries(&address, &expression, "expression_post", "expression")?;
                        hdk::api::link_entries(&expression, &address, "auth", "owner")?;
                        addressed_params[i].insert("address".to_string(), address.to_string());
                    },
                    None => {
                        return Err(ZomeApiError::from("No user with given username string".to_string()))
                    }
                };
            },
            _ => {
                return Err(ZomeApiError::from("That contextual link type does not exist".to_string()))
            }
        };
    };

    if query_type == "contextual" {
        create_contextual_links(&addressed_params, expression)?;
    };
    Ok("ok".to_string())
}

//Only expression posts are being indexed using contextual_links
pub fn create_contextual_links(query_points: &Vec<HashMap<String, String>>, expression: &Address) -> ZomeApiResult<String>{
    let mut link_combinations = vec![]; //Vector for link combinations on expression

    for (i, _) in query_points.iter().enumerate(){
        let combinations = query_points.iter().combinations(i);
        for c in combinations.into_iter(){
            link_combinations.push(c);
        };
    };
    link_combinations.push(query_points.iter().collect());
    link_combinations = link_combinations[1..link_combinations.len()].to_vec();

    for link in link_combinations{ //Create link combinations for expression indexing
        let start = link[0];
        let link_strings: Vec<String> = link.iter().map(|link_value| format!("{}<{}>", link_value["value"], link_value["type"],) ).collect();
        let link_string = link_strings.join(":");
        hdk::api::link_entries(&HashString::from(start["address"].clone()), expression, "expression_post", &link_string)?;
    };
    Ok("ok".to_string())
}