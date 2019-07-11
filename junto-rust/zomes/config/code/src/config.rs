use hdk::{
    error::{
        ZomeApiResult
    },
    holochain_core_types::{
        link::LinkMatch,
        entry::Entry
    }
};

use types::app_definition;

pub fn get_current_bit_prefix() -> ZomeApiResult<u32>{
    let bit_prefix_anchor = hdk::commit_entry(&Entry::App("anchor".into(), app_definition::Anchor{anchor_type: String::from("bit_prefix")}.into()))?;
    let bit_prefixs = hdk::utils::get_links_and_load_type::<app_definition::Config>(&bit_prefix_anchor, LinkMatch::Exactly("bit_prefix"), LinkMatch::Any)?;
    Ok(bit_prefixs[0].value.parse::<u32>().unwrap())
}

pub fn update_bit_prefix(bit_prefix: u32) -> ZomeApiResult<u32>{
    let bit_prefix_anchor = hdk::commit_entry(&Entry::App("anchor".into(), app_definition::Anchor{anchor_type: String::from("bit_prefix")}.into()))?;
    let bit_prefixs = hdk::get_links(&bit_prefix_anchor, LinkMatch::Exactly("bit_prefix"), LinkMatch::Any)?.addresses();
    if bit_prefixs.len() > 0{
        hdk::remove_link(&bit_prefix_anchor, &bit_prefixs[0], "bit_prefix", "")?;
    };
    let bit_prefix_entry = Entry::App("config".into(), app_definition::Config{value: bit_prefix.to_string(), config_type: "bit_prefix".to_string()}.into());
    let bit_prefix_address = hdk::commit_entry(&bit_prefix_entry)?;
    hdk::link_entries(&bit_prefix_anchor, &bit_prefix_address, "bit_prefix", "")?;
    Ok(bit_prefix)
}
