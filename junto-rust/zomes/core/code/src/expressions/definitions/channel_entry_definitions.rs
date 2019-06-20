use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing
    }
};

//Entry Definition(s)
use super::app_definitions;

pub fn channel_definition() -> ValidatingEntryType {
    entry!(
        name: "channel",
        description: "Channel Object Entry",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<app_definitions::Channel>| {
            Ok(())
        },

        links: [
            to!(
                "username",
                link_type: "auth", //link type which will handle all auth links e.g: owner, member etc

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "time",
                link_type: "time", //Link for channels which are being used as an anchor for users to store a collection of private/shared/public posts

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ), 
            to!( 
                "expression_post",
                link_type: "expression_post", //post to channel which is being used as an anchor for users to store a collection of private/shared/public posts

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "tag",
                link_type: "tag", 

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "tag",
                link_type: "expression_type",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
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