#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate types;
extern crate utils;

pub mod collection;

use hdk::{
    error::{
        ZomeApiResult
    },
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_json_api::{
        json::JsonString,
        error::JsonError
    }
};


define_zome! {
    entries: [
        types::collection_definition::collection_definition(),
        types::user_definition::username_definition()
    ]

    genesis: || { Ok(()) }

    functions: [    
        create_den: {
            inputs: |username_address: Address, first_name: String|,
            outputs: |result: ZomeApiResult<types::function_definition::UserDens>|,
            handler: collection::create_den
        }    
        get_user_dens: {
            inputs: |username_address: Address|,
            outputs: |result: ZomeApiResult<types::function_definition::UserDens>|,
            handler: collection::get_user_dens
        }
        is_collection_owner: {
            inputs: |collection: Address, username_address: Address|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: collection::is_collection_owner
        }
        create_collection: {
            inputs: |collection: types::app_definition::Collection, collection_tag: String|,
            outputs: |result: ZomeApiResult<types::function_definition::EntryAndAddress<types::app_definition::Collection>>|,
            handler: collection::create_collection
        }
    ]

    traits: {
        hc_public [
            create_den,
            get_user_dens,
            is_collection_owner,
            create_collection
        ]
    }
}
