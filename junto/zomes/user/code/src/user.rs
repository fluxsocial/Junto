use hdk::{
    error::{
        ZomeApiResult,
        ZomeApiError
    },
    holochain_core_types::{
        entry::Entry, 
        link::LinkMatch
    },
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_json_api::{
        json::JsonString
    },
    api::{
        AGENT_ADDRESS
    }
};

use serde_json::json;
use std::convert::TryInto;
use std::convert::TryFrom;

//Our modules for holochain actions
use utils;
use types::{
    app_definition,
    function_definition::{
        FunctionDescriptor,
        FunctionParameters,
        EntryAndAddress,
        CreateUserInformation,
        JuntoUser
    }
};

//Create methods
//Function to create user and all necassary expression centers for the user
pub fn handle_create_user(user_data: CreateUserInformation) -> ZomeApiResult<JuntoUser> {
    let username_struct = app_definition::UserName{username: user_data.username.clone()};
    let username_hook = Entry::App("username".into(), username_struct.clone().into()); //Username is the starting point of a users tree - from this comes profile(s), den, pack etc
    let username_address = hdk::commit_entry(&username_hook)?;
    let user_meta_data = app_definition::User{parent: username_address.clone(), first_name: user_data.first_name.clone(), last_name: user_data.last_name, bio: user_data.bio, profile_picture: user_data.profile_picture, verified: true};
    let entry = Entry::App("user".into(), user_meta_data.clone().into());
    let address = hdk::commit_entry(&entry)?;

    hdk::link_entries(&AGENT_ADDRESS, &address, "user", "")?; 
    hdk::link_entries(&AGENT_ADDRESS, &username_address, "username", "")?; 
    hdk::link_entries(&username_address, &address, "profile", "")?;
    hdk::debug("Creating hook definitions")?;
    //Build hook definitions to link user to timestamps and create pack/den
    let hook_definitions = vec![FunctionDescriptor{name: "create_pack", parameters: FunctionParameters::CreatePack{username_address: username_address.clone(), first_name: user_data.first_name.clone()}},
                                FunctionDescriptor{name: "create_den", parameters: FunctionParameters::CreateDen{username_address: username_address.clone(), first_name: user_data.first_name}}];

    let hook_result = utils::helpers::handle_hooks(hook_definitions)?;
    let pack = hook_result[0].clone().create_pack_result()?;
    let dens = hook_result[1].clone().create_den_result()?;
    let user_perspective = hdk::call(hdk::THIS_INSTANCE, "perspective", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                                "create_perspective", JsonString::from(json!({"name": "Default Perspective"})))?;
    let user_perspective: ZomeApiResult<EntryAndAddress<types::app_definition::Perspective>> = user_perspective.try_into()?;

    let junto_user = JuntoUser{profile: EntryAndAddress{entry: user_meta_data.into(), address: address}, username: EntryAndAddress{entry: username_struct.into(), address: username_address},
                                private_den: dens.private_den, shared_den: dens.shared_den, public_den: dens.public_den, pack: pack, user_perspective: user_perspective?};
    Ok(junto_user)
}

pub fn get_user_data_by_agent_address() -> ZomeApiResult<JuntoUser> {
    let username = utils::helpers::get_links_and_load_type::<app_definition::UserName>(&AGENT_ADDRESS, LinkMatch::Exactly("username"), LinkMatch::Exactly(""))?.remove(0);
    let mut profile = utils::helpers::get_links_and_load_type::<app_definition::User>(&username.address, LinkMatch::Exactly("profile"), LinkMatch::Exactly(""))?;

    let den_links = utils::helpers::get_links_and_load_type::<app_definition::Collection>(&username.address, LinkMatch::Exactly("collection"), LinkMatch::Exactly("den"))?;
    let mut private_den = None;
    let mut shared_den = None;
    let mut public_den = None;
    for den in den_links{
        if den.entry.privacy == app_definition::Privacy::Private{
            private_den = Some(den);
        } else if den.entry.privacy == app_definition::Privacy::Shared{
            shared_den = Some(den);
        } else if den.entry.privacy == app_definition::Privacy::Public{
            public_den = Some(den);
        };
    };
    if private_den.is_none() == true{
        return Err(ZomeApiError::from("User has no private den".to_string()))
    } else if shared_den.is_none() == true{
        return Err(ZomeApiError::from("User has no shared den".to_string()))
    } else if public_den.is_none() == true{
        return Err(ZomeApiError::from("User has no public den".to_string()))
    };

    let mut pack = utils::helpers::get_links_and_load_type::<app_definition::Group>(&username.address, LinkMatch::Exactly("group"), LinkMatch::Exactly("pack"))?;
    let mut user_perspective = utils::helpers::get_links_and_load_type::<app_definition::Perspective>(&username.address, LinkMatch::Exactly("perspective"), LinkMatch::Exactly(""))?; 
    //In return type we are just using perspective at index 0 - this will get hairy when more perspectives are created
    //but for now it works
    Ok(JuntoUser{profile: profile.remove(0), username: username, private_den: private_den.unwrap(), shared_den: shared_den.unwrap(), public_den: public_den.unwrap(), pack: pack.remove(0), user_perspective: user_perspective.remove(0)})
}

//Get methods 
//Returns user JsonObject from a given address
pub fn get_username_from_address(username_address: Address) -> JsonString {
    let entry = hdk::get_entry(&username_address);
    match entry {
        Ok(Some(Entry::App(_, entry_value))) => {
            match app_definition::UserName::try_from(&entry_value){
                Ok(entry) => json!({ "Ok": entry }).into(),
                Err(_err) => json!({ "Err": "Address specified was not a username"}).into()
            }
        },
        Ok(Some(_)) => {json!({ "Err": "Address specified was not an app entry" }).into()},
        Ok(None) => json!({ "Ok": {} }).into(),
        Err(hdk_err) => hdk_err.into() 
    }
}

pub fn get_user_profile_from_address(username_address: Address) -> ZomeApiResult<EntryAndAddress<app_definition::User>> {
    let user_links = utils::helpers::get_links_and_load_type::<app_definition::User>(&username_address, LinkMatch::Exactly("profile"), LinkMatch::Exactly(""))?;
    if user_links.len() == 0{
        return Err(ZomeApiError::from("User address does not have any profile links".to_string()))
    };
    Ok(user_links[0].clone())
}

pub fn get_user_profile_by_agent_address() -> ZomeApiResult<EntryAndAddress<app_definition::User>>{
    let user_links = utils::helpers::get_links_and_load_type::<app_definition::User>(&AGENT_ADDRESS, LinkMatch::Exactly("user"), LinkMatch::Exactly(""))?;
    if user_links.len() == 0{
        return Err(ZomeApiError::from("agent does not have any profile links".to_string()))
    };
    Ok(user_links[0].clone())
}

pub fn get_user_username_by_agent_address() -> ZomeApiResult<EntryAndAddress<app_definition::UserName>>{
    let user_name_links = utils::helpers::get_links_and_load_type::<app_definition::UserName>(&AGENT_ADDRESS, LinkMatch::Exactly("username"), LinkMatch::Exactly(""))?;
    if user_name_links.len() == 0{
        return Err(ZomeApiError::from("agent does not have any profile links".to_string()))
    };
    Ok(user_name_links[0].clone())
}