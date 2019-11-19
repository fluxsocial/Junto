use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing
    },
};

//Entry Definition(s)
use super::app_definition;

pub fn perspective_definition() -> ValidatingEntryType {
    entry!(
        name: "perspective",
        description: "Perspective Object Entry",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<app_definition::Perspective>| {
            Ok(())
        },

        links: [
            to!(
                "username",
                link_type: "user_perspective", //link to a user who is part of a given perspective

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