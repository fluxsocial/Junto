#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use] 
extern crate maplit;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
extern crate chrono;

use multihash::Hash;
use hdk::{
    api::DNA_ADDRESS,
    error::{ZomeApiResult, ZomeApiError},
    holochain_core_types::{json::JsonString, hash::HashString}
};

mod user;
mod utils;
mod definitions;

define_zome! {
    entries: [
       user::user_definition()
    ]

    genesis: || { 
        {
            match utils::create_timestamps(HashString::encode_from_str(&DNA_ADDRESS.to_string(), Hash::SHA2256)){//Create core application timestamps "global"
                Ok(_) => {},
                Err(hdk_err) => return Err(hdk_err.to_string())
            };
            Ok(())
        }
    }

    functions: {
        main (Public) {
            create_user: {
                inputs: |user_data: definitions::User|,
				outputs: |result: JsonString|,
				handler: user::handle_create_user
            }
        }
    }
}
