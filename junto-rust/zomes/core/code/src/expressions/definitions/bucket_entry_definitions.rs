use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing
    },
};

use super::app_definitions;

pub fn bucket_definition() -> ValidatingEntryType {
    entry!(
        name: "bucket",
        description: "Entry to be used as anchor for entries which need to be distributed across the hash space",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<app_definitions::Bucket>| {
            Ok(())
        },

        links: [
            to!(
                "expression_post",
                link_type: "expression_post",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}