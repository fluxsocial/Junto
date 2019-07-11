#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate types;

pub mod definition;
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
        definition::bucket_definition(),
        definition::config_definition()
    ]

    genesis: || { Ok(()) }

    functions: [
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
            get_current_bit_prefix,
            update_bit_prefix
        ]
    }
}
