use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing
    }
};

use super::app_definitions;

pub fn user_name_definition() -> ValidatingEntryType {
    entry!(
        name: "username",
        description: "Username Object Entry",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<app_definitions::UserName>| {
            Ok(())
        },

        links: [
            from!(
                "%agent_id",
                link_type: "username", //links username object to agent_id

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "attribute",
                link_type: "created_at", //Link user to time which they are created

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "username",
                link_type: "follower", //Occurs when user add another user to one of their perspective and thus are "following" their posts - name due to change

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "username",
                link_type: "following", //Occurs when user add another user to one of their perspective and thus are "following" their posts - name due to change

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "group",
                link_type: "group", //Link type to associate a group with a user - tag can then define group type; in our case/implementation: pack

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "group",
                link_type: "auth", //link type which will handle all auth links e.g: owner, member etc

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "expression_post",
                link_type: "expression_post", //users posts links

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "user",
                link_type: "profile",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!( 
                "collection",
                link_type: "collection", //Link type to associate a collection with a user - tag can then define collection type; in our case/implementation: den

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "perspective",
                link_type: "perspective", //link to a users perspective

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

pub fn user_definition() -> ValidatingEntryType {
    entry!(
        name: "user",
        description: "User Metadata Object Entry",
        sharing: Sharing::Public,
        //native_type: app_definitions::User,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<app_definitions::User>| {
            Ok(())
        },

        links: [
            from!(
                "%agent_id",
                link_type: "user",

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