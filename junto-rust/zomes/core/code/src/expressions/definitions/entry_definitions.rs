//Entry Definition(s)
use super::app_definitions;

pub mod user {
    //Definition for user expression object
    use super::app_definitions;

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
            }
        )
    }
}

pub mod time {
    use super::app_definitions;

    use hdk::{
        entry_definition::ValidatingEntryType,
        holochain_core_types::{
            dna::entry_types::Sharing
        }
    };
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
            }
        )
    }
}

pub mod channel {
    use super::app_definitions;

    use hdk::{
        entry_definition::ValidatingEntryType,
        holochain_core_types::{
            dna::entry_types::Sharing
        }
    };
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
            }
        )
    }
}

pub mod group {
    use super::app_definitions;

    use hdk::{
        entry_definition::ValidatingEntryType,
        holochain_core_types::{
            dna::entry_types::Sharing
        }
    };
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
            }
        )
    }
}

pub mod post {
    use super::app_definitions;

    use hdk::{
        entry_definition::ValidatingEntryType,
        holochain_core_types::{
            dna::entry_types::Sharing
        }
    };
    pub fn post_definition() -> ValidatingEntryType {
        entry!(
            name: "post",
            description: "Post Object Entry",
            sharing: Sharing::Public,
            native_type: app_definitions::User,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: |_post: app_definitions::Post, _ctx: hdk::ValidationData| {
                Ok(())
            }
        )
    }
}

pub mod resonation {
    use super::app_definitions;

    use hdk::{
        entry_definition::ValidatingEntryType,
        holochain_core_types::{
            dna::entry_types::Sharing
        }
    };
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
            }
        )
    }
}