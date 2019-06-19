use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        entry::Entry,
        dna::entry_types::Sharing
    }
};

use super::app_definitions;

pub fn anchor_definition() -> ValidatingEntryType {
    entry!(
        name: "anchor",
        description: "Entry which is a global index point - for example an entry which all users may link from to be indexable",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<app_definitions::Anchor>| {
            Ok(())
        },

        links: [            
            to!(
                "username",
                link_type: "registered",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "anchor",
                link_type: "bit_prefix", //bit_prefix config entry

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |validation_data: hdk::LinkValidationData| {
                    let bit_prefix_base_anchor = hdk::entry_address(&Entry::App("anchor".into(), app_definitions::Anchor{anchor_type: "bit_prefix".to_string()}.into()))?;
                    match validation_data{
                        hdk::LinkValidationData::LinkAdd{link, validation_data} => {
                            if *link.link.base() != bit_prefix_base_anchor{
                                Err("Base of link is not equal to bit prefix anchor".to_string())
                            } else {
                                // let provenances = validation_data.package.chain_header.provenances();
                                // hdk::debug(format!("Provenances: {:?}", provenances))?;
                                // let entry = hdk::get_entry(link.link.target())?.unwrap();
                                // hdk::debug(format!("Target entry: {:?}", entry))?;
                                // if provenances[0].source != pub_key_of_our_agent | hdk::verify_signature(provenances[0], entry){ //validate source against our agent
                                //     return Err("You are not allowed to make that link".to_string())
                                // };
                                Ok(())
                            }
                        },
                        hdk::LinkValidationData::LinkRemove{link, validation_data} =>{
                            if *link.link.base() != bit_prefix_base_anchor{
                                Err("Base of link is not equal to bit prefix anchor".to_string())
                            } else {
                                // let provenances = validation_data.package.chain_header.provenances();
                                // hdk::debug(format!("Provenances: {:?}", provenances))?;
                                // let entry = hdk::get_entry(link.link.target())?.unwrap();
                                // hdk::debug(format!("Target entry: {:?}", entry))?;
                                // if provenances[0].source != pub_key_of_our_agent | hdk::verify_signature(provenances[0], entry){ //validate source against our agent
                                //     return Err("You are not allowed to make that link".to_string())
                                // };
                                Ok(())
                            }
                        }
                    }
                }
            ),
            to!(
                "channel",
                link_type: "tag",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "channel",
                link_type: "expression_type",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "time",
                link_type: "time",

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