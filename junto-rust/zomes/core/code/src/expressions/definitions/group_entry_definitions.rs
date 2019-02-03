//Entry Definition(s)
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
        },

        links: [
        ]
    )
}