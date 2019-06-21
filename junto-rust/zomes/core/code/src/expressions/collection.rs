//Module to handle all channel related operations
use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        entry::Entry, 
        cas::content::Address
    }
};

use super::utils;
// use super::group;
use super::user;
use super::definitions::{
    app_definitions,
    function_definitions::{
        FunctionDescriptor,
        FunctionParameters,
        UserDens,
        EntryAndAddress
    }
};

//Commits den entry to DHT and runs necassary hooks
pub fn commit_collection(collection: app_definitions::Collection, user: &Address) -> ZomeApiResult<Address> {
    let entry = Entry::App("collection".into(), collection.into());
    let address = hdk::commit_entry(&entry)?;
    //Build vector describing hook functions which should run to correctly link this data
    let hook_definitions = vec![FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "collection".to_string(), tag: "den".to_string(), direction: "reverse".to_string(), parent_expression: address.clone(), child_expression: user.clone()}},
                                FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "auth".to_string(), tag: "owner".to_string(), direction: "forward".to_string(), parent_expression: address.clone(), child_expression: user.clone()}}];

    utils::handle_hooks(hook_definitions)?;
    Ok(address)
}

//Create den(s) (multiple dens as signified by app_definitions data) and link to user with required tags as defined by definitons data
pub fn create_den(username_address: &Address, first_name: String) -> ZomeApiResult<UserDens> {
    hdk::debug("Creating dens")?;
    let private_den = app_definitions::Collection{ //Create private den
        parent: username_address.clone(),
        name: (first_name.clone() + "'s Den").to_string(),
        privacy: app_definitions::Privacy::Private,
    };
    let shared_den = app_definitions::Collection{ //Create shared den - den viewable by people in your pack
        parent: username_address.clone(),
        name: (first_name.clone()  + "'s Den").to_string(),
        privacy: app_definitions::Privacy::Shared,
    };
    let public_den = app_definitions::Collection{ //Create public den - personal expression place viewable by everyone
        parent: username_address.clone(),
        name: (first_name.clone()  + "'s Den").to_string(),
        privacy: app_definitions::Privacy::Public,
    };

    let private_den_address = commit_collection(private_den.clone(), &username_address)?;
    let shared_den_address = commit_collection(shared_den.clone(), &username_address)?;
    let public_den_address = commit_collection(public_den.clone(), &username_address)?;

    Ok(UserDens{private_den: EntryAndAddress{address: private_den_address, entry: private_den}, 
                        shared_den: EntryAndAddress{address: shared_den_address, entry: shared_den}, 
                        public_den: EntryAndAddress{address: public_den_address, entry: public_den}})
}

pub fn is_collection_owner(collection: Address, user: Address) -> ZomeApiResult<bool>{
    let den_owner_results = utils::get_links_and_load_type::<app_definitions::UserName>(&collection, Some("auth".to_string()), Some("owner".to_string()))?;
    Ok(den_owner_results[0].address == user)
}