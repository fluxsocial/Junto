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
extern crate chrono;

use hdk::api::DNA_ADDRESS;
use hdk::holochain_core_types::json::JsonString;

mod user;
mod utils;
mod definitions;

define_zome! {
    entries: [
       user::user_definition()
    ]

    genesis: || { 
        {
            utils::create_timestamps(""); //Create core application timestamps "global"
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
