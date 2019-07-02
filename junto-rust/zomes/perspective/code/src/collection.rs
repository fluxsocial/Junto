//Module to handle all channel related operations
use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        entry::Entry, 
        cas::content::Address,
        link::LinkMatch
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

// //Commits den entry to DHT and runs necassary hooks
// pub fn commit_collection(collection: app_definitions::Collection, tag: String) -> ZomeApiResult<Address> {
//     let parent = collection.parent.clone();
//     let entry = Entry::App("collection".into(), collection.into());
//     let address = hdk::commit_entry(&entry)?;
//     //Build vector describing hook functions which should run to correctly link this data
//     let hook_definitions = vec![FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "collection", tag: tag.as_str(), direction: "reverse", parent_expression: address.clone(), child_expression: parent.clone()}},
//                                 FunctionDescriptor{name: "link_expression", parameters: FunctionParameters::LinkExpression{link_type: "auth", tag: "owner", direction: "forward", parent_expression: address.clone(), child_expression: parent}}];

//     utils::handle_hooks(hook_definitions)?;
//     Ok(address)
// }

// //Create den(s) (multiple dens as signified by app_definitions data) and link to user with required tags as defined by definitons data
// pub fn create_den(username_address: &Address, first_name: String) -> ZomeApiResult<UserDens> {
//     hdk::debug("Creating dens")?;
//     let private_den = app_definitions::Collection{ //Create private den
//         parent: username_address.clone(),
//         name: (first_name.clone() + "'s Den").to_string(),
//         privacy: app_definitions::Privacy::Private,
//     };
//     let shared_den = app_definitions::Collection{ //Create shared den - den viewable by people in your pack
//         parent: username_address.clone(),
//         name: (first_name.clone()  + "'s Den").to_string(),
//         privacy: app_definitions::Privacy::Shared,
//     };
//     let public_den = app_definitions::Collection{ //Create public den - personal expression place viewable by everyone
//         parent: username_address.clone(),
//         name: (first_name.clone()  + "'s Den").to_string(),
//         privacy: app_definitions::Privacy::Public,
//     };

//     let private_den_address = commit_collection(private_den.clone(), String::from("den"))?;
//     let shared_den_address = commit_collection(shared_den.clone(), String::from("den"))?;
//     let public_den_address = commit_collection(public_den.clone(), String::from("den"))?;

//     Ok(UserDens{private_den: EntryAndAddress{address: private_den_address, entry: private_den}, 
//                         shared_den: EntryAndAddress{address: shared_den_address, entry: shared_den}, 
//                         public_den: EntryAndAddress{address: public_den_address, entry: public_den}})
// }

// pub fn create_collection(collection: app_definitions::Collection, collection_tag: String) -> ZomeApiResult<EntryAndAddress<app_definitions::Collection>>{
//     let collection_address = commit_collection(collection.clone(), collection_tag)?;
//     Ok(EntryAndAddress{address: collection_address, entry: collection})
// }

// pub fn is_collection_owner(collection: Address, user: Address) -> ZomeApiResult<bool>{
//     let den_owner_results = utils::get_links_and_load_type::<app_definitions::UserName>(&collection, LinkMatch::Exactly("auth"), LinkMatch::Exactly("owner"))?;
//     Ok(den_owner_results[0].address == user)
// }