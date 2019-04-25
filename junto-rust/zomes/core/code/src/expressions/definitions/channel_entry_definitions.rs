//Entry Definition(s)
use super::app_definitions;

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing,
        cas::content::Address
    }
};

pub fn channel_definition() -> ValidatingEntryType {
    entry!(
        name: "channel",
        description: "Channel Object Entry",
        sharing: Sharing::Public,
        //native_type: app_definitions::Channel,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |validation_data: hdk::EntryValidationData<app_definitions::Channel>| {
            Ok(())
        },

        links: [
            from!(
                "username",
                tag: "den",

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "username",
                tag: "*",

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "channel",
                tag: "*",

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "time",
                tag: "*",

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "group",
                tag: "*",

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "resonation",
                tag: "*",

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "expression_post",
                tag: "*", //Any tag or expression tag

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "resonation",
                tag: "*", //Any tag or resonation tag

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "user",
                tag: "*", //Any tag or user tag

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "username",
                tag: "owner", //Any tag or user tag

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "channel",
                tag: "*",

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "time",
                tag: "*", //Any tag or time tag

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}