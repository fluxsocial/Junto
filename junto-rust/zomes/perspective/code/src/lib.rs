#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
extern crate maplit;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate holochain_json_derive;
extern crate types;
extern crate utils;

pub mod perspective;

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
        types::perspective_definition::perspective_definition()
    ]

    genesis: || { Ok(()) }

    functions: [
        create_perspective: {
            inputs: |name: String|,
            outputs: |result: ZomeApiResult<types::function_definition::EntryAndAddress<types::app_definition::Perspective>>|,
            handler: perspective::create_perspective
        }
        add_user_to_perspective: {
            inputs: |perspective: Address, target_user: Address|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: perspective::add_user_to_perspective
        }
        get_perspectives_users: {
            inputs: |perspective: Address|,
            outputs: |result: ZomeApiResult<Vec<types::function_definition::EntryAndAddress<types::app_definition::UserName>>>|,
            handler: perspective::get_perspectives_users
        }
    ]

    traits: {
        hc_public [
            create_perspective,
            add_user_to_perspective,
            get_perspectives_users
        ]
    }
}
