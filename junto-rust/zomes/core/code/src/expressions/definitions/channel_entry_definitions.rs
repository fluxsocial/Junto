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
        native_type: app_definitions::Channel,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_channel: app_definitions::Channel, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [
            from!(
                "user",
                tag: "*",

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
                "time",
                tag: "*",

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            from!(
                "group",
                tag: "*",

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            from!(
                "resonation",
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
                tag: "*", //Any tag or expression tag

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            to!(
                "resonation",
                tag: "*", //Any tag or resonation tag

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            to!(
                "user",
                tag: "*", //Any tag or user tag

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
                "time",
                tag: "*", //Any tag or time tag

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