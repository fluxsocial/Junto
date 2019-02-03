//Entry Definition(s)
use super::app_definitions;
use super::expression_links;
use super::expression_validation;

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing
    }
};

pub fn user_definition() -> ValidatingEntryType {
    entry!(
        name: "user",
        description: "User Object Entry",
        sharing: Sharing::Public,
        native_type: app_definitions::User,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_user: app_definitions::User, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [
        ]
    )
}

pub fn time_definiton() -> ValidatingEntryType {
    entry!(
        name: "time",
        description: "Time Object Entry",
        sharing: Sharing::Public,
        native_type: app_definitions::User,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_time: app_definitions::Time, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [
        ]
    )
}

pub fn channel_definition() -> ValidatingEntryType {
    entry!(
        name: "channel",
        description: "Channel Object Entry",
        sharing: Sharing::Public,
        native_type: app_definitions::User,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_channel: app_definitions::Channel, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [
        ]
    )
}

pub fn group_definition() -> ValidatingEntryType {
    entry!(
        name: "group",
        description: "Group Object Entry",
        sharing: Sharing::Public,
        native_type: app_definitions::User,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_group: app_definitions::Group, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [
        ]
    )
}

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