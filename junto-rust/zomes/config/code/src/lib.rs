#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate types;

pub mod config;

use hdk::{
    error::{
        ZomeApiResult
    },
    holochain_json_api::{
        json::JsonString,
        error::JsonError
    }
};


define_zome! {
    entries: [
        types::config_definition::config_definition(),
        types::anchor_definition::anchor_definition()
    ]

    genesis: || { Ok(()) }

    functions: [
        get_env: {
            inputs: | |,
            outputs: |result: ZomeApiResult<types::function_definition::Env>|,
            handler: config::get_env
        }
        get_current_bit_prefix: {
            inputs: | |,
            outputs: |result: ZomeApiResult<u32>|,
            handler: config::get_current_bit_prefix
        }
        update_bit_prefix: {
            inputs: |bit_prefix: u32|,
            outputs: |result: ZomeApiResult<u32>|,
            handler: config::update_bit_prefix
        }
    ]

    traits: {
        hc_public [
            get_env,
            get_current_bit_prefix,
            update_bit_prefix
        ]
    }
}
