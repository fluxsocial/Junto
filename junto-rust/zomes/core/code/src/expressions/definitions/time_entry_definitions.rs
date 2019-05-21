//Entry Definition(s)
use super::app_definitions;

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing,
        cas::content::Address
    }
};

pub fn time_definiton() -> ValidatingEntryType {
    entry!(
        name: "time",
        description: "Time Object Entry",
        sharing: Sharing::Public,
        //native_type: app_definitions::Time,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |validation_data: hdk::EntryValidationData<app_definitions::Time>| {
            Ok(())
        },

        links: [
            from!(
                "group",
                link_type: "group_time", //Time entry in group to be used to associate group actions to given time entries

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "channel",
                link_type: "channel_time", //Link for channels which are being used as an anchor for users to store a collection of private/shared/public posts

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "expression_post",
                link_type: "time", //time entries which the expression is associated to

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "expression_post",
                link_type: "expression_post", //expression posts which are associated to this time

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "group",
                link_type: "group", //Link groups to time which they are created
    
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }            
            ),
            to!(
                "username",
                link_type: "user", //Link user to time which they are created

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