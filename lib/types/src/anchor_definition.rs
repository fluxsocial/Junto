use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing
    },
    holochain_json_api::{
        json::JsonString
    }
};

use super::app_definition;

pub fn anchor_definition() -> ValidatingEntryType {
    entry!(
        name: "anchor",
        description: "Entry which is a global index point - for example an entry which all users may link from to be indexable",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<app_definition::Anchor>| {
            Ok(())
        },

        links: []
    )
}