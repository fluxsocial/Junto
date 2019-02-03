//Entry Definition(s)
use super::app_definitions;

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing
    }
};

pub fn post_definition() -> ValidatingEntryType {
    entry!(
        name: "expression_post",
        description: "ExpressionPost Object Entry",
        sharing: Sharing::Public,
        native_type: app_definitions::User,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_expression_post: app_definitions::ExpressionPost, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [
        ]
    )
}

pub fn resonation_definition() -> ValidatingEntryType {
    entry!(
        name: "resonation",
        description: "Resonation Object Entry",
        sharing: Sharing::Public,
        native_type: app_definitions::User,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_resonation: app_definitions::Resonation, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [
        ]
    )
}