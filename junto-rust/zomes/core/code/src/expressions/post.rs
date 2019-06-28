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
    let mut indexes: Vec<HashMap<&'static str, String>> = attributes.iter().map(|attribute| hashmap!{"type" => "channel".to_string(), "value" => attribute.to_string()}).collect();

    let expression_type = expression.expression_type.clone().to_string();
    let entry = Entry::App("expression_post".into(), expression.into());
    let address = hdk::commit_entry(&entry)?;
    let username_entry_address = user::get_user_username_by_agent_address()?;
    let timestamps = utils::get_entries_timestamp(&address)?;

    hdk::debug("Link user to expression as owner")?;
    hdk::api::link_entries(&address, &username_entry_address.address, "auth".to_string(), "owner".to_string())?;

    indexes.push(hashmap!{"type" => "user".to_string(), "value" => username_entry_address.entry.username.to_lowercase()});
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

    let hook_definitions = build_hooks(context, &address, &indexes, index_string.as_str())?; //build function hooks that need to be ran on expression based on which contexts are being used
    indexing::create_post_attributes(&indexes, &address)?;
    hdk::debug("Hook definitions generated")?;

    utils::handle_hooks(hook_definitions)?;
    Ok(address)
}

pub fn post_comment_expression(expression: app_definitions::ExpressionPost, parent_expression: Address) -> ZomeApiResult<Address> {
    hdk::debug("Making a comment")?;
    let _parent_entry = hdk::utils::get_as_type::<app_definitions::ExpressionPost>(parent_expression.clone())
        .map_err(|_err| ZomeApiError::from(String::from("Parent expression was not of type ExpressionPost")))?;
    let expression_type = expression.expression_type.clone().to_string();
    let entry = Entry::App("expression_post".into(), expression.into());
    let address = hdk::commit_entry(&entry)?;
    let username_entry_address = user::get_user_username_by_agent_address()?;
    let timestamps = utils::get_entries_timestamp(&address)?;
    
    let indexes = vec![hashmap!{"type" => "channel".to_string(), "value" => "*null*".to_string()},
                            hashmap!{"type" => "channel".to_string(), "value" => "*null*".to_string()},
                            hashmap!{"type" => "channel".to_string(), "value" => "*null*".to_string()},
                            hashmap!{"type" => "channel".to_string(), "value" => "*null*".to_string()},
                            hashmap!{"type" => "user".to_string(), "value" => username_entry_address.entry.username.to_lowercase()},
                            hashmap!{"type" => "type".to_string(), "value" => expression_type.to_lowercase()},
                            hashmap!{"type" => "time:y".to_string(), "value" => timestamps["year"].to_string()},
                            hashmap!{"type" => "time:m".to_string(), "value" => timestamps["month"].to_string()},
                            hashmap!{"type" => "time:d".to_string(), "value" => timestamps["day"].to_string()},
                            hashmap!{"type" => "time:h".to_string(), "value" => timestamps["hour"].to_string()}];

    let mut index_string = indexes.clone().iter().map(|qp| format!("{}<{}>", qp["value"], qp["type"])).collect::<Vec<String>>().join("/");
    index_string = format!("{}{}{}", "/", index_string, "/");
    hdk::debug(format!("Index string: {}", index_string))?;
    indexing::create_post_attributes(&indexes, &address)?;
    hdk::api::link_entries(&address, &username_entry_address.address, "auth".to_string(), "owner".to_string())?;
    let hook_definitions = vec![FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "sub_expression", tag: index_string.as_str(), direction: "forward", parent_expression: username_entry_address.address, child_expression: address.clone()}},
                                    FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "sub_expression", tag: index_string.as_str(), direction: "forward", parent_expression: parent_expression.clone(), child_expression: address.clone()}},
                                    FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "parent_expression", tag: "", direction: "forward", parent_expression: address.clone(), child_expression: parent_expression}}];
    utils::handle_hooks(hook_definitions)?;
    Ok(address)
}

pub fn build_hooks<'a>(contexts: Vec<Address>, address: &Address, indexes: &'a Vec<HashMap<&'static str, String>>, index_string: &'a str) 
                        -> ZomeApiResult<Vec<FunctionDescriptor<'a>>> {
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
            hook_definitions.push(FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "expression_post", tag: index_string, direction: "forward", parent_expression: user_name_address.clone(), child_expression: address.clone()}});
            //Link between random bit bucket and expression so random post querying can happen on this post
            hook_definitions.push(FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "expression_post", tag: index_string, direction: "forward", parent_expression: bit_bucket.clone(), child_expression: address.clone()}});
            //Link expression to private den
            hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{indexes: indexes, context: private_den.clone(),  expression: address.clone(), index_string: index_string, link_type: "expression_post"}});
            //Link expression to shared den
            hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{indexes: indexes, context: shared_den.clone(), expression: address.clone(), index_string: index_string, link_type: "expression_post"}});
            //Link expression to public den
            hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{indexes: indexes, context: public_den.clone(), expression: address.clone(), index_string: index_string, link_type: "expression_post"}});
            //Link expression to user pack
            hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{indexes: indexes, context: user_pack.clone(), expression: address.clone(), index_string: index_string, link_type: "expression_post"}});
            for pack in &member_results{ //Link expression to each pack user is a member of
                hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{indexes: indexes, context: pack.clone(), expression: address.clone(), index_string: index_string, link_type: "expression_post"}});
            }; 
        } else {
            if local_contexts.contains(&context) == true && collective_count == 1 {return Err(ZomeApiError::from("You have submitted a default Junto context and global context, you can only submit one or the other".to_string()))}
            let _privacy_auth_result = utils::run_context_auth(context, &user_name_address)?
                .ok_or(ZomeApiError::from("Context address was not a collection, group or dna address (global context)".to_string()))?;
            hook_definitions.push(FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{indexes: indexes, context: context.clone(), expression: address.clone(), index_string: index_string, link_type: "expression_post"}});
        }
    };
    Ok(hook_definitions)
}

//Function to handle the resonation of an expression post - will put the post into packs which the post should be resonated into
pub fn handle_resonation(expression: Address) -> ZomeApiResult<String>{
    let expression_entry = hdk::utils::get_as_type::<app_definitions::ExpressionPost>(expression.clone())
        .map_err(|_err| ZomeApiError::from(String::from("Expression was not of type ExpressionPost")))?;
    let username = user::get_user_username_by_agent_address()?;
    let user_pack = user::get_user_pack(username.address.clone())?.address;

    let mut channels = utils::get_links_and_load_type::<app_definitions::Attribute>(&expression, LinkMatch::Exactly("channels"), LinkMatch::Any)?
                        .iter().map(|channel| channel.entry.value.clone()).collect::<Vec<_>>();
    let owner = utils::get_links_and_load_type::<app_definitions::UserName>(&expression, LinkMatch::Exactly("auth"), LinkMatch::Exactly("owner"))?;
    let timestamps = utils::get_entries_timestamp(&expression)?;
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
    let hook_definitions = vec![FunctionDescriptor{name: "create_post_index", parameters: FunctionParameters::CreatePostIndex{indexes: &indexes, context: user_pack.clone(), expression: expression.clone(), link_type: "resonation", index_string: index_string.as_str()}},
                                FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "resonation", tag: "", direction: "forward", parent_expression: user_pack, child_expression: expression.clone()}},
                                FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "resonation", tag: "", direction: "forward", parent_expression: expression, child_expression: username.address}}];
    utils::handle_hooks(hook_definitions)?;
    Ok("Resonation Generated".to_string())
}