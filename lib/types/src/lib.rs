#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;
#[macro_use]
extern crate strum_macros;

pub mod app_definition;
pub mod function_definition;
pub mod attribute_definition;