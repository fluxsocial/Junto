//Entry Definition(s)
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
        },

        links: [
        ]
    )
}