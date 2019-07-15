#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate types;
extern crate utils;

pub mod user;

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
        types::user_definition::username_definition(),
        types::user_definition::user_definition(),
        types::attribute_definition::attribute_definition(),
        types::group_definition::group_definition(),
        types::collection_definition::collection_definition()
    ]

    genesis: || { Ok(()) }

    functions: [
        create_user: {
            inputs: |user_data: types::function_definition::CreateUserInformation|,
            outputs: |result: ZomeApiResult<types::function_definition::JuntoUser>|,
            handler: user::handle_create_user
        }
        get_username_from_address: {
            inputs: |username_address: Address|,
            outputs: |result: JsonString|,
            handler: user::get_username_from_address
        }
        get_user_profile_from_address: {
            inputs: |username_address: Address|,
            outputs: |result: ZomeApiResult<types::function_definition::EntryAndAddress<types::app_definition::User>>|,
            handler: user::get_user_profile_from_address
        }
        get_user_profile_by_agent_address: {
            inputs: | |,
            outputs: |result: ZomeApiResult<types::function_definition::EntryAndAddress<types::app_definition::User>>|,
            handler: user::get_user_profile_by_agent_address
        }
        get_user_username_by_agent_address: {
            inputs: | |,
            outputs: |result: ZomeApiResult<types::function_definition::EntryAndAddress<types::app_definition::UserName>>|,
            handler: user::get_user_username_by_agent_address
        }
    ]

    traits: {
        hc_public [
            create_user,
            get_username_from_address,
            get_user_profile_from_address,
            get_user_profile_by_agent_address,
            get_user_username_by_agent_address
        ]
    }
}
