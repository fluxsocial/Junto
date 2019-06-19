use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        entry::Entry,
        dna::entry_types::Sharing
    }
};

use super::app_definitions;

pub fn config_definition() -> ValidatingEntryType {
    entry!(
        name: "config",
        description: "Config entry to be used by everyone in the application and only editable by our agent",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<app_definitions::Config>| {
            Ok(())
        },

        links: []
    )
}