use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        entry::Entry,
        dna::entry_types::Sharing,
        validation::EntryValidationData,
    }
};

use super::app_definition;

pub fn config_definition() -> ValidatingEntryType {
    entry!(
        name: "config",
        description: "Config entry to be used by everyone in the application and only editable by our agent",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |validation_data: hdk::EntryValidationData<app_definition::Config>| {
            match validation_data{
                EntryValidationData::Create{entry: _entry, validation_data: _validation_data} =>
                {
                    // hdk::debug(format!("Sources: {:?}", validation_data.sources()))?;
                    // if validation_data.sources()[0] != "agent-hash"{
                    //     Err("Only the junto agent is allowed to create config entries")
                    // }
                    Ok(())
                },
                EntryValidationData::Modify{new_entry:_,old_entry:_,old_entry_header:_,validation_data:_} =>
                {
                    Ok(())
                },
                EntryValidationData::Delete{old_entry:_,old_entry_header:_,validation_data:_} =>
                {
                    Err("Not allowed to delete entry".to_string())
                }

            }
        },

        links: [
            from!(
                "anchor",
                link_type: "bit_prefix", //bit_prefix config entry

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |validation_data: hdk::LinkValidationData| {
                    let bit_prefix_base_anchor = hdk::entry_address(&Entry::App("anchor".into(), app_definition::Anchor{anchor_type: "bit_prefix".to_string()}.into()))?;
                    match validation_data{
                        hdk::LinkValidationData::LinkAdd{link, validation_data: _validation_data} => {
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
                        hdk::LinkValidationData::LinkRemove{link, validation_data: _validation_data} =>{
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
            )
        ]
    )
}