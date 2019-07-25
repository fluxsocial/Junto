#![feature(vec_remove_item)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate types;
extern crate utils;

pub mod indexing;
pub mod dos;
pub mod query;
pub mod post;

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
        types::expression_definition::post_definition(),
        types::bucket_definition::bucket_definition(),
        types::attribute_definition::attribute_definition(),
        types::group_definition::group_definition()
    ]

    genesis: || { Ok(()) }

    functions: [
        query_expressions: {
            inputs: |perspective: String, attributes: Vec<String>, query_options: types::function_definition::QueryOptions, 
                    target_type: types::function_definition::QueryTarget, query_type: types::function_definition::QueryType, dos: u32, 
                    seed: String, resonations: bool|,
            outputs: |result: ZomeApiResult<JsonString>|,
            handler: query::query_expressions
        }
        get_expression: {
            inputs: |expression: Address|,
            outputs: |result: ZomeApiResult<types::function_definition::ExpressionData>|,
            handler: query::get_expression
        }
        post_expression: {
            inputs: |expression: types::app_definition::ExpressionPost, attributes: Vec<String>, context: Vec<Address>|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: post::handle_post_expression
        }
        post_comment_expression: {
            inputs: |expression: types::app_definition::ExpressionPost, parent_expression: Address|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: post::post_comment_expression
        }
        post_resonation: {
            inputs: |expression: Address|,
            outputs: |result: ZomeApiResult<String>|,
            handler: post::post_resonation
        }
    ]

    traits: {
        hc_public [
            query_expressions,
            get_expression,
            post_expression,
            post_comment_expression,
            post_resonation
        ]
    }
}
