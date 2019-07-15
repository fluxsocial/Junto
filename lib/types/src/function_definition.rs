use hdk::{
    error::{
        ZomeApiResult,
        ZomeApiError
    },
    holochain_core_types::{
        dna::capabilities::CapabilityRequest
    },
    holochain_persistence_api::{
        cas::content::Address,
    },
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
        json::default_to_json
    }
};
use holochain_json_derive::{ 
    DefaultJson 
};

use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

use super::app_definition;

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Env {
    pub dna_name: String,
    pub dna_address: String,
    pub agent_id: String,
    pub agent_address: String,
    pub cap_request: Option<CapabilityRequest>
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
    pub private_den: EntryAndAddress<app_definition::Collection>,
    pub shared_den: EntryAndAddress<app_definition::Collection>,
    pub public_den: EntryAndAddress<app_definition::Collection>,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct JuntoUser{
    pub private_den: EntryAndAddress<app_definition::Collection>,
    pub shared_den: EntryAndAddress<app_definition::Collection>,
    pub public_den: EntryAndAddress<app_definition::Collection>,
    pub pack: EntryAndAddress<app_definition::Group>,
    pub profile: EntryAndAddress<app_definition::User>,
    pub username: EntryAndAddress<app_definition::UserName>,
    pub user_perspective: EntryAndAddress<app_definition::Perspective>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupMembers{
    pub members: Vec<EntryAndAddress<app_definition::UserName>>
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct ExpressionData{
    pub expression: EntryAndAddress<app_definition::ExpressionPost>,
    pub sub_expressions: Vec<ExpressionData>,
    pub author_username: EntryAndAddress<app_definition::UserName>,
    pub author_profile: EntryAndAddress<app_definition::User>,
    pub resonations: Vec<EntryAndAddress<app_definition::UserName>>,
    pub timestamp: String,
    pub channels: Vec<EntryAndAddress<app_definition::Attribute>>
}

#[derive(Clone)]
pub enum HooksResultTypes{
    CreatePack(EntryAndAddress<app_definition::Group>),
    CreateDen(UserDens)
}

#[derive(Debug, Serialize, Deserialize, DefaultJson)]
pub enum QueryTarget{
    ExpressionPost,
    User
}

#[derive(Debug, Serialize, Deserialize, DefaultJson)]
pub enum QueryOptions {
    FilterPopular,
    FilterNew,
    FilterOld
}

#[derive(Debug, Serialize, Deserialize, DefaultJson)]
pub enum QueryType {
    And,
    Or
}

#[derive(Debug, Serialize, Deserialize, DefaultJson)]
pub enum ContextAuthResult {
    Collection(app_definition::Collection),
    Group(app_definition::Group)
}

#[derive(Debug, Serialize, Deserialize, DefaultJson)]
pub enum ContextType {
    Collection,
    Group
}

#[derive(Debug, Clone, Eq, Hash, Deserialize, Serialize)]
pub struct EntryAndAddress<T>{
	pub address: Address,
	pub entry: T
}

pub type EntryAndAddressResult<T> = Vec<EntryAndAddress<T>>;

//Parameters for each function in holochain application
#[derive(Serialize, Debug)]
pub enum FunctionParameters{
    CreatePack{
        username_address: Address,
        first_name: String
    },
    CreateDen{
        username_address: Address,
        first_name: String
    }
}

impl<T: Into<JsonString>> From<EntryAndAddress<T>> for JsonString  where T: Serialize + Debug{
    fn from(result: EntryAndAddress<T>) -> JsonString {
        JsonString::from(default_to_json(result))
    }
}

impl<T> From<JsonString> for EntryAndAddress<T> where T: DeserializeOwned + Debug{
    fn from(result: JsonString) -> EntryAndAddress<T>{
        serde_json::from_str(result.to_string().as_str())
            .unwrap_or_else(|_| panic!("could not deserialize: {:?}", result))
    }
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

impl From<FunctionParameters> for JsonString {
    fn from(result: FunctionParameters) -> JsonString {
        JsonString::from(default_to_json(result))
    }
}

impl HooksResultTypes{
    pub fn create_pack_result(self) -> ZomeApiResult<EntryAndAddress<app_definition::Group>> {
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
}