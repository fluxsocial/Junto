#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate holochain_core_types_derive;

define_zome! {
    entries: [
    ]

    genesis: || { Ok(()) }

    functions: []

    traits: {
        hc_public []
    }
}
