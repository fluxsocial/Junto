use hdk::{
    error::{
        ZomeApiResult,
        ZomeApiError
    },
    holochain_core_types::{
        cas::content::Address, 
        hash::HashString,
        json::{
            JsonString,
            default_to_json
        },
        error::HolochainError,
        dna::capabilities::CapabilityRequest
    }
};

use std::collections::HashMap;
use serde::Serialize;
use std::fmt::Debug;

use super::app_definitions;

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Env {
    pub dna_name: String,
    pub dna_address: String,
    pub agent_id: String,
    pub agent_address: String,
    pub cap_request: CapabilityRequest,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, DefaultJson)]
pub struct CreateUserInformation{
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture: String,
    pub bio: String
}

//Basic struct to be used to describe a function and its parameters to the handle_hooks function
pub struct FunctionDescriptor{  
    pub name: &'static str,
    pub parameters: FunctionParameters,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct UserDens{
    pub private_den: EntryAndAddress<app_definitions::Collection>,
    pub shared_den: EntryAndAddress<app_definitions::Collection>,
    pub public_den: EntryAndAddress<app_definitions::Collection>,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct JuntoUser{
    pub private_den: EntryAndAddress<app_definitions::Collection>,
    pub shared_den: EntryAndAddress<app_definitions::Collection>,
    pub public_den: EntryAndAddress<app_definitions::Collection>,
    pub pack: EntryAndAddress<app_definitions::Group>,
    pub profile: EntryAndAddress<app_definitions::User>,
    pub username: EntryAndAddress<app_definitions::UserName>,
    pub user_perspective: EntryAndAddress<app_definitions::Perspective>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupMembers{
    pub members: Vec<EntryAndAddress<app_definitions::UserName>>
}

#[derive(Clone)]
pub enum HooksResultTypes{
    TimeToExpression(Vec<Address>),
    CreatePack(EntryAndAddress<app_definitions::Group>),
    CreateDen(UserDens),
    LinkExpression(String),
    CreatePostIndex(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QueryTarget{
    ExpressionPost,
    User
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QueryOptions {
    FilterPopular,
    FilterNew,
    FilterOld
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QueryType {
    And,
    Or
}

pub type EntryAndAddressResult<T> = Vec<EntryAndAddress<T>>;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash)]
pub struct EntryAndAddress<T>{
	pub address: HashString,
	pub entry: T
}

impl HooksResultTypes{
    // pub fn time_to_expression_result(self) -> ZomeApiResult<Vec<Address>> {
    //     match self {
    //         HooksResultTypes::TimeToExpression(r) => Ok(r),
    //         _ => Err(ZomeApiError::from("Hook result enum value not: TimeToExpression".to_string())),
    //     }
    // }
    pub fn create_pack_result(self) -> ZomeApiResult<EntryAndAddress<app_definitions::Group>> {
        match self {
            HooksResultTypes::CreatePack(r) => Ok(r),
            _ => Err(ZomeApiError::from("Hook result enum value not: CreatePack".to_string())),
        }
    }
    pub fn create_den_result(self) -> ZomeApiResult<UserDens> {
        match self {
            HooksResultTypes::CreateDen(r) => Ok(r),
            _ => Err(ZomeApiError::from("Hook result enum value not: CreateDen".to_string())),
        }
    }
    // pub fn link_expression_result(self) -> ZomeApiResult<String> {
    //     match self {
    //         HooksResultTypes::LinkExpression(r) => Ok(r),
    //         _ => Err(ZomeApiError::from("Hook result enum value not: LinkExpression".to_string())),
    //     }
    // }
    // pub fn create_post_index_result(self) -> ZomeApiResult<String> {
    //     match self {
    //         HooksResultTypes::CreateQueryPoints(r) => Ok(r),
    //         _ => Err(ZomeApiError::from("Hook result enum value not: CreateQueryPoints".to_string())),
    //     }
    // }
}

impl<T> PartialEq for EntryAndAddress<T>{
    fn eq(self: &Self, other: &EntryAndAddress<T>) -> bool {
        self.address == other.address
    }
}

impl From<GroupMembers> for JsonString {
    fn from(result: GroupMembers) -> JsonString {
        JsonString::from(json!(default_to_json(result)))
    }
}

impl<T: Into<JsonString>> From<EntryAndAddress<T>> for JsonString  where T: Serialize + Debug{
    fn from(result: EntryAndAddress<T>) -> JsonString {
        JsonString::from(default_to_json(result))
    }
}

//Parameters for each function in holochain application
pub enum FunctionParameters{
    TimeToExpression{
        link_type: String,
        tag: String, 
        direction: String, 
        expression_address: Address
    },
    CreatePack{
        username_address: Address,
        first_name: String
    },
    CreateDen{
        username_address: Address,
        first_name: String
    },
    LinkExpression{
        link_type: String,
        tag: String, 
        direction: String, 
        parent_expression: Address, 
        child_expression: Address
    },
    CreatePostIndex{
        indexes: Vec<HashMap<String, String>>, 
        context: Address, 
        expression: Address,
        index_string: String,
        link_type: String
    }
}
