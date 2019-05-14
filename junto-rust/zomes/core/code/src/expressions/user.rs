use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        cas::content::Address,
        entry::Entry, 
        json::JsonString,
        hash::HashString
    },
    api::{
        AGENT_ADDRESS, AGENT_ID_STR, CAPABILITY_REQ, DNA_ADDRESS, DNA_NAME
    }
};

use std::convert::TryFrom;

//Our modules for holochain actions
use super::utils;
use super::definitions::{
    app_definitions,
    function_definitions::{
        FunctionDescriptor,
        FunctionParameters,
        UserDens,
        GetLinksLoadElement,
        CreateUserInformation,
        UserPack,
        Env
    }
};

/// This handler shows how you can access the globals that are always available
/// inside a zome.  In this case it just creates an object with their values
/// and returns it as the result.
pub fn show_env() -> ZomeApiResult<Env> {
    let _dna_entry = hdk::get_entry(&DNA_ADDRESS)?;
    let _agent_entry = hdk::get_entry(&AGENT_ADDRESS)?;
    Ok(Env {
        dna_name: DNA_NAME.to_string(),
        dna_address: DNA_ADDRESS.to_string(),
        agent_id: AGENT_ID_STR.to_string(),
        agent_address: AGENT_ADDRESS.to_string(),
        cap_request: CAPABILITY_REQ.clone(),
    })
}

//Create methods
//Function to create user and all necassary expression centers for the user
pub fn handle_create_user(user_data: CreateUserInformation) -> ZomeApiResult<Address> {
    let username_struct = app_definitions::UserName{username: user_data.username.clone()};
    let username_hook = Entry::App("username".into(), username_struct.into()); //Username is the starting point of a users tree - from this comes profile(s), den, pack etc
    let username_address = hdk::commit_entry(&username_hook)?;
    let user_meta_data = app_definitions::User{parent: username_address.clone(), first_name: user_data.first_name.clone(), last_name: user_data.last_name, bio: user_data.bio, profile_picture: user_data.profile_picture, verified: true};

    let entry = Entry::App("user".into(), user_meta_data.into());
    match hdk::commit_entry(&entry) {
        Ok(address) => {
            hdk::link_entries(&AGENT_ADDRESS, &address, "user")?; 
            hdk::link_entries(&AGENT_ADDRESS, &username_address, "username")?; 
            hdk::link_entries(&username_address, &address, "profile")?;
            //hdk::link_entries(&hdk::api::DNA_ADDRESS, &username_address, user_data.username.clone())?; //add link on DNA address where tag is username so this can be used for searching later
            //Build hook definitions to link user to timestamps and create pack/den
            let hook_definitions = vec![FunctionDescriptor{name: "global_time_to_expression", parameters: FunctionParameters::TimeToExpression{tag: "user", direction: "reverse", expression_address: username_address.clone(), context: Address::from(DNA_ADDRESS.to_string())}},
                                        FunctionDescriptor{name: "create_pack", parameters: FunctionParameters::CreatePack{username_address: username_address.clone(), first_name: user_data.first_name.clone()}},
                                        FunctionDescriptor{name: "create_den", parameters: FunctionParameters::CreateDen{username_address: username_address.clone(), first_name: user_data.first_name}}];

            match utils::handle_hooks("User".to_string(), hook_definitions){
                Ok(_result) => {
                    Ok(username_address) 
                },
                Err(hdk_err) => return Err(ZomeApiError::from(hdk_err.to_string()))
            }
        }
        Err(hdk_err) => return Err(ZomeApiError::from(hdk_err.to_string())),
    }
}

//Get methods 
//Returns user JsonObject from a given address
pub fn get_username_from_address(username_address: Address) -> JsonString {
    let entry = hdk::get_entry(&username_address);
    match entry {
        Ok(Some(Entry::App(_, entry_value))) => {
            match app_definitions::UserName::try_from(&entry_value){
                Ok(entry) => json!({ "Ok": entry }).into(),
                Err(_err) => json!({ "Err": "Address specified was not a username"}).into()
            }
        },
        Ok(Some(_)) => {json!({ "Err": "Address specified was not an app entry" }).into()},
        Ok(None) => json!({ "Ok": {} }).into(),
        Err(hdk_err) => hdk_err.into() 
    }
}

pub fn get_user_profile_from_address(username_address: Address) -> ZomeApiResult<app_definitions::User> {
    let user_links = utils::get_links_and_load_type::<String, app_definitions::User>(&username_address, "profile".to_string())?;
    if user_links.len() == 0{
        return Err(ZomeApiError::from("User address does not have any profile links".to_string()))
    };
    Ok(user_links[0].entry.clone())
}

pub fn get_user_profile_by_agent_address() -> ZomeApiResult<app_definitions::User>{
    let user_links = utils::get_links_and_load_type::<String, app_definitions::User>(&AGENT_ADDRESS, "user".to_string())?;
    if user_links.len() == 0{
        return Err(ZomeApiError::from("agent does not have any profile links".to_string()))
    };
    Ok(user_links[0].entry.clone())
}

pub fn get_user_profile_address_by_agent_address() -> ZomeApiResult<Address>{
    let user_links = utils::get_links_and_load_type::<String, app_definitions::User>(&AGENT_ADDRESS, "user".to_string())?;
    if user_links.len() == 0{
        return Err(ZomeApiError::from("agent does not have any profile links".to_string()))
    };
    Ok(user_links[0].address.clone())
}

pub fn get_user_username_by_agent_address() -> ZomeApiResult<app_definitions::UserName>{
    let user_name_links = utils::get_links_and_load_type::<String, app_definitions::UserName>(&AGENT_ADDRESS, "username".to_string())?;
    if user_name_links.len() == 0{
        return Err(ZomeApiError::from("agent does not have any profile links".to_string()))
    };
    Ok(user_name_links[0].entry.clone())
}


pub fn get_user_username_address_by_agent_address() -> ZomeApiResult<Address>{
    let user_name_links = utils::get_links_and_load_type::<String, app_definitions::UserName>(&AGENT_ADDRESS, "username".to_string())?;
    if user_name_links.len() == 0{
        return Err(ZomeApiError::from("agent does not have any profile links".to_string()))
    };
    Ok(user_name_links[0].address.clone())
}

pub fn get_user_dens(user: Address) -> ZomeApiResult<UserDens>{
    let den_links = utils::get_links_and_load_type::<String, app_definitions::Channel>(&user, "den".to_string())?;
    let mut private_den = None;
    let mut shared_den = None;
    let mut public_den = None;
    for den in den_links{
        if den.entry.privacy == app_definitions::Privacy::Private{
            private_den = Some(den.clone());
        };
        if den.entry.privacy == app_definitions::Privacy::Shared{
            shared_den = Some(den.clone());
        };
        if den.entry.privacy == app_definitions::Privacy::Public{
            public_den = Some(den.clone());
        };
    };
    Ok(UserDens{private_den: private_den, shared_den: shared_den, public_den: public_den})
}

pub fn get_user_pack(username_address: HashString) -> ZomeApiResult<UserPack>{
    let pack_links = utils::get_links_and_load_type::<String, app_definitions::Group>(&username_address, "pack".to_string())?;
    hdk::debug(format!("Pack links on username: {}", pack_links.len().to_string()))?;
    if pack_links.len() > 1{
        return Err(ZomeApiError::from("pack links on user greater than 1".to_string()))
    } else if pack_links.len() == 0{
        return Ok(UserPack{pack: None})
    }
    Ok(UserPack{pack: Some(pack_links[0].clone())})
}

pub fn get_user_member_packs(username_address: HashString) -> ZomeApiResult<Vec<GetLinksLoadElement<app_definitions::Group>>>{
    let pack_links = utils::get_links_and_load_type::<String, app_definitions::Group>(&username_address, "pack_member".to_string())?;
    let mut packs: Vec<GetLinksLoadElement<app_definitions::Group>> = vec![];
    for pack in pack_links{
        packs.push(pack.clone());
    };
    Ok(packs)
}