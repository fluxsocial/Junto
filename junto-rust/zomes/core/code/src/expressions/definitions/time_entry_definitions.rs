use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing
    }
};

use super::app_definitions;

pub fn time_definiton() -> ValidatingEntryType {
    entry!(
        name: "time",
        description: "Time Object Entry",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<app_definitions::Time>| {
            Ok(())
        },

        links: []
    )
}