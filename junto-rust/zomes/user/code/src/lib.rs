#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate holochain_core_types_derive;
extern crate types;
extern crate utils;

define_zome! {
    entries: [
        types::user_definition::username_definition(),
        types::user_definition::user_definition()
    ]

    genesis: || { Ok(()) }

    functions: []

    traits: {
        hc_public []
    }
}
