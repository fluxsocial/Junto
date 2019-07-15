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
        cas::content::Address
    },
    holochain_json_api::{
        json::JsonString
    },
    api::DNA_ADDRESS
};

use std::collections::HashMap;
use std::string::ToString;
use std::convert::TryInto;

//Our modules for holochain actins
use types::{
    app_definition,
    function_definition::{
        EntryAndAddress,
        UserDens,
        ContextAuthResult,
        ContextType
    }
};
use utils;

use super::indexing;

pub fn handle_post_expression(expression: app_definition::ExpressionPost, mut attributes: Vec<String>, context: Vec<Address>) -> ZomeApiResult<Address>{
    hdk::debug("Handling post expression")?;
    //TODO implement expression type assertion
    attributes = attributes.into_iter().map(|attribute| attribute.to_lowercase()).collect::<Vec<String>>();
    if utils::helpers::has_unique_elements(attributes.clone()) == false {return Err(ZomeApiError::from("You have duplicated attributes".to_string()))};
    if attributes.len() > 4 { //TODO implement duplicate tag checking
        return Err(ZomeApiError::from("You are not allowed to specify more than 4 attributes on a given expression".to_string()))
    } else if attributes.len() < 4{
        attributes.sort_by(|a, b| b.cmp(&a)); //Order tags vector in reverse alphabetical order
        for _ in attributes.len()..4{
            attributes.push("*null*".to_string());
        };
    } else {
        attributes.sort_by(|a, b| b.cmp(&a)); //Order attributes vector in reverse alphabetical order
    };
    hdk::debug(format!("Sorted attributes vector: {:?}", attributes))?;
    let mut indexes: Vec<HashMap<&'static str, String>> = attributes.iter().map(|attribute| hashmap!{"type" => "channel".to_string(), "value" => attribute.to_string()}).collect();

    let expression_type = expression.expression_type.clone().to_string();
    let entry = Entry::App("expression_post".into(), expression.into());
    let address = hdk::commit_entry(&entry)?;
    let current_agent_username = utils::helpers::call_and_get_current_user_username()?;
    let timestamps = utils::time::get_entries_timestamp(&address)?;

    hdk::debug("Link user to expression as owner")?;
    hdk::api::link_entries(&address, &current_agent_username.address, "expression_auth".to_string(), "owner".to_string())?;

    indexes.push(hashmap!{"type" => "user".to_string(), "value" => current_agent_username.entry.username.to_lowercase()});
    indexes.push(hashmap!{"type" => "type".to_string(), "value" => expression_type.to_lowercase()});
    indexes.push(hashmap!{"type" => "time:y".to_string(), "value" => timestamps["year"].to_string()}); //add year slice to query params
    indexes.push(hashmap!{"type" => "time:m".to_string(), "value" => timestamps["month"].to_string()}); //add month slice to query params
    indexes.push(hashmap!{"type" => "time:d".to_string(), "value" => timestamps["day"].to_string()}); //add day slice to query params
    indexes.push(hashmap!{"type" => "time:h".to_string(), "value" => timestamps["hour"].to_string()}); //add hour slice to query params

    //query params are saved in following order: tag1<channel>/tag2<channel>/tag3<channel>/tag4<channel>/user<user>/type<type>/time:y<time>/time:m<time>/time:d<time>/time:h<time> 
    //thus tag for each expression link will also be in this order and if there is not four channels present placeholder value will be used
    let mut index_string = indexes.clone().iter().map(|qp| format!("{}<{}>", qp["value"], qp["type"])).collect::<Vec<String>>().join("/");
    index_string = format!("{}{}{}", "/", index_string, "/");
    hdk::debug(format!("Index string: {}", index_string))?;
    indexes = indexes.into_iter().filter(|index| index["value"] != "*null*".to_string()).collect();

    build_indexes(context, &address, &indexes, index_string.as_str())?; //build function hooks that need to be ran on expression based on which contexts are being used
    indexing::create_post_attributes(&indexes, &address)?;
    hdk::debug("Indexes created on post and on contexts")?;

    Ok(address)
}

pub fn build_indexes<'a>(contexts: Vec<Address>, address: &Address, indexes: &'a Vec<HashMap<&'static str, String>>, index_string: &'a str) 
                        -> ZomeApiResult<&'static str> {
    let dna_hash_string = Address::from(DNA_ADDRESS.to_string());
    if utils::helpers::has_unique_elements(contexts.clone()) == false {return Err(ZomeApiError::from("Contexts must be unique".to_string()))};
    let collective_count = contexts.iter().filter(|&c| *c == *&dna_hash_string).count();
    
    //Get junto related contexts
    let current_agent_username = utils::helpers::call_and_get_current_user_username()?;

    let users_pack = hdk::call(hdk::THIS_INSTANCE, "group", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                "user_pack", JsonString::from(json!({"username_address": current_agent_username.address})))?;
    let users_pack: ZomeApiResult<EntryAndAddress<app_definition::Group>> = users_pack.try_into()?;
    let users_pack: EntryAndAddress<app_definition::Group> = users_pack?;

    let member_results = hdk::call(hdk::THIS_INSTANCE, "group", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                "get_user_member_packs", JsonString::from(json!({"username_address": current_agent_username.address})))?;
    let member_results: ZomeApiResult<Vec<EntryAndAddress<app_definition::Group>>> = member_results.try_into()?;
    let member_results: Vec<Address> = member_results?.iter().map(|pack| pack.address.clone()).collect();

    let agents_dens = hdk::call(hdk::THIS_INSTANCE, "collection", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                "user_dens", JsonString::from(json!({"username_address": current_agent_username.address})))?;
    let agents_dens: ZomeApiResult<UserDens> = agents_dens.try_into()?;
    let agents_dens: UserDens = agents_dens?;
    hdk::debug("Got agents dens")?;

    let private_den = agents_dens.private_den.address;
    let shared_den = agents_dens.shared_den.address;
    let public_den = agents_dens.public_den.address;
    let mut local_contexts = vec![&private_den, &shared_den, &public_den, &users_pack.address];
    local_contexts.extend(&member_results);
    
    let current_bit_prefix = hdk::call(hdk::THIS_INSTANCE, "config", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                                "get_current_bit_prefix", JsonString::from(json!({})))?;
    let current_bit_prefix: ZomeApiResult<u32> = current_bit_prefix.try_into()?;
    let current_bit_prefix: u32 = current_bit_prefix?;

    let bit_prefix_value = utils::helpers::hash_prefix(address.clone(), current_bit_prefix);
    hdk::debug(format!("Entry being linked to prefix bucket: {}", bit_prefix_value))?;
    let bit_bucket = hdk::commit_entry(&Entry::App("bucket".into(), app_definition::Bucket{id: bit_prefix_value}.into()))?;

    for context in &contexts{
        if context == &dna_hash_string{
            hdk::debug("Context is a global context")?;
            //Link expression to user
            utils::helpers::link_expression("expression_post", index_string, "forward", &current_agent_username.address, &address)?;
            //Link between random bit bucket and expression so random post querying can happen on this post
            utils::helpers::link_expression("bucket_expression_post", index_string, "forward", &bit_bucket, &address)?;
            //Link expression to private den
            indexing::create_post_index(indexes, &private_den, &address, index_string, "collection_expression_post", ContextType::Collection)?;
            //Link expression to shared den
            indexing::create_post_index(indexes, &shared_den, &address, index_string, "collection_expression_post", ContextType::Collection)?;
            //Link expression to public den
            indexing::create_post_index(indexes, &public_den, &address, index_string, "collection_expression_post", ContextType::Collection)?;
            //Link expression to user pack
            indexing::create_post_index(indexes, &users_pack.address, &address, index_string, "group_expression_post", ContextType::Group)?;
            for pack in &member_results{ //Link expression to each pack user is a member of
                indexing::create_post_index(indexes, &pack, &address, index_string, "group_expression_post", ContextType::Group)?;
            }; 
        } else {
            if local_contexts.contains(&context) == true && collective_count == 1 {return Err(ZomeApiError::from("You have submitted a default Junto context and global context, you can only submit one or the other".to_string()))}
            let privacy_auth_result = indexing::run_context_auth(context, &current_agent_username.address)?
                .ok_or(ZomeApiError::from("Context address was not a collection, group or dna address (global context)".to_string()))?;
            match privacy_auth_result{
                ContextAuthResult::Collection(_context_entry) => {
                    indexing::create_post_index(indexes, &context, &address, index_string, "collection_expression_post", ContextType::Collection)?;
                },
                ContextAuthResult::Group(_context_entry) => {
                    indexing::create_post_index(indexes, &context, &address, index_string, "group_expression_post", ContextType::Group)?;
                }
            };
        }
    };
    Ok("indexes made")
}

pub fn post_comment_expression(expression: app_definition::ExpressionPost, parent_expression: Address) -> ZomeApiResult<Address> {
    hdk::debug("Making a comment")?;
    let _parent_entry = hdk::utils::get_as_type::<app_definition::ExpressionPost>(parent_expression.clone())
        .map_err(|_err| ZomeApiError::from(String::from("Parent expression was not of type ExpressionPost")))?;
    let expression_type = expression.expression_type.clone().to_string();
    let entry = Entry::App("expression_post".into(), expression.into());
    let address = hdk::commit_entry(&entry)?;
    let current_agent_username = utils::helpers::call_and_get_current_user_username()?;
    let timestamps = utils::time::get_entries_timestamp(&address)?;
    
    let indexes = vec![hashmap!{"type" => "channel".to_string(), "value" => "*null*".to_string()},
                            hashmap!{"type" => "channel".to_string(), "value" => "*null*".to_string()},
                            hashmap!{"type" => "channel".to_string(), "value" => "*null*".to_string()},
                            hashmap!{"type" => "channel".to_string(), "value" => "*null*".to_string()},
                            hashmap!{"type" => "user".to_string(), "value" => current_agent_username.entry.username.to_lowercase()},
                            hashmap!{"type" => "type".to_string(), "value" => expression_type.to_lowercase()},
                            hashmap!{"type" => "time:y".to_string(), "value" => timestamps["year"].to_string()},
                            hashmap!{"type" => "time:m".to_string(), "value" => timestamps["month"].to_string()},
                            hashmap!{"type" => "time:d".to_string(), "value" => timestamps["day"].to_string()},
                            hashmap!{"type" => "time:h".to_string(), "value" => timestamps["hour"].to_string()}];

    let mut index_string = indexes.clone().iter().map(|qp| format!("{}<{}>", qp["value"], qp["type"])).collect::<Vec<String>>().join("/");
    index_string = format!("{}{}{}", "/", index_string, "/");
    hdk::debug(format!("Index string: {}", index_string))?;
    indexing::create_post_attributes(&indexes, &address)?;
    hdk::api::link_entries(&address, &current_agent_username.address, "expression_auth".to_string(), "owner".to_string())?;
    utils::helpers::link_expression("sub_expression", index_string.as_str(), "forward", &current_agent_username.address, &address)?;
    utils::helpers::link_expression("expression_sub_expression", index_string.as_str(), "forward", &parent_expression, &address)?;
    utils::helpers::link_expression("parent_expression", "", "forward", &address, &parent_expression)?;
    Ok(address)
}

//Function to handle the resonation of an expression post - will put the post into packs which the post should be resonated into
pub fn handle_resonation(expression: Address) -> ZomeApiResult<String>{
    let expression_entry = hdk::utils::get_as_type::<app_definition::ExpressionPost>(expression.clone())
        .map_err(|_err| ZomeApiError::from(String::from("Expression was not of type ExpressionPost")))?;

    let current_agent_username = utils::helpers::call_and_get_current_user_username()?;

    let users_pack = hdk::call(hdk::THIS_INSTANCE, "group", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                "user_pack", JsonString::from(json!({"username_address": current_agent_username.address.clone()})))?;
    let users_pack: ZomeApiResult<EntryAndAddress<app_definition::Group>> = users_pack.try_into()?;
    let users_pack: EntryAndAddress<app_definition::Group> = users_pack?;

    let mut channels = utils::helpers::get_links_and_load_type::<app_definition::Attribute>(&expression, LinkMatch::Exactly("channels"), LinkMatch::Any)?
                        .iter().map(|channel| channel.entry.value.clone()).collect::<Vec<_>>();
    let owner = utils::helpers::get_links_and_load_type::<app_definition::UserName>(&expression, LinkMatch::Exactly("expression_auth"), LinkMatch::Exactly("owner"))?;
    let timestamps = utils::time::get_entries_timestamp(&expression)?;
    channels.sort_by(|a, b| b.cmp(&a));
    if channels.len() < 4{
        for _ in channels.len()..4{
            channels.push("*null*".to_string());
        };
    };

    let mut indexes: Vec<HashMap<&'static str, String>> = channels.iter().map(|channel| hashmap!{"type" => "channel".to_string(), "value" => channel.clone()}).collect();
    indexes.push(hashmap!{"type" => "user".to_string(), "value" => owner[0].entry.username.to_lowercase()});
    indexes.push(hashmap!{"type" => "type".to_string(), "value" => expression_entry.expression_type.clone().to_string().to_lowercase()});
    indexes.push(hashmap!{"type" => "time:y".to_string(), "value" => timestamps["year"].to_string()}); //add year slice to query params
    indexes.push(hashmap!{"type" => "time:m".to_string(), "value" => timestamps["month"].to_string()}); //add month slice to query params
    indexes.push(hashmap!{"type" => "time:d".to_string(), "value" => timestamps["day"].to_string()}); //add day slice to query params
    indexes.push(hashmap!{"type" => "time:h".to_string(), "value" => timestamps["hour"].to_string()}); //add hour slice to query params
    let mut index_string = indexes.clone().iter().map(|qp| format!("{}<{}>", qp["value"], qp["type"])).collect::<Vec<String>>().join("/");
    index_string = format!("{}{}{}", "/", index_string, "/");
    indexes = indexes.into_iter().filter(|index| index["value"] != "*null*".to_string()).collect();

    //add link on expression to user who made the resonation?
    indexing::create_post_index(&indexes, &users_pack.address, &expression, index_string.as_str(), "resonation", ContextType::Group)?;
    hdk::debug("Created post index")?;
    utils::helpers::link_expression("resonator", "", "forward", &expression, &current_agent_username.address)?;
    Ok("Resonation Generated".to_string())
}