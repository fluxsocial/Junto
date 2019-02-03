//Entry Definition(s)
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
        },

        links: [
        ]
    )
}