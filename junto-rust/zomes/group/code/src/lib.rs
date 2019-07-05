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
extern crate types;
extern crate utils;

pub mod get;

define_zome! {
    entries: [
        types::group_definition::group_definition()
    ]

    genesis: || { Ok(()) }

    functions: []

    traits: {
        hc_public []
    }
}
