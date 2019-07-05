#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
extern crate types;
extern crate utils;

use hdk::{
    error::{
        ZomeApiResult
    },
    holochain_core_types::{
        cas::content::Address,
        json::JsonString,
        error::HolochainError
    }
};

pub mod user;

define_zome! {
    entries: [
        types::user_definition::username_definition(),
        types::user_definition::user_definition()
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
