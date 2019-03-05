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
    holochain_core_types::{
        json::JsonString, 
        cas::content::Address,
        hash::HashString
    }
};

mod expressions;

define_zome! {
    entries: [
       expressions::definitions::user_entry_definitions::user_definition(),
       expressions::definitions::time_entry_definitions::time_definiton(),
       expressions::definitions::channel_entry_definitions::channel_definition(),
       expressions::definitions::group_entry_definitions::group_definition(),
       expressions::definitions::post_entry_definitions::post_definition(),
       expressions::definitions::post_entry_definitions::resonation_definition()
    ]

    genesis: || { 
        {
            match expressions::time::create_timestamps(&HashString::encode_from_str(&DNA_ADDRESS.to_string(), Hash::SHA2256), DNA_ADDRESS.to_string()){//Create core application timestamps "global"
                Ok(_) => {},
                Err(hdk_err) => return Err(hdk_err.to_string())
            };
            Ok(())
        }
    }

    functions: {
        main (Public) { //Capability main - public 
            create_user: {
                inputs: |user_data: expressions::definitions::app_definitions::User|,
				outputs: |result: JsonString|,
				handler: expressions::user::handle_create_user
            }
            get_user:{
                inputs: |user: Address|,
				outputs: |result: JsonString|,
				handler: expressions::user::handle_get_user
            }
        }
    }
}
