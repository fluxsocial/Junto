use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        cas::content::Address,
        entry::Entry,
        link::LinkMatch
    },
    api::DNA_ADDRESS
};

use std::collections::HashMap;
use std::string::ToString;

//Our modules for holochain actins
use super::definitions::{
    app_definitions,
    function_definitions::{
        FunctionDescriptor,
        FunctionParameters
    }
};

use super::utils;
use super::user;
use super::indexing;
use super::random;

pub fn handle_post_expression(expression: app_definitions::ExpressionPost, mut attributes: Vec<String>, context: Vec<Address>) -> ZomeApiResult<Address>{
    hdk::debug("Handling post expression")?;
    //TODO implement expression type assertion
    attributes = attributes.into_iter().map(|attribute| attribute.to_lowercase()).collect::<Vec<String>>();
    if utils::has_unique_elements(attributes.clone()) == false {return Err(ZomeApiError::from("You have duplicated attributes".to_string()))};
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
    let mut indexes: Vec<HashMap<String, String>> = attributes.iter().map(|attribute| hashmap!{"type".to_string() => "channel".to_string(), "value".to_string() => attribute.to_string().to_lowercase()}).collect();

    let expression_type = expression.expression_type.clone();
    let entry = Entry::App("expression_post".into(), expression.into());
    let address = hdk::commit_entry(&entry)?;
    let username_entry_address = user::get_user_username_by_agent_address()?;
    let timestamps = utils::get_entries_timestamp(&address)?;

    hdk::debug("Link user to expression as owner")?;
    hdk::api::link_entries(&address, &username_entry_address.address, "auth".to_string(), "owner".to_string())?;

    indexes.push(hashmap!{"type".to_string() => "user".to_string(), "value".to_string() => username_entry_address.entry.username.to_string().to_lowercase()});
    indexes.push(hashmap!{"type".to_string() => "type".to_string(), "value".to_string() => expression_type.to_string().to_lowercase()});
    indexes.push(hashmap!{"type".to_string() => "time:y".to_string(), "value".to_string() => timestamps["year"].clone()}); //add year slice to query params
    indexes.push(hashmap!{"type".to_string() => "time:m".to_string(), "value".to_string() => timestamps["month"].clone()}); //add month slice to query params
    indexes.push(hashmap!{"type".to_string() => "time:d".to_string(), "value".to_string() => timestamps["day"].clone()}); //add day slice to query params
    indexes.push(hashmap!{"type".to_string() => "time:h".to_string(), "value".to_string() => timestamps["hour"].clone()}); //add hour slice to query params

    //query params are saved in following order: tag1<channel>/tag2<channel>/tag3<channel>/tag4<channel>/user<user>/type<type>/time:y<time>/time:m<time>/time:d<time>/time:h<time> 
    //thus tag for each expression link will also be in this order and if there is not four channels present placeholder value will be used
    let mut index_string = indexes.clone().iter().map(|qp| qp["value"].clone() + "<" + &qp["type"].clone() + ">" ).collect::<Vec<String>>().join("/");
    index_string = format!("{}{}{}", "/", index_string, "/");
    hdk::debug(format!("Index string: {}", index_string))?;
    indexes = indexes.into_iter().filter(|index| index["value"] != "*null*".to_string()).collect();

    let hook_definitions = build_hooks(context, &address, &indexes, index_string)?; //build function hooks that need to be ran on expression based on which contexts are being used
    indexing::create_post_attributes(&indexes, &address)?;
    hdk::debug("Hook defnitions generated")?;

    utils::handle_hooks(hook_definitions)?;
    Ok(address)
}

// pub fn comment_expression(expression: app_definitions::ExpressionPost) -> ZomeApiResult<Address> {

// }

pub fn build_hooks(contexts: Vec<Address>, address: &Address, indexes: &Vec<HashMap<String, String>>, index_string: String) -> ZomeApiResult<Vec<FunctionDescriptor>> {
    let mut hook_definitions = vec![];
    let dna_hash_string = Address::from(DNA_ADDRESS.to_string());
    if utils::has_unique_elements(contexts.clone()) == false {return Err(ZomeApiError::from("Contexts must be unique".to_string()))};
    let collective_count = contexts.iter().filter(|&c| *c == *&dna_hash_string).count();
    
    //Get junto related contexts
    let user_name_address = user::get_user_username_by_agent_address()?.address; 
    let user_pack = user::get_user_pack(user_name_address.clone())?.address;
    let member_results: Vec<Address> = user::get_user_member_packs(user_name_address.clone())?.iter().map(|pack| pack.address.clone()).collect();
    let den_result = user::get_user_dens(user_name_address.clone())?;
    let private_den = den_result.private_den.address;
    let shared_den = den_result.shared_den.address;
    let public_den = den_result.public_den.address;
    let mut local_contexts = vec![&private_den, &shared_den, &public_den, &user_pack];
    local_contexts.extend(&member_results);
    
    let current_bit_prefix = random::get_current_bit_prefix()?; //get current bit buckets
    let bit_prefix_value = random::hash_prefix(address.clone(), current_bit_prefix);
    hdk::debug(format!("Entry being linked to prefix bucket: {}", bit_prefix_value))?;
    let bit_bucket = hdk::commit_entry(&Entry::App("bucket".into(), app_definitions::Bucket{id: bit_prefix_value}.into()))?;

    for context in &contexts{
        if context == &dna_hash_string{
            hdk::debug("Context is a global context")?;
            //Link expression to user
            hook_definitions.push(FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "expression_post".to_string(), tag: index_string.clone(), direction: "forward".to_string(), parent_expression: user_name_address.clone(), child_expression: address.clone()}});
            //Link between random bit bucket and expression so random post querying can happen on this post
            hook_definitions.push(FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "expression_post".to_string(), tag: index_string.clone(), direction: "forward".to_string(), parent_expression: bit_bucket.clone(), child_expression: address.clone()}});
            //Link expression to private den
            hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{indexes: indexes.clone(), context: private_den.clone(),  expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});
            //Link expression to shared den
            hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{indexes: indexes.clone(), context: shared_den.clone(), expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});
            //Link expression to public den
            hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{indexes: indexes.clone(), context: public_den.clone(), expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});
            //Link expression to user pack
            hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{indexes: indexes.clone(), context: user_pack.clone(), expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});
            for pack in &member_results{ //Link expression to each pack user is a member of
                hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{indexes: indexes.clone(), context: pack.clone(), expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});
            }; 
        } else {
            if local_contexts.contains(&context) == true && collective_count == 1 {return Err(ZomeApiError::from("You have submitted a default Junto context and global context, you can only submit one or the other".to_string()))}
            let _privacy_auth_result = utils::run_context_auth(context, &user_name_address)?
                .ok_or(ZomeApiError::from("Context address was not a collection, group or dna address (global context)".to_string()))?;
            hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{indexes: indexes.clone(), context: context.clone(), expression: address.clone(), index_string: index_string.clone(), link_type: "expression_post".to_string()}});
        }
    };
    Ok(hook_definitions)
}

//Function to handle the resonation of an expression post - will put the post into packs which the post should be resonated into
pub fn handle_resonation(expression: Address) -> ZomeApiResult<String>{
    match hdk::api::get_entry(&expression)?{
        None => {return Err(ZomeApiError::from("No expression at given address".to_string()))},
        _ => {}
    };
    let user_name_address = user::get_user_username_by_agent_address()?.address;
    let user_pack = user::get_user_pack(user_name_address.clone())?.address;

    let channels = utils::get_links_and_load_type::<app_definitions::Attribute>(&expression, LinkMatch::Exactly("channels"), LinkMatch::Any)?;
    let times = utils::get_links_and_load_type::<app_definitions::Attribute>(&expression, LinkMatch::Exactly("created_at"), LinkMatch::Any)?;
    let exp_type = utils::get_links_and_load_type::<app_definitions::Attribute>(&expression, LinkMatch::Exactly("expression_type"), LinkMatch::Any)?;
    
    let mut index: Vec<HashMap<String, String>> = channels.iter().map(|channel| hashmap!{"value".to_string() => channel.entry.value.clone(), "type".to_string() => "channel".to_string()}).collect();
    for time in times{
        match time.entry.attribute_type{
            app_definitions::AttributeType::Year => {index.push(hashmap!{"value".to_string() => time.entry.value.clone(), "type".to_string() => "time:y".to_string()});},
            app_definitions::AttributeType::Month => {index.push(hashmap!{"value".to_string() => time.entry.value.clone(), "type".to_string() => "time:m".to_string()});},
            app_definitions::AttributeType::Day => {index.push(hashmap!{"value".to_string() => time.entry.value.clone(), "type".to_string() => "time:d".to_string()});},
            app_definitions::AttributeType::Hour => {index.push(hashmap!{"value".to_string() => time.entry.value.clone(), "type".to_string() => "time:h".to_string()});},
            _ => {}
        };
    }
    index.push(hashmap!{"value".to_string() => exp_type[0].entry.value.clone(), "type".to_string() => "type".to_string()});
    let index_string = index.clone().iter().map(|qp| qp["value"].clone()).collect::<Vec<String>>().join("/");
    //add link on expression to user who made the resonation?
    let hook_definitions = vec![FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{indexes: index.clone(), context: user_pack.clone(), expression: expression.clone(), link_type: "resonation".to_string(), index_string: index_string.clone()}},
                                FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "resonation".to_string(), tag: "".to_string(), direction: "both".to_string(), parent_expression: user_pack, child_expression: expression}}];
    utils::handle_hooks(hook_definitions)?;
    Ok("Resonation Generated".to_string())
}