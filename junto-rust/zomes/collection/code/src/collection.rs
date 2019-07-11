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
    }
};


use types::{
    app_definition,
    function_definition::{
        FunctionDescriptor,
        FunctionParameters,
        UserDens,
        EntryAndAddress
    }
};
use utils;

//Commits den entry to DHT and runs necassary hooks
pub fn commit_collection(collection: app_definition::Collection, tag: String) -> ZomeApiResult<Address> {
    let parent = collection.parent.clone();
    let entry = Entry::App("collection".into(), collection.into());
    let address = hdk::commit_entry(&entry)?;
    //Build vector describing hook functions which should run to correctly link this data
    let hook_definitions = vec![FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "collection", tag: tag.as_str(), direction: "reverse", parent_expression: address.clone(), child_expression: parent.clone()}},
                                FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "auth", tag: "owner", direction: "forward", parent_expression: address.clone(), child_expression: parent}}];

    utils::helpers::handle_hooks(hook_definitions)?;
    Ok(address)
}

//Create den(s) (multiple dens as signified by app_definition data) and link to user with required tags as defined by definitons data
pub fn create_den(username_address: Address, first_name: String) -> ZomeApiResult<UserDens> {
    hdk::debug("Creating dens")?;
    let private_den = app_definition::Collection{ //Create private den
        parent: username_address.clone(),
        name: (first_name.clone() + "'s Den").to_string(),
        privacy: app_definition::Privacy::Private,
    };
    let shared_den = app_definition::Collection{ //Create shared den - den viewable by people in your pack
        parent: username_address.clone(),
        name: (first_name.clone()  + "'s Den").to_string(),
        privacy: app_definition::Privacy::Shared,
    };
    let public_den = app_definition::Collection{ //Create public den - personal expression place viewable by everyone
        parent: username_address.clone(),
        name: (first_name.clone()  + "'s Den").to_string(),
        privacy: app_definition::Privacy::Public,
    };

    let private_den_address = commit_collection(private_den.clone(), String::from("den"))?;
    let shared_den_address = commit_collection(shared_den.clone(), String::from("den"))?;
    let public_den_address = commit_collection(public_den.clone(), String::from("den"))?;

    Ok(UserDens{private_den: EntryAndAddress{address: private_den_address, entry: private_den}, 
                        shared_den: EntryAndAddress{address: shared_den_address, entry: shared_den}, 
                        public_den: EntryAndAddress{address: public_den_address, entry: public_den}})
}

pub fn create_collection(collection: app_definition::Collection, collection_tag: String) -> ZomeApiResult<EntryAndAddress<app_definition::Collection>>{
    let collection_address = commit_collection(collection.clone(), collection_tag)?;
    Ok(EntryAndAddress{address: collection_address, entry: collection})
}

pub fn is_collection_owner(collection: Address, user: Address) -> ZomeApiResult<bool>{
    let den_owner_results = utils::helpers::get_links_and_load_type::<app_definition::UserName>(&collection, LinkMatch::Exactly("auth"), LinkMatch::Exactly("owner"))?;
    Ok(den_owner_results[0].address == user)
}

pub fn get_user_dens(username_address: Address) -> ZomeApiResult<UserDens>{
    let den_links = utils::helpers::get_links_and_load_type::<app_definition::Collection>(&username_address, LinkMatch::Exactly("collection"), LinkMatch::Exactly("den"))?;
    let mut private_den = None;
    let mut shared_den = None;
    let mut public_den = None;
    for den in den_links{
        if den.entry.privacy == app_definition::Privacy::Private{
            private_den = Some(den.clone());
        };
        if den.entry.privacy == app_definition::Privacy::Shared{
            shared_den = Some(den.clone());
        };
        if den.entry.privacy == app_definition::Privacy::Public{
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