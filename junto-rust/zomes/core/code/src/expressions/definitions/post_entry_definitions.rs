//Entry Definition(s)
use super::app_definitions;

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing,
        cas::content::Address
    }
};

pub fn post_definition() -> ValidatingEntryType {
    entry!(
        name: "expression_post",
        description: "ExpressionPost Object Entry",
        sharing: Sharing::Public,
        //native_type: app_definitions::ExpressionPost,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |validation_data: hdk::EntryValidationData<app_definitions::ExpressionPost>| {
            Ok(())
        },

        links: [
            from!(
                "user",
                tag: "expression",

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "channel",
                tag: "*", //Any tag or expression tag

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "resonation",
                tag: "*", //Any tag or expression tag

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "time",
                tag: "*", //Any tag or expression tag

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "group",
                tag: "expression",

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
            ),
            to!(
                "expression_post",
                tag: "comment", 

                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "resonation",
                tag: "*", //Any tag to help make resonation colour searchable or just resonation tag 

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

// pub fn resonation_definition() -> ValidatingEntryType {
//     entry!(
//         name: "resonation",
//         description: "Resonation Object Entry",
//         sharing: Sharing::Public,
//         native_type: app_definitions::Resonation,
//         validation_package: || {
//             hdk::ValidationPackageDefinition::Entry
//         },

//         validation: |_resonation: app_definitions::Resonation, _ctx: hdk::ValidationData| {
//             Ok(())
//         },

//         links: [
//             from!(
//                 "expression_post",
//                 tag: "*", //Either any tag containing resonation colour/search query through expression or just resonation tag

//                 validation_package: || {
//                     hdk::ValidationPackageDefinition::ChainFull
//                 },

//                 validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
//                     Ok(())
//                 }
//             ),
//             from!(
//                 "channel",
//                 tag: "*", //Either any tag containing resonation colour/search query through expression or just resonation tag

//                 validation_package: || {
//                     hdk::ValidationPackageDefinition::ChainFull
//                 },

//                 validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
//                     Ok(())
//                 }
//             ),
//             from!(
//                 "user",
//                 tag: "*",//Either any tag containing resonation colour/search query through expression or just resonation tag

//                 validation_package: || {
//                     hdk::ValidationPackageDefinition::ChainFull
//                 },

//                 validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
//                     Ok(())
//                 }
//             ),
//             from!(
//                 "group",
//                 tag: "*",//Either any tag containing resonation colour/search query through expression or just resonation tag

//                 validation_package: || {
//                     hdk::ValidationPackageDefinition::ChainFull
//                 },

//                 validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
//                     Ok(())
//                 }
//             ),
//             from!(
//                 "time",
//                 tag: "*",//Either any tag containing resonation colour/search query through expression or just resonation tag

//                 validation_package: || {
//                     hdk::ValidationPackageDefinition::ChainFull
//                 },

//                 validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
//                     Ok(())
//                 }
//             ),
//             to!(
//                 "expression_post",
//                 tag: "expression", 

//                 validation_package: || {
//                     hdk::ValidationPackageDefinition::ChainFull
//                 },

//                 validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
//                     Ok(())
//                 }
//             ),
//             to!(
//                 "channel",
//                 tag: "*", //Any tag to provide searchable trees 

//                 validation_package: || {
//                     hdk::ValidationPackageDefinition::ChainFull
//                 },

//                 validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
//                     Ok(())
//                 }
//             )
//         ]
//     )
// }