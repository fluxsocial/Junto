//Entry Definition(s)
use super::app_definitions;

pub mod user {
    //Definition for user expression object
    use super::app_definitions;

    use hdk::{
        entry_definition::ValidatingEntryType,
        holochain_core_types::{
            dna::entry_types::Sharing
        }
    };
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
            }
        )
    }
}