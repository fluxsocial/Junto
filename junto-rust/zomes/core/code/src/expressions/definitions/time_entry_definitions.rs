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
        native_type: app_definitions::Time,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_time: app_definitions::Time, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [
            from!(
                "group",
                tag: "time", 

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            from!(
                "user",
                tag: "time",

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            from!(
                "channel",
                tag: "*",

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            from!(
                "expression_post",
                tag: "*", 

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            to!(
                "user",
                tag: "user", 

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            to!(
                "channel",
                tag: "*", 

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            to!(
                "expression_post",
                tag: "*", 

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            to!(
                "resonation",
                tag: "*", 

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            )
        ]
    )
}