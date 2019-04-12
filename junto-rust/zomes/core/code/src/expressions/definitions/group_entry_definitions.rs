//Entry Definition(s)
use super::app_definitions;

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing,
        cas::content::Address
    }
};

pub fn group_definition() -> ValidatingEntryType {
    entry!(
        name: "group",
        description: "Group Object Entry",
        sharing: Sharing::Public,
        //native_type: app_definitions::Group,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |validation_data: hdk::EntryValidationData<app_definitions::Group>| {
            Ok(())
        },

        links: [
            from!(
                "user",
                tag: "pack", //Any tag or expression tag

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "user",
                tag: "member", //Any tag or expression tag

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "user",
                tag: "owner", //Any tag or expression tag

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "time",
                tag: "*", //Any tag 

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "expression_post",
                tag: "expression", 

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
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
                tag: "*", 

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "user",
                tag: "member", 

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "user",
                tag: "owner", 

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