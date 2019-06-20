use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        entry::Entry,
        cas::content::Address
    }
};

use rust_base58::{FromBase58};
use multihash;

use super::definitions::app_definitions;

pub fn get_current_bit_prefix() -> ZomeApiResult<u32>{
    let bit_prefix_anchor = hdk::commit_entry(&Entry::App("anchor".into(), app_definitions::Anchor{anchor_type: String::from("bit_prefix")}.into()))?;
    let bit_prefixs = hdk::utils::get_links_and_load_type::<app_definitions::Config>(&bit_prefix_anchor, Some(String::from("bit_prefix")), None)?;
    Ok(bit_prefixs[0].value.parse::<u32>().unwrap())
}

pub fn update_bit_prefix(bit_prefix: u32) -> ZomeApiResult<u32>{
    let bit_prefix_anchor = hdk::commit_entry(&Entry::App("anchor".into(), app_definitions::Anchor{anchor_type: String::from("bit_prefix")}.into()))?;
    let bit_prefixs = hdk::get_links(&bit_prefix_anchor, Some(String::from("bit_prefix")), None)?.addresses();
    if bit_prefixs.len() > 0{
        hdk::remove_link(&bit_prefix_anchor, &bit_prefixs[0], "bit_prefix", "")?;
    };
    let bit_prefix_entry = Entry::App("config".into(), app_definitions::Config{value: bit_prefix.to_string(), config_type: "bit_prefix".to_string()}.into());
    let bit_prefix_address = hdk::commit_entry(&bit_prefix_entry)?;
    hdk::link_entries(&bit_prefix_anchor, &bit_prefix_address, "bit_prefix", "")?;
    Ok(bit_prefix)
}

//for now we use functions from holochain-collections directly in our app, HDK is unstable and holochain-collection may go out of date
pub fn hash_prefix(hash: Address, n_prefix_bits: u32) -> u32{
	// multi-hash encoding has a prefix which tells the hashing algorithm. We need to remove this or
	// everything will be put in the same bucket
	let multihash_bytes = String::from(hash).from_base58().unwrap();
	let bytes: &[u8] = multihash::decode(&multihash_bytes).unwrap().digest;

	// encode the bucket it as a 32 bit integer stringified. Not optimal but not terrible
	let mask: u32 = 2_u32.pow(n_prefix_bits) - 1;

	// println!("{:b}", mask);
	// println!("{:b} {:b}", bytes[1], bytes[0]);

	let id = u32::from_ne_bytes([
		bytes[0],
		bytes[1],
		bytes[2],
		bytes[3],
	]) & mask;

	// println!("{:b}", id);
	id
}