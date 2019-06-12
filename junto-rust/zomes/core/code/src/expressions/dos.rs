use hdk::{
    error::ZomeApiResult,
    error::ZomeApiError,
    holochain_core_types::{
        cas::content::Address,
        entry::AppEntryValue
    }
};

use std::convert::TryFrom;
use multihash::{encode, Hash};

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

pub fn generate_random_number(min: f32, max: f32, seed: &String) -> u32{
    let seed_hash = encode(Hash::SHA2256, seed.as_bytes()).unwrap();
    let bytes: &[u8] = multihash::decode(&seed_hash).unwrap().digest;
    let mask: u32 = 2_u32.pow(14) - 1; //16383 - we shouldnt have to generate number outside of this bound
    let id = u32::from_ne_bytes([
		bytes[0],
		bytes[1],
		bytes[2],
		bytes[3],
	]) & mask;
    let id = id as f32 / mask as f32;
    ((id * (max - min + 1.0)).floor() + min) as u32
}

pub fn choose_pack_member(mut pack_members: Vec<Address>, depth: i32, avoid_addresses: &Vec<Address>, seed: &String) -> ZomeApiResult<Address>{
    pack_members.retain(|pack_member| avoid_addresses.contains(&pack_member) == false);
    if pack_members.len() == 0{
        Err(ZomeApiError::from(format!("Search exhasted at pack depth: {}", depth)))   
    } else{
        let random_index = generate_random_number(1.0, pack_members.len() as f32, seed) -1;
        hdk::debug(format!("Getting random pack member between bounds: {} and {}", 0.0, pack_members.len()-1))?;
        Ok(pack_members[usize::try_from(random_index).unwrap()].clone())
    }
}

pub fn get_packs_posts(pack_members: &Vec<Address>, query_string: String, post_addresses: &Vec<Address>, seed: &String) -> ZomeApiResult<Vec<Address>>{
    let mut out_posts = vec![];
    for pack_member in pack_members{
        let mut posts = hdk::api::get_links(pack_member, Some(String::from("expression_post")), Some(query_string.clone()))?.addresses(); //regex get_links query string here when supported 
        posts.retain(|post| post_addresses.contains(&post) == false);
        for ps in 1..USER_POST_SELECTION_COUNT{
            hdk::debug(format!("{}", posts.len()))?;
            if posts.len() != 0{
                hdk::debug(format!("Getting post between bounds: {} and {}. Post Selection: {}", 1.0, posts.len(), ps))?;
                let random_index = generate_random_number(1.0, posts.len() as f32, &(seed.clone()+&ps.to_string())) -1;
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
pub fn dos_query<T: TryFrom<AppEntryValue>>(query_string: String, _query_options: QueryOptions, target_type: QueryTarget, _query_type: QueryType, dos: i32, seed: String) -> ZomeApiResult<Vec<Address>>{
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
            hdk::debug(format!("Desired depth of: {} met, getting posts randomly from pack members in pack at current depth and recursion", depth))?;
            //get posts from all members in pack - then comapre amount retrieved and see if we need to do another recursion from first pack
            let mut posts = get_packs_posts(&pack_members, query_string.clone(), &post_addresses, &seed)?;
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