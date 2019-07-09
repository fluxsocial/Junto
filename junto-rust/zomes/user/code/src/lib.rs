#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate holochain_json_derive;
extern crate types;
extern crate utils;

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

pub mod user;
pub mod definition;

define_zome! {
    entries: [
        definition::username_definition(),
        definition::user_definition()
    ]

    genesis: || { Ok(()) }

    functions: [
        show_env: {
            inputs: | |,
            outputs: |result: ZomeApiResult<types::function_definition::Env>|,
            handler: user::show_env
        }
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
            show_env,
            create_user,
            get_username_from_address,
            get_user_profile_from_address,
            get_user_profile_by_agent_address,
            get_user_username_by_agent_address
        ]
    }
}
