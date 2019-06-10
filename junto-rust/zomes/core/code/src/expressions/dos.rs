use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        cas::content::Address,
        entry::AppEntryValue
    }
};

use std::convert::TryFrom;
use rand::Rng;

//Our modules for holochain actins
use super::definitions::{
    function_definitions::{
        QueryTarget,
        QueryOptions,
        QueryType
    }
};

use super::user;

const MAXIMUM_PACK_RECURSIONS: i32 = 50;
const MAXIMUM_USER_RECURSIONS: i32 = 50;
const DESIRED_POST_COUNT: i32 = 50;
const USER_POST_SELECTION_COUNT: i32 = 5;

pub fn choose_pack_member(mut pack_members: Vec<Address>, depth: i32, avoid_addresses: &Vec<Address>) -> ZomeApiResult<Address>{
    let mut rng = rand::thread_rng();
    pack_members.retain(|pack_member| avoid_addresses.contains(&pack_member) == false);
    let random_index = rng.gen_range(0, pack_members.len()-1);
    if pack_members.len() == 0{
        Err(ZomeApiError::from(format!("Search exhasted at pack depth: {}", depth)))   
    } else{
        Ok(pack_members[random_index].clone())
    }
}

pub fn get_packs_posts(pack_members: &Vec<Address>, query_string: String, post_addresses: &Vec<Address>) -> ZomeApiResult<Vec<Address>>{
    let mut rng = rand::thread_rng();
    let mut out_posts = vec![];
    for pack_member in pack_members{
        let mut posts = hdk::api::get_links(pack_member, Some(String::from("expression_post")), Some(query_string.clone()))?.addresses(); //regex get_links query string here when supported 
        posts.retain(|post| post_addresses.contains(&post) == false);
        for _ in 1..USER_POST_SELECTION_COUNT{
            if posts.len() != 0{
                let random_index = rng.gen_range(0, posts.len()-1);
                out_posts.push(posts[random_index].clone());
                posts.remove_item(&posts[random_index].clone());
            };
        };
    };
    Ok(out_posts)
}

pub fn dos_query<T: TryFrom<AppEntryValue>>(query_string: String, _query_options: QueryOptions, target_type: QueryTarget, _query_type: QueryType, dos: i32) -> ZomeApiResult<Vec<Address>>{
    let mut avoid_addresses = vec![];
    let mut post_addresses = vec![];
    let mut users_checked_count = 0;
    let mut packs_traversed_count = 0;
    let mut depth = 0;
    let mut has_new_path: bool;
    let user_username = user::get_user_username_by_agent_address()?;
    let users_pack = user::get_user_pack(user_username.address)?;
    let mut pack_members = hdk::get_links(&users_pack.address, Some(String::from("auth")), Some(String::from("member")))?.addresses();
    
    if pack_members.len() == 0 {return Err(ZomeApiError::from("You have no pack members and thus cannot make degree of seperation query".to_string()))};
    let mut pack_recursions = vec![];
    for _ in 0..dos-1{pack_recursions.push(vec![]);};
    pack_recursions[0].append(&mut pack_members);
    
    loop {
        if (post_addresses.len() as i32 >= DESIRED_POST_COUNT) | (users_checked_count >= MAXIMUM_USER_RECURSIONS) | (packs_traversed_count >= MAXIMUM_PACK_RECURSIONS) {
            break;
        };
        if depth != dos-1{
            match choose_pack_member(pack_members.clone(), depth, &avoid_addresses){
                Ok(pack_member) => {
                    avoid_addresses.push(pack_member.clone());
                    let recursions_pack = user::get_user_pack(pack_member)?;
                    pack_members = hdk::get_links(&recursions_pack.address, Some(String::from("auth")), Some(String::from("member")))?.addresses();
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
            //get posts from all members in pack - then comapre amount retrieved and see if we need to do another recursion from first pack
            let mut posts = get_packs_posts(&pack_members, query_string.clone(), &post_addresses)?;
            users_checked_count += pack_members.len() as i32;
            post_addresses.append(&mut posts);
            depth = 0;
            hdk::debug(format!("Desired number of posts not reached at depth of: {} restarting recursion from depth 0, avoiding addresses which have already been used in pack hops, current amount of posts retrieved: {}", dos, post_addresses.len()))?;
        };
    };
    Ok(post_addresses)
}

//build dos post query and user query into seperate functions which are handled and called by dos_query

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