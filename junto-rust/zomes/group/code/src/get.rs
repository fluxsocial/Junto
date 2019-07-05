// use hdk::{
//     error::ZomeApiResult,
//     error::ZomeApiError,
//     holochain_core_types::{
//         cas::content::Address,
//         entry::Entry, 
//         json::JsonString,
//         link::LinkMatch
//     },
//     api::{
//         AGENT_ADDRESS, AGENT_ID_STR, CAPABILITY_REQ, DNA_ADDRESS, DNA_NAME
//     }
// };

// use std::convert::TryFrom;

// //Our modules for holochain actions
// use super::utils;
// use super::definitions::{
//     app_definitions,
//     function_definitions::{
//         FunctionDescriptor,
//         FunctionParameters,
//         UserDens,
//         EntryAndAddress,
//         CreateUserInformation,
//         Env,
//         JuntoUser
//     }
// };

// pub fn get_user_dens(user: Address) -> ZomeApiResult<UserDens>{
//     let den_links = utils::get_links_and_load_type::<app_definitions::Collection>(&user, LinkMatch::Exactly("collection"), LinkMatch::Exactly("den"))?;
//     let mut private_den = None;
//     let mut shared_den = None;
//     let mut public_den = None;
//     for den in den_links{
//         if den.entry.privacy == app_definitions::Privacy::Private{
//             private_den = Some(den.clone());
//         };
//         if den.entry.privacy == app_definitions::Privacy::Shared{
//             shared_den = Some(den.clone());
//         };
//         if den.entry.privacy == app_definitions::Privacy::Public{
//             public_den = Some(den.clone());
//         };
//     };
//     if private_den.is_none() == true{
//         return Err(ZomeApiError::from("User has no private den".to_string()))
//     } else if shared_den.is_none() == true{
//         return Err(ZomeApiError::from("User has no shared den".to_string()))
//     } else if public_den.is_none() == true{
//         return Err(ZomeApiError::from("User has no public den".to_string()))
//     };
//     Ok(UserDens{private_den: private_den.unwrap(), shared_den: shared_den.unwrap(), public_den: public_den.unwrap()})
// }

// pub fn get_user_pack(username_address: Address) -> ZomeApiResult<EntryAndAddress<app_definitions::Group>>{
//     let pack_links = utils::get_links_and_load_type::<app_definitions::Group>(&username_address, LinkMatch::Exactly("group"), LinkMatch::Exactly("pack"))?;
//     hdk::debug(format!("Pack links on username: {}", pack_links.len().to_string()))?;
//     if pack_links.len() > 1{
//         return Err(ZomeApiError::from("Pack links on user greater than 1".to_string()))
//     } else if pack_links.len() == 0{
//         return Err(ZomeApiError::from("No pack links on user".to_string()))
//     }
//     Ok(pack_links[0].clone())
// }

// pub fn get_user_member_packs(username_address: Address) -> ZomeApiResult<Vec<EntryAndAddress<app_definitions::Group>>>{
//     let pack_links = utils::get_links_and_load_type::<app_definitions::Group>(&username_address, LinkMatch::Exactly("auth"), LinkMatch::Exactly("member"))?;
//     let mut packs: Vec<EntryAndAddress<app_definitions::Group>> = vec![];
//     for pack in pack_links{
//         packs.push(pack.clone());
//     };
//     Ok(packs)
// }