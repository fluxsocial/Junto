#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate types;
extern crate utils;

pub mod group;

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

define_zome! {
    entries: [
        types::group_definition::group_definition(),
        types::user_definition::username_definition()
    ]

    genesis: || { Ok(()) }

    functions: [
        create_pack: {
            inputs: |username_address: Address, first_name: String|,
            outputs: |result: ZomeApiResult<types::function_definition::EntryAndAddress<types::app_definition::Group>>|,
            handler: group::create_pack
        }
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
            outputs: |result: ZomeApiResult<types::function_definition::GroupMembers>|,
            handler: group::get_group_members
        }
        is_group_member: {
            inputs: |group: Address, user: Address|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: group::is_group_member
        }
        user_pack: {
            inputs: |username_address: Address|,
            outputs: |result: ZomeApiResult<types::function_definition::EntryAndAddress<types::app_definition::Group>>|,
            handler: group::get_user_pack
        }
        get_user_member_packs: {
            inputs: |username_address: Address|,
            outputs: |result: ZomeApiResult<Vec<types::function_definition::EntryAndAddress<types::app_definition::Group>>>|,
            handler: group::get_user_member_packs
        }
    ]

    traits: {
        hc_public [
            create_pack,
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
