use hdk::{
    holochain_core_types::{
        cas::content::Address,
        entry::Entry, 
        json::JsonString
    }
};

//Our modules for holochain actins
use super::definitions;
use super::utils;
use super::channel;

//Function to handle the posting of an expression - will link to any specified channels and insert into relevant groups/packs
pub fn handle_post_expression(expression: definitions::app_definitions::ExpressionPost, channels: Vec<String>) -> JsonString{
    let entry = Entry::App("expression_post".into(), expression.into());
    match hdk::commit_entry(&entry){
        Ok(address) => {},
        Err(hdk_err) => return json!({"error": hdk_err}).into()
    };
    //get user den
    //get user pack
    //get packs a user is a part of
    //check that channel(s) exist in each of the above expression locals
    //if not create needed channels 
    //check for time in each expression locals
    //create contextual links of times & channels in each expression locals & any other queryable link structures which may want to be made such as username
    json!({"message": "ok"}).into()
}

//Function to handle the resonation of an expression post - will put the post into packs which the post should be resonated into
pub fn handle_resonation(expression: Address, resonation: definitions::app_definitions::Resonation) -> JsonString{
    json!({"message": "ok"}).into()
}

//Function to handle the getting of expression with a given query root and query string
//for example: query_root: Channel: Technology, query_string: Timestamp<2018>:Channel<holochain>:Channel<dht>:User<Eric>
//this would search for all posts in the channel Technology, which where posted in 2018 and also contain the channels Holochain & Dht by the user Eric
pub fn get_expression(query_root: Address, query_string: String) -> JsonString{
    json!({"message": "ok"}).into()
}