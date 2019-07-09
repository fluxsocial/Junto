#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
extern crate maplit;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate holochain_json_derive;
extern crate types;
extern crate utils;

pub mod group;
pub mod definition;


use hdk::{
    error::{
        ZomeApiResult
    },
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_json_api::{
        json::JsonString,
        error::JsonError
    }
};

use types::{
    app_definition,
    function_definition
};

define_zome! {
    entries: [
        definition::group_definition()
    ]

    genesis: || { Ok(()) }

    functions: [
        add_pack_member: {
            inputs: |username_address: Address|,
            outputs: |result: ZomeApiResult<JsonString>|,
            handler: group::add_pack_member
        }
        add_member_to_group: {
            inputs: |username_address: Address, group: Address|,
            outputs: |result: ZomeApiResult<JsonString>|,
            handler: group::add_member_to_group
        }
        remove_group_member: {
            inputs: |username_address: Address, group: Address|,
            outputs: |result: ZomeApiResult<JsonString>|,
            handler: group::remove_group_member
        }
        group_members: {
            inputs: |group: Address|,
            outputs: |result: ZomeApiResult<function_definition::GroupMembers>|,
            handler: group::get_group_members
        }
        is_group_member: {
            inputs: |group: Address, user: Address|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: group::is_group_member
        }
        user_pack: {
            inputs: |username_address: Address|,
            outputs: |result: ZomeApiResult<function_definition::EntryAndAddress<app_definition::Group>>|,
            handler: group::get_user_pack
        }
        get_user_member_packs: {
            inputs: |group: Address, user: Address|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: group::is_group_member
        }
    ]

    traits: {
        hc_public [
            user_pack,
            add_pack_member,
            add_member_to_group,
            remove_group_member,
            group_members,
            is_group_member,
            user_pack,
            get_user_member_packs
        ]
    }
}
