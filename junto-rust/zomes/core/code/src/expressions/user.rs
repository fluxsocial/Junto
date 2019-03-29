use hdk::{
    AGENT_ADDRESS,
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        cas::content::Address,
        entry::Entry, 
        json::JsonString
    }
};

//Our modules for holochain actions
use super::utils;
use super::definitions::{
    app_definitions,
    function_definitions::{
        FunctionDescriptor,
        FunctionParameters,
        UserDens
    }
};

//Create methods
//Function to create user and all necassary expression centers for the user
pub fn handle_create_user(username: String, user_data: app_definitions::User) -> ZomeApiResult<Address> {
    let username_struct = app_definitions::UserName{username: username};
    let username_hook = Entry::App("username".into(), username_struct.into()); //Username is the starting point of a users tree - from this comes profile(s), den, pack etc
    let username_address = hdk::commit_entry(&username_hook)?;

    let entry = Entry::App("user".into(), user_data.into());
    match hdk::commit_entry(&entry) {
        Ok(address) => {
            //Build hook definitions to link user to timestamps and create pack/den
            let hook_definitions = vec![FunctionDescriptor{name: "global_time_to_expression", parameters: FunctionParameters::GlobalTimeToExpression{tag: "user", direction: "reverse", expression_address: username_address.clone()}},
                            FunctionDescriptor{name: "create_pack", parameters: FunctionParameters::CreatePack{user: username_address.clone()}},
                            FunctionDescriptor{name: "create_den", parameters: FunctionParameters::CreateDen{user: username_address.clone()}}];

            match utils::handle_hooks("User".to_string(), hook_definitions){
                Ok(result) => {
                    hdk::link_entries(&AGENT_ADDRESS, &address, "user"); 
                    hdk::link_entries(&AGENT_ADDRESS, &username_address, "username"); 
                    hdk::link_entries(&username_address, &address, "profile");
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
pub fn handle_get_user(user: Address) -> JsonString {
    match hdk::get_entry(&user){
        Ok(result) => json!({ "user": result }).into(),
        Err(hdk_err) => hdk_err.into()
    }
}

pub fn get_user_profile() -> ZomeApiResult<app_definitions::GetLinksLoadElement<app_definitions::User>>{
    let user_links = utils::get_links_and_load_type::<String, app_definitions::User>(&AGENT_ADDRESS, "user".to_string())?;
    if user_links.len() == 0{
        return Err(ZomeApiError::from("agent does not have any profile links".to_string()))
    };
    Ok(user_links[0].clone())
}

pub fn get_user_username() -> ZomeApiResult<app_definitions::GetLinksLoadElement<app_definitions::UserName>>{
    let user_name_links = utils::get_links_and_load_type::<String, app_definitions::UserName>(&AGENT_ADDRESS, "username".to_string())?;
    if user_name_links.len() == 0{
        return Err(ZomeApiError::from("agent does not have any profile links".to_string()))
    };
    Ok(user_name_links[0].clone())
}

pub fn get_user_dens(user_profile: &Address) -> ZomeApiResult<UserDens>{
    let den_links = utils::get_links_and_load_type::<String, app_definitions::Channel>(user_profile, "den".to_string())?;
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

pub fn get_user_pack(user_name_address: &Address) -> ZomeApiResult<Option<app_definitions::GetLinksLoadElement<app_definitions::Channel>>>{
    let pack_links = utils::get_links_and_load_type::<String, app_definitions::Channel>(user_name_address, "pack".to_string())?;
    if pack_links.len() > 1{
        return Err(ZomeApiError::from("pack links on user greater than 1".to_string()))

    } else if pack_links.len() == 0{
        return Ok(None)
    }
    Ok(Some(pack_links[0].clone()))
}

pub fn get_user_member_packs(user_profile: &Address) -> ZomeApiResult<Vec<app_definitions::GetLinksLoadElement<app_definitions::Channel>>>{
    let pack_links = utils::get_links_and_load_type::<String, app_definitions::Channel>(&user_profile, "pack_member".to_string())?;
    let mut packs: Vec<app_definitions::GetLinksLoadElement<app_definitions::Channel>> = vec![];
    for pack in pack_links{
        packs.push(pack.clone());
    };
    Ok(packs)
}