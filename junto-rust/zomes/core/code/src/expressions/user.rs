use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        cas::content::Address,
        entry::Entry, 
        json::JsonString,
        hash::HashString,
        link::LinkMatch
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
        EntryAndAddress,
        CreateUserInformation,
        Env,
        JuntoUser
    }
};
use super::perspective;

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
pub fn handle_create_user(user_data: CreateUserInformation) -> ZomeApiResult<JuntoUser> {
    let username_struct = app_definitions::UserName{username: user_data.username.clone()};
    let username_hook = Entry::App("username".into(), username_struct.clone().into()); //Username is the starting point of a users tree - from this comes profile(s), den, pack etc
    let username_address = hdk::commit_entry(&username_hook)?;
    let user_meta_data = app_definitions::User{parent: username_address.clone(), first_name: user_data.first_name.clone(), last_name: user_data.last_name, bio: user_data.bio, profile_picture: user_data.profile_picture, verified: true};
    let entry = Entry::App("user".into(), user_meta_data.clone().into());
    let address = hdk::commit_entry(&entry)?;

    hdk::link_entries(&AGENT_ADDRESS, &address, "user", "")?; 
    hdk::link_entries(&AGENT_ADDRESS, &username_address, "username", "")?; 
    hdk::link_entries(&username_address, &address, "profile", "")?;
    //Build hook definitions to link user to timestamps and create pack/den
    let hook_definitions = vec![FunctionDescriptor{name: "time_to_expression", parameters: FunctionParameters::TimeToExpression{link_type: "created_at".to_string(), tag: "".to_string(), direction: "reverse".to_string(), expression_address: username_address.clone()}},
                                FunctionDescriptor{name: "create_pack", parameters: FunctionParameters::CreatePack{username_address: username_address.clone(), first_name: user_data.first_name.clone()}},
                                FunctionDescriptor{name: "create_den", parameters: FunctionParameters::CreateDen{username_address: username_address.clone(), first_name: user_data.first_name}}];

    let hook_result = utils::handle_hooks(hook_definitions)?;
    let pack = hook_result[1].clone().create_pack_result()?;
    let dens = hook_result[2].clone().create_den_result()?;
    let user_perspective = perspective::create_perspective("Default Perspective".to_string())?;
    let junto_user = JuntoUser{profile: EntryAndAddress{entry: user_meta_data.into(), address: address}, username: EntryAndAddress{entry: username_struct.into(), address: username_address},
                                private_den: dens.private_den, shared_den: dens.shared_den, public_den: dens.public_den, pack: pack, user_perspective: user_perspective};
    Ok(junto_user)
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

pub fn get_user_profile_from_address(username_address: Address) -> ZomeApiResult<EntryAndAddress<app_definitions::User>> {
    let user_links = utils::get_links_and_load_type::<app_definitions::User>(&username_address, LinkMatch::Exactly("profile"), LinkMatch::Exactly(""))?;
    if user_links.len() == 0{
        return Err(ZomeApiError::from("User address does not have any profile links".to_string()))
    };
    Ok(user_links[0].clone())
}

pub fn get_user_profile_by_agent_address() -> ZomeApiResult<EntryAndAddress<app_definitions::User>>{
    let user_links = utils::get_links_and_load_type::<app_definitions::User>(&AGENT_ADDRESS, LinkMatch::Exactly("user"), LinkMatch::Exactly(""))?;
    if user_links.len() == 0{
        return Err(ZomeApiError::from("agent does not have any profile links".to_string()))
    };
    Ok(user_links[0].clone())
}

pub fn get_user_username_by_agent_address() -> ZomeApiResult<EntryAndAddress<app_definitions::UserName>>{
    let user_name_links = utils::get_links_and_load_type::<app_definitions::UserName>(&AGENT_ADDRESS, LinkMatch::Exactly("username"), LinkMatch::Exactly(""))?;
    if user_name_links.len() == 0{
        return Err(ZomeApiError::from("agent does not have any profile links".to_string()))
    };
    Ok(user_name_links[0].clone())
}

pub fn get_user_dens(user: Address) -> ZomeApiResult<UserDens>{
    let den_links = utils::get_links_and_load_type::<app_definitions::Collection>(&user, LinkMatch::Exactly("collection"), LinkMatch::Exactly("den"))?;
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
    if private_den.is_none() == true{
        return Err(ZomeApiError::from("User has no private den".to_string()))
    } else if shared_den.is_none() == true{
        return Err(ZomeApiError::from("User has no shared den".to_string()))
    } else if public_den.is_none() == true{
        return Err(ZomeApiError::from("User has no public den".to_string()))
    };
    Ok(UserDens{private_den: private_den.unwrap(), shared_den: shared_den.unwrap(), public_den: public_den.unwrap()})
}

pub fn get_user_pack(username_address: HashString) -> ZomeApiResult<EntryAndAddress<app_definitions::Group>>{
    let pack_links = utils::get_links_and_load_type::<app_definitions::Group>(&username_address, LinkMatch::Exactly("group"), LinkMatch::Exactly("pack"))?;
    hdk::debug(format!("Pack links on username: {}", pack_links.len().to_string()))?;
    if pack_links.len() > 1{
        return Err(ZomeApiError::from("Pack links on user greater than 1".to_string()))
    } else if pack_links.len() == 0{
        return Err(ZomeApiError::from("No pack links on user".to_string()))
    }
    Ok(pack_links[0].clone())
}

pub fn get_user_member_packs(username_address: HashString) -> ZomeApiResult<Vec<EntryAndAddress<app_definitions::Group>>>{
    let pack_links = utils::get_links_and_load_type::<app_definitions::Group>(&username_address, LinkMatch::Exactly("auth"), LinkMatch::Exactly("member"))?;
    let mut packs: Vec<EntryAndAddress<app_definitions::Group>> = vec![];
    for pack in pack_links{
        packs.push(pack.clone());
    };
    Ok(packs)
}