//Entry Definition(s)
use super::app_definitions;

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing,
        cas::content::Address
    }
};

pub fn user_name_definition() -> ValidatingEntryType {
    entry!(
        name: "username",
        description: "Username Object Entry",
        sharing: Sharing::Public,
        native_type: app_definitions::UserName,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_username: app_definitions::UserName, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: []
    )
}

pub fn user_definition() -> ValidatingEntryType {
    entry!(
        name: "user",
        description: "User Object Entry",
        sharing: Sharing::Public,
        native_type: app_definitions::User,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_user: app_definitions::User, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [
            from!(
                "expression_post",
                tag: "owner",

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            from!(
                "group",
                tag: "member",

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            from!(
                "group",
                tag: "owner",

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            from!(
                "channel",
                tag: "user",

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            from!(
                "time",
                tag: "user",

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
                tag: "resonation",

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            to!(
                "channel",
                tag: "*", //Any link & Den

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            to!(
                "time",
                tag: "time",

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            to!(
                "group",
                tag: "pack",

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