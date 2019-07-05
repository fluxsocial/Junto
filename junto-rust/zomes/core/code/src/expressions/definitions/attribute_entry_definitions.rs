use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing
    }
};

use crate::app_definition;;

pub fn attribute_definition() -> ValidatingEntryType {
    entry!(
        name: "attribute",
        description: "Attribute of an expression, can include expression type, channel and time",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<app_definitions::Attribute>| {
            Ok(())
        },

        links: []
    )
}