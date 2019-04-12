use hdk::{
    holochain_core_types::{
        cas::content::Address, 
        hash::HashString
    }
};
use std::collections::HashMap;

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
        user: Address
    },
    CreateDen{
        user: Address
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
