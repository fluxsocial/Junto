//Entry Definition(s)
use super::app_definitions;

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing
    }
};

pub fn group_definition() -> ValidatingEntryType {
    entry!(
        name: "group",
        description: "Group Object Entry",
        sharing: Sharing::Public,
        native_type: app_definitions::User,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_group: app_definitions::Group, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [
            from!(
                "user",
                tag: "pack", //Any tag or expression tag

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            from!(
                "user",
                tag: "member", //Any tag or expression tag

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            from!(
                "user",
                tag: "owner", //Any tag or expression tag

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            from!(
                "time",
                tag: "*", //Any tag 

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            to!(
                "expression_post",
                tag: "expression", 

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
                tag: "member", 

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            to!(
                "user",
                tag: "owner", 

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