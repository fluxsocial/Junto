use hdk::{
    holochain_core_types::{
        cas::content::Address, 
    }
};

pub struct FunctionDescriptor{  
    pub name: &'static str,
    pub parameters: FunctionParameters,
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
    PackLink{
        tag: &'static str, 
        direction: &'static str, 
        pack: Address, 
        expression: Address
    },
    LinkUserChannel{
        tag: &'static str, 
        direction: &'static str, 
        channel: Address, 
        user: Address
    },
}
