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
    LinkExpression{
        tag: &'static str, 
        direction: &'static str, 
        parent_expression: Address, 
        child_expression: Address
    },
}
