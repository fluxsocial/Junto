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
#[macro_use]
extern crate strum_macros;

pub mod app_definition;
pub mod function_definition;
pub mod anchor_definition;
pub mod attribute_definition;
pub mod bucket_definition;
pub mod collection_definition;
pub mod config_definition;
pub mod group_definition;
pub mod perspective_definition;
pub mod post_definition;
pub mod user_definition;