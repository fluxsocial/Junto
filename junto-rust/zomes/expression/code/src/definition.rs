use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing
    },
    holochain_json_api::{
        json::JsonString
    }
};

use types::app_definition;

pub fn post_definition() -> ValidatingEntryType {
    entry!(
        name: "expression_post",
        description: "ExpressionPost Object Entry",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<app_definition::ExpressionPost>| {
            Ok(())
        },

        links: [
            to!(
                "attribute",
                link_type: "channels", 

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "attribute",
                link_type: "expression_type", //type which expression is

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "attribute",
                link_type: "created_at", //time entries which the expression is associated to

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "username",
                link_type: "auth", //links types which will contain auth information of a given post: example: owner, co-writer etc

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "expression_post",
                link_type: "related", //used for mapping other expressions to current expression: example: comments

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "expression_post",
                link_type: "sub_expression", //Links parent expression to its sub

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "expression_post",
                link_type: "parent_expression", //Links sub expression to its parent

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "username",
                link_type: "resonation",

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

pub fn bucket_definition() -> ValidatingEntryType {
    entry!(
        name: "bucket",
        description: "Entry to be used as anchor for entries which need to be distributed across the hash space",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<app_definition::Bucket>| {
            Ok(())
        },

        links: [
            to!(
                "expression_post",
                link_type: "expression_post",

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