#![feature(try_from, vec_remove_item)]
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
extern crate regex;
extern crate multihash;
extern crate rust_base58;

use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        json::JsonString, 
        cas::content::Address,
        hash::HashString,
        error::HolochainError
    }
};

mod expressions;

use expressions::definitions::app_definitions;
use expressions::definitions::function_definitions;

define_zome! {
    entries: [
        expressions::definitions::user_entry_definitions::user_name_definition(),
        expressions::definitions::user_entry_definitions::user_definition(),
        expressions::definitions::collection_entry_definitions::collection_definition(),
        expressions::definitions::perspective_entry_definitions::perspective_definition(),
        expressions::definitions::group_entry_definitions::group_definition(),
        expressions::definitions::post_entry_definitions::post_definition(),
        expressions::definitions::anchor_entry_definitions::anchor_definition(),
        expressions::definitions::bucket_entry_definitions::bucket_definition(),
        expressions::definitions::config_entry_definitions::config_definition(),
        expressions::definitions::attribute_entry_definitions::attribute_definition()
    ]

    genesis: || { 
        {
            //expressions::channel::create_collective_channel(&Address::from(hdk::api::DNA_ADDRESS.to_string()))?; - this should not happen here but instead in the future callback function initialize
            Ok(())
        }
    }

    functions: [
        create_user: {
            inputs: |user_data: function_definitions::CreateUserInformation|,
            outputs: |result: ZomeApiResult<function_definitions::JuntoUser>|,
            handler: expressions::user::handle_create_user
        }
        get_username_from_address: {
            inputs: |username_address: Address|,
            outputs: |result: JsonString|,
            handler: expressions::user::get_username_from_address
        }
        get_user_profile_from_address: {
            inputs: |username_address: Address|,
            outputs: |result: ZomeApiResult<function_definitions::EntryAndAddress<app_definitions::User>>|,
            handler: expressions::user::get_user_profile_from_address
        }
        get_user_profile_by_agent_address: {
            inputs: | |,
            outputs: |result: ZomeApiResult<function_definitions::EntryAndAddress<app_definitions::User>>|,
            handler: expressions::user::get_user_profile_by_agent_address
        }
        get_user_username_by_agent_address: {
            inputs: | |,
            outputs: |result: ZomeApiResult<function_definitions::EntryAndAddress<app_definitions::UserName>>|,
            handler: expressions::user::get_user_username_by_agent_address
        }
        user_dens: {
            inputs: |username_address: Address|,
            outputs: |result: ZomeApiResult<function_definitions::UserDens>|,
            handler: expressions::user::get_user_dens
        }
        is_den_owner: {
            inputs: |den: Address, user: Address|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: expressions::collection::is_den_owner
        }
        user_pack: {
            inputs: |username_address: HashString|,
            outputs: |result: ZomeApiResult<function_definitions::EntryAndAddress<app_definitions::Group>>|,
            handler: expressions::user::get_user_pack
        }
        add_pack_member: {
            inputs: |username_address: Address|,
            outputs: |result: ZomeApiResult<JsonString>|,
            handler: expressions::group::add_pack_member
        }
        add_member_to_group: {
            inputs: |username_address: Address, group: Address|,
            outputs: |result: ZomeApiResult<JsonString>|,
            handler: expressions::group::add_member_to_group
        }
        remove_group_member: {
            inputs: |username_address: Address, group: Address|,
            outputs: |result: ZomeApiResult<JsonString>|,
            handler: expressions::group::remove_group_member
        }
        group_members: {
            inputs: |group: Address|,
            outputs: |result: ZomeApiResult<function_definitions::GroupMembers>|,
            handler: expressions::group::get_group_members
        }
        is_group_member: {
            inputs: |group: Address, user: Address|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: expressions::group::is_group_member
        }
        get_expression: {
            inputs: |perspective: String, attributes: Vec<String>, query_options: function_definitions::QueryOptions, 
                    target_type: function_definitions::QueryTarget, query_type: function_definitions::QueryType, dos: u32, seed: String|,
            outputs: |result: ZomeApiResult<JsonString>|,
            handler: expressions::query::get_expression
        }
        post_expression: {
            inputs: |expression: app_definitions::ExpressionPost, attributes: Vec<String>, context: Vec<Address>|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: expressions::post::handle_post_expression
        }
        resonation: {
            inputs: |expression: Address|,
            outputs: |result: ZomeApiResult<String>|,
            handler: expressions::post::handle_resonation
        }
        show_env: {
            inputs: | |,
            outputs: |result: ZomeApiResult<function_definitions::Env>|,
            handler: expressions::user::show_env
        }
        create_perspective: {
            inputs: |name: String|,
            outputs: |result: ZomeApiResult<function_definitions::EntryAndAddress<app_definitions::Perspective>>|,
            handler: expressions::perspective::create_perspective
        }
        add_user_to_perspective: {
            inputs: |perspective: Address, target_user: Address|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: expressions::perspective::add_user_to_perspective
        }
        get_perspectives_users: {
            inputs: |perspective: Address|,
            outputs: |result: ZomeApiResult<Vec<function_definitions::EntryAndAddress<app_definitions::UserName>>>|,
            handler: expressions::perspective::get_perspectives_users
        }
        update_bit_prefix: {
            inputs: |bit_prefix: u32|,
            outputs: |result: ZomeApiResult<u32>|,
            handler: expressions::random::update_bit_prefix
        }
    ]

    traits: {
        hc_public [
            create_user, 
            get_username_from_address,
            get_user_profile_from_address,
            get_user_profile_by_agent_address,
            get_user_username_by_agent_address,
            user_dens,
            is_den_owner,
            user_pack,
            add_pack_member,
            add_member_to_group,
            remove_group_member,
            group_members,
            is_group_member,
            get_expression,
            post_expression,
            resonation,
            show_env,
            add_user_to_perspective,
            get_perspectives_users,
            update_bit_prefix
        ]
    }
}
