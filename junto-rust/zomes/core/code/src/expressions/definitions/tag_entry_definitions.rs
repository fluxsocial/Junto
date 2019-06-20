use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        entry::Entry,
        dna::entry_types::Sharing
    }
};

use super::app_definitions;

pub fn tag_definition() -> ValidatingEntryType {
    entry!(
        name: "tag",
        description: "Attribute of a expression post, can include expression type",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<app_definitions::Tag>| {
            Ok(())
        },

        links: []
    )
}