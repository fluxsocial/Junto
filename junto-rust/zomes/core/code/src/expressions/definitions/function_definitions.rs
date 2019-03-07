use hdk::{
    holochain_core_types::{
        cas::content::Address, 
    }
};

use super::app_definitions;

//Basic struct to be used to describe a function and its parameters to the handle_hooks & handle_contextual_links functions
pub struct FunctionDescriptor{  
    pub name: &'static str,
    pub parameters: FunctionParameters,
}

pub struct UserDens{
    pub private_den: Option<app_definitions::GetLinksLoadElement<app_definitions::Channel>>,
    pub shared_den: Option<app_definitions::GetLinksLoadElement<app_definitions::Channel>>,
    pub public_den: Option<app_definitions::GetLinksLoadElement<app_definitions::Channel>>
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
    }
}
