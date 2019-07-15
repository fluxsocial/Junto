use hdk::{
    error::{
        ZomeApiResult,
        ZomeApiError
    },
    holochain_core_types::{
        link::LinkMatch
    },
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_json_api::{
        json::JsonString
    }
};

use std::convert::TryFrom;
use std::convert::TryInto;

//Our modules for holochain actins
use types::{
    app_definition,
    function_definition::{
        QueryOptions,
        QueryType,
        EntryAndAddress
    }
};
use utils;

const MAXIMUM_PACK_RECURSIONS: i32 = 50;
const MAXIMUM_USER_RECURSIONS: i32 = 50;
const DESIRED_POST_COUNT: i32 = 50;
const USER_POST_SELECTION_COUNT: i32 = 5;

pub fn choose_pack_member(mut pack_members: Vec<Address>, depth: u32, avoid_addresses: &Vec<Address>, seed: &String) -> ZomeApiResult<Address>{
    pack_members.retain(|pack_member| avoid_addresses.contains(&pack_member) == false);
    if pack_members.len() == 0{
        Err(ZomeApiError::from(format!("Search exhasted at pack depth: {}", depth)))   
    } else{
        let random_index = utils::helpers::generate_random_number(1.0, pack_members.len() as f32, seed) -1;
        hdk::debug(format!("Getting random pack member between bounds: {} and {}", 0.0, pack_members.len()-1))?;
        Ok(pack_members[usize::try_from(random_index).unwrap()].clone())
    }
}

pub fn get_packs_posts(pack_members: &Vec<Address>, index_strings: &Vec<String>, post_addresses: &Vec<Address>, seed: &String, resonations: bool) -> ZomeApiResult<Vec<Address>>{
    let mut out_posts = vec![];
    for pack_member in pack_members{
        let mut posts = vec![];
        for index_string in index_strings{
            if resonations == true{
                posts.append(&mut hdk::api::get_links(pack_member, LinkMatch::Exactly("resonation"), LinkMatch::Regex(index_string.as_str()))?.addresses());
            } else {
                posts.append(&mut hdk::api::get_links(pack_member, LinkMatch::Exactly("expression_post"), LinkMatch::Regex(index_string.as_str()))?.addresses());
            };
        };
        posts.retain(|post| post_addresses.contains(&post) == false);
        for ps in 1..USER_POST_SELECTION_COUNT{
            hdk::debug(format!("{}", posts.len()))?;
            if posts.len() != 0{
                hdk::debug(format!("Getting post between bounds: {} and {}. Post Selection: {}", 1.0, posts.len(), ps))?;
                let random_index = utils::helpers::generate_random_number(1.0, posts.len() as f32, &(seed.clone()+&ps.to_string())) -1;
                hdk::debug(format!("Generated number: {}", random_index))?;
                out_posts.push(posts[usize::try_from(random_index).unwrap()].clone());
                posts.remove_item(&posts[usize::try_from(random_index).unwrap()].clone());
            } else {
                break;
            };
        };
    };
    Ok(out_posts)
}

//TODO build dos post query and user query into seperate functions which are handled and called by dos_query
//Currently this algorithm will iterate until either all searches are exhasted from each pack recursion tree or 50 posts are found or user/pack recursions have reached their max - then the loop will break and return whatever posts it has
pub fn dos_query(index_strings: Vec<String>, _query_options: QueryOptions, _query_type: QueryType, dos: u32, seed: String, resonations: bool) -> ZomeApiResult<Vec<Address>>{
    let mut avoid_addresses = vec![];
    let mut post_addresses = vec![];
    let mut users_checked_count = 0;
    let mut packs_traversed_count = 0;
    let mut depth = 0;
    let mut has_new_path: bool;
    let current_agent_username = utils::helpers::call_and_get_current_user_username()?;

    let users_pack = hdk::call(hdk::THIS_INSTANCE, "group", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                "get_user_pack", JsonString::from(json!({"username_address": current_agent_username.address})))?;
    let users_pack: ZomeApiResult<EntryAndAddress<app_definition::Group>> = users_pack.try_into()?;
    let users_pack: EntryAndAddress<app_definition::Group> = users_pack?;

    let mut pack_members = hdk::get_links(&users_pack.address, LinkMatch::Exactly("group_auth"), LinkMatch::Exactly("member"))?.addresses();
    
    if pack_members.len() == 0 {return Err(ZomeApiError::from("You have no pack members and thus cannot make degree of seperation query".to_string()))};
    let mut pack_recursions = vec![];
    for _ in 0..dos-1{pack_recursions.push(vec![]);};
    pack_recursions.push(pack_members.clone());
    hdk::debug(format!("Pack recursion vector before loop starts: {:?}", pack_recursions))?;
    
    loop {
        hdk::debug(format!("At depth: {}", depth))?;
        if (post_addresses.len() as i32 >= DESIRED_POST_COUNT) | (users_checked_count >= MAXIMUM_USER_RECURSIONS) | (packs_traversed_count >= MAXIMUM_PACK_RECURSIONS) {
            hdk::debug("Post or recursion limit has been reached - breaking loop")?;
            break;
        };
        if depth != dos-1{
            match choose_pack_member(pack_members.clone(), depth, &avoid_addresses, &seed){
                Ok(pack_member) => {
                    hdk::debug(format!("Pack member choosen at depth: {}", depth))?;
                    avoid_addresses.push(pack_member.clone());
                    let recursions_pack = hdk::call(hdk::THIS_INSTANCE, "group", Address::from(hdk::PUBLIC_TOKEN.to_string()), 
                                "get_user_pack", JsonString::from(json!({"username_address": pack_member})))?;
                    let recursions_pack: ZomeApiResult<EntryAndAddress<app_definition::Group>> = recursions_pack.try_into()?;
                    let recursions_pack: EntryAndAddress<app_definition::Group> = recursions_pack?;

                    pack_members = hdk::get_links(&recursions_pack.address, LinkMatch::Exactly("group_auth"), LinkMatch::Exactly("member"))?.addresses();
                    packs_traversed_count += 1;
                    depth += 1;
                    pack_recursions[depth as usize].append(&mut pack_members);
                },
                Err(_err) => {
                    has_new_path = false;
                    for r in 0..dos-1 { //iterate over pack_recurstions and remove any already visited addresses
                        pack_recursions[r as usize].retain(|address| avoid_addresses.contains(&address) == false);
                        
                        if pack_recursions[r as usize].len() != 0{
                            depth = r;
                            pack_members = pack_recursions[r as usize].clone();
                            hdk::debug(format!("Pack recurstion couldn't reach {}. Restarting recursion from from depth {}, avoiding addresses which have already been used.", dos, depth))?;
                            has_new_path = true;
                            break;
                        };
                    };
                    if has_new_path == false{
                        return Err(ZomeApiError::from(format!("Cannot reach depth of: {}", dos)))
                    };
                }
            };
        } else {
            hdk::debug(format!("Desired depth of: {} met, getting posts randomly from pack members in pack at current depth and recursion", depth))?;
            //get posts from all members in pack - then comapre amount retrieved and see if we need to do another recursion from first pack
            let mut posts = get_packs_posts(&pack_members, &index_strings, &post_addresses, &seed, resonations)?;
            users_checked_count += pack_members.len() as i32;
            post_addresses.append(&mut posts);
            hdk::debug(format!("Number of users checked: {}, number of total users checked: {}", pack_members.len(), users_checked_count))?;
            hdk::debug(format!("Number of posts found: {}, total number of posts: {}", posts.len(), post_addresses.len()))?;
            has_new_path = false;
            for r in 0..dos-1 { //iterate over pack_recurstions and remove any already visited addresses
                pack_recursions[r as usize].retain(|address| avoid_addresses.contains(&address) == false);
                
                if pack_recursions[r as usize].len() != 0{
                    depth = r;
                    pack_members = pack_recursions[r as usize].clone();
                    hdk::debug(format!("Pack recurstion couldn't reach {}. Restarting recursion from from depth {}, avoiding addresses which have already been used.", dos, depth))?;
                    has_new_path = true;
                    break;
                };
            };
            if has_new_path == false{
                break;
            };
            hdk::debug(format!("Checking if there are any more pack searches possible - if not then loop break - otherwise it will restart from depth {}. Posts retreived so far: {}", dos, post_addresses.len()))?;
        };
    };
    Ok(post_addresses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn choose_pack_member_test(){
        assert_eq!(1, 2)
    }

    #[test]
    fn get_packs_posts_test(){
        assert_eq!(1, 1)
    }

    #[test]
    fn dos_query_test(){
        assert_eq!(1, 1)
    }
}