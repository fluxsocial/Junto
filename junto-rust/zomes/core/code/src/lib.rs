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
extern crate chrono;
extern crate regex;

use multihash::Hash;
use hdk::{
    api::DNA_ADDRESS,
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
        expressions::definitions::time_entry_definitions::time_definiton(),
        expressions::definitions::channel_entry_definitions::channel_definition(),
        expressions::definitions::group_entry_definitions::group_definition(),
        expressions::definitions::post_entry_definitions::post_definition()
        //expressions::definitions::post_entry_definitions::resonation_definition()
    ]

    genesis: || { 
        {
            match expressions::time::create_timestamps(&HashString::encode_from_str(&DNA_ADDRESS.to_string(), Hash::SHA2256), DNA_ADDRESS.to_string()){//Create core application timestamps "global"
                Ok(_) => {},
                Err(hdk_err) => return Err(hdk_err.to_string())
            };
            Ok(())
        }
    }

    functions: [
        create_user: {
            inputs: |user_data: function_definitions::CreateUserInformation|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: expressions::user::handle_create_user
        }
        get_username_from_address: {
            inputs: |user: Address|,
            outputs: |result: JsonString|,
            handler: expressions::user::get_username_from_address
        }
        get_user_profile_from_address: {
            inputs: |user: Address|,
            outputs: |result: ZomeApiResult<app_definitions::User>|,
            handler: expressions::user::get_user_profile_from_address
        }
        get_user_profile_by_agent_address: {
            inputs: | |,
            outputs: |result: ZomeApiResult<app_definitions::User>|,
            handler: expressions::user::get_user_profile_by_agent_address
        }
        get_user_profile_address_by_agent_address: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Address>|,
            handler: expressions::user::get_user_profile_address_by_agent_address 
        }
        get_user_username_by_agent_address: {
            inputs: | |,
            outputs: |result: ZomeApiResult<app_definitions::UserName>|,
            handler: expressions::user::get_user_username_by_agent_address
        }
        get_user_username_address_by_agent_address: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Address>|,
            handler: expressions::user::get_user_username_address_by_agent_address
        }
        get_user_dens: {
            inputs: |user: Address|,
            outputs: |result: ZomeApiResult<function_definitions::UserDens>|,
            handler: expressions::user::get_user_dens
        }
        is_den_owner: {
            inputs: |den: Address, user: Address|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: expressions::channel::is_den_owner
        }
        get_user_pack: {
            inputs: |username_address: HashString|,
            outputs: |result: ZomeApiResult<function_definitions::UserPack>|,
            handler: expressions::user::get_user_pack
        }
        add_to_pack: {
            inputs: |username_address: Address|,
            outputs: |result: ZomeApiResult<JsonString>|,
            handler: expressions::group::add_to_pack
        }
        get_pack_members: {
            inputs: |pack: Address|,
            outputs: |result: ZomeApiResult<function_definitions::PackMembers>|,
            handler: expressions::group::get_pack_members
        }
        is_pack_member: {
            inputs: |pack: Address, user: Address|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: expressions::group::is_pack_member
        }
    ]

    traits: {
        hc_public [
            create_user, 
            get_username_from_address,
            get_user_profile_from_address,
            get_user_profile_by_agent_address,
            get_user_profile_address_by_agent_address,
            get_user_username_by_agent_address,
            get_user_username_address_by_agent_address,
            get_user_dens,
            is_den_owner,
            get_user_pack,
            add_to_pack,
            get_pack_members,
            is_pack_member
        ]
    }
}
