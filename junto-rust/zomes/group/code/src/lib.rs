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
extern crate holochain_json_derive;
extern crate types;
extern crate utils;

pub mod get;
pub mod definition;

define_zome! {
    entries: [
        definition::group_definition()
    ]

    genesis: || { Ok(()) }

    functions: []

    traits: {
        hc_public []
    }
}
