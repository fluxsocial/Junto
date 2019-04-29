use hdk::{
    holochain_core_types::{
        cas::content::Address, 
        hash::HashString,
        json::JsonString,
        error::HolochainError
    }
};

use std::collections::HashMap;
use serde::Serialize;

use super::app_definitions;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CreateUserInformation{
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture: String,
    pub bio: String
}

//Basic struct to be used to describe a function and its parameters to the handle_hooks & handle_contextual_links functions
pub struct FunctionDescriptor{  
    pub name: &'static str,
    pub parameters: FunctionParameters,
}

#[derive(Serialize, Deserialize)]
pub struct UserDens{
    pub private_den: Option<GetLinksLoadElement<app_definitions::Channel>>,
    pub shared_den: Option<GetLinksLoadElement<app_definitions::Channel>>,
    pub public_den: Option<GetLinksLoadElement<app_definitions::Channel>>
}

pub enum QueryTarget{
    ExpressionPost,
    User
}

pub enum QueryOptions {
    FilterPopular,
    FilterNew,
    FilterOld
}

impl From<UserDens> for JsonString {
    fn from(result: UserDens) -> JsonString {
        JsonString::from_json(json!({
            "private_den": match result.private_den{
                Some(den) => JsonString::from(den),
                None => JsonString::from("{}")
            },
            "shared_den": match result.shared_den{
                Some(den) => JsonString::from(den),
                None => JsonString::from("{}")
            },
            "public_den": match result.public_den{
                Some(den) => JsonString::from(den),
                None => JsonString::from("{}")
            }
        }).to_string().as_ref())
    }
}

impl<T: Into<JsonString>> From<GetLinksLoadElement<T>> for JsonString  where T: Serialize{
    fn from(result: GetLinksLoadElement<T>) -> JsonString {
        let entry = serde_json::to_string(&result.entry);
        let entry_string: String;
        match entry {
            Ok(entry) => entry_string = entry,
            Err(e) => return JsonString::from(HolochainError::SerializationError(e.to_string()))
        };
        JsonString::from_json(&format!("{{\"address\": {}, \"entry\": {}}}", result.address, entry_string))
    }
}


pub type GetLinksLoadResult<T> = Vec<GetLinksLoadElement<T>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetLinksLoadElement<T> {
	pub address: HashString,
	pub entry: T
}

impl<T> PartialEq for GetLinksLoadElement<T>{
    fn eq(self: &Self, other: &GetLinksLoadElement<T>) -> bool {
        self.address == other.address
    }
}

//Parameters for each function in holochain application
pub enum FunctionParameters{
    GlobalTimeToExpression{
        tag: &'static str, 
        direction: &'static str, 
        expression_address: Address,
    },
    LocalTimeToExpression{
        tag: &'static str, 
        direction: &'static str, 
        expression_address: Address,
        context: Address,
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
        tag: &'static str, 
        direction: &'static str, 
        parent_expression: Address, 
        child_expression: Address
    },
    CreateChannels{
        channels: Vec<String>,
        parent: Address,
        privacy: app_definitions::Privacy
    },
    CreateQueryPoints{
        query_points: Vec<HashMap<String, String>>, 
        context: Address, 
        privacy: app_definitions::Privacy,
        query_type: String,
        expression: Address
    }
}
