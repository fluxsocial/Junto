use hdk::{
    holochain_core_types::{
        cas::content::Address, 
        error::HolochainError,
        json::JsonString,
        hash::HashString
    }
};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, DefaultJson, PartialEq, Clone)]
pub enum Privacy {
    Public, //Viewable by everyone
    Shared, //Viewable by selected people
    Private //Viewable by only owner
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, PartialEq, Clone)]
pub enum ChannelType {
    Tag, 
    Den,
    Type 
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, PartialEq, Clone)]
pub enum TimeType {
    Year, 
    Month,
    Day,
    Hour
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, PartialEq, Clone)]
pub enum Expression {
    PostExpression{},
    VideoExpression{},
    LongFormExpression{}
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct User {
    pub parent: HashString, //Parent HashString data objects to be contextual to given data trees
    pub first_name: String,
    pub last_name: String,
    pub bio: String,
    pub profile_picture: String,
    pub verified: bool
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct UserName {
    pub username: String
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Channel {
    //Channels expressions through given objects to provide searchable tree's 
    pub parent: HashString, //Should either be app hash for normal expression channel or user hash for den
    pub name: String,
    pub privacy: Privacy, //Privacy enum 
    pub channel_type: ChannelType
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct ExpressionPost { 
    pub parent: HashString,
    pub expression_type: String,
    pub expression: Expression
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Group {
    pub parent: HashString,
    pub name: String,
    pub owner: Address,
    pub privacy: Privacy 
}

//Possible that Time could be handles by Channel Expression Object
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Time {
    pub parent: HashString,
    pub time: String,
    pub time_type: TimeType
}  

//CODE BELLOW IS OLD AND MESSY - refactoring is necassary here, currently only the contextual_links & hooks vectors are being used to validate that certain functions can be ran on given expression objects
//Its possible this struct may not be useful going forward perhaps this can just be definted in the !entry of the the given expression and then retrieved later
//For now its here - even if just to have some code which defines the structure of the applications data trees
pub struct ExpressionLinkDefinition {
    //Describes possible links that can be made from any given expression object
    //This is used to define routes that the network can/must take in order to provide a searchable library of expression
    //direction: forward = current expression object -> Link item. Reverse = Link item -> current expression object
    //* = tag can be anything, used to allow searching through data trees - will be used to search through multiple channels at the same time

    pub up_links: Vec<HashMap<&'static str, &'static str>>, //Defines expression expression_type(s) which object can be down linked from
    pub down_links: Vec<HashMap<&'static str, &'static str>>, //Defines expression expression_type(s) which object can be linked to current expression object
    pub contextual_links: Vec<HashMap<&'static str, &'static str>>, //Defines expression expression_type(s) which must be linked to given expression upon entry (building contextual paths for searching)
    pub hooks: Vec<HashMap<&'static str, &'static str>> //Defines expression expression_type(s) which must be committed when given expression is committed - might not be necassary to have for any other data other than user - because usually expression object exist independant of each other without children
}

pub fn get_user_definitions() -> ExpressionLinkDefinition {
    let user_expression_link_definitions: ExpressionLinkDefinition = ExpressionLinkDefinition {  
        //Links which user expression can received: UP-LINK -> USER-EXPRESSION-OBJECT
        up_links: vec![hashmap!{"tag" => "owner", "expression_type" => "expression_post"},
                    hashmap!{"tag" => "member", "expression_type" => "Group"},
                    hashmap!{"tag" => "owner", "expression_type" => "Group"}, 
                    hashmap!{"tag" => "user", "expression_type" => "Channel"},
                    hashmap!{"tag" => "user", "expression_type" => "Time"}],
    
        //Links which can attach to user expression as child: USER-EXPRESSION-OBJECT -> DOWN-LINK
        down_links: vec![hashmap!{"tag" => "expression", "expression_type" => "ExpressionPost"}, 
                        hashmap!{"tag" => "resonation", "expression_type" => "Resonation"}, 
                        hashmap!{"tag" => "*", "expression_type" => "Channel"}, 
                        hashmap!{"tag" => "den", "expression_type" => "Channel"}, 
                        hashmap!{"tag" => "time", "expression_type" => "Time"},
                        hashmap!{"tag" => "pack", "expression_type" => "Group"}],

        //Links to be made to create searchable trees to object or child object(s) - complex link structures usually consisting of many links
        contextual_links: vec![],

        //Links which have to be made upon user expression object commit - some of the objects to be linked to wont exist - they must be created in accordance with schema - these are basic links with not more than one link - unlike contextual links
        //Function is just being stored as a string here and not an actual refrence to the function which would make more sense
        //This is beacuse I cant figure out how to store a function in a hashmap/struct/enum gyahhh
        hooks: vec![hashmap!{"tag" => "user", "expression_type" => "Time", "function" => "time_to_expression", "direction" => "reverse"}, //Might need to define some data attribute which explains direction of the link
                    hashmap!{"tag" => "pack", "expression_type" => "Group", "function" => "create_pack", "direction" => "forward"}, //Example is => time goes Time -> User where as pack would go User -> Pack
                    hashmap!{"tag" => "den", "expression_type" => "Channel", "function" => "create_den", "direction" => "forward"}]
    };
    user_expression_link_definitions
}

pub fn get_channel_definitions() -> ExpressionLinkDefinition {
    let channel_expression_link_definitions: ExpressionLinkDefinition = ExpressionLinkDefinition { 
        //Links which channel expression can received: UP-LINK -> CHANNEL-EXPRESSION-OBJECT
        up_links: vec![hashmap!{"tag" => "*", "expression_type" => "User"}, 
                    hashmap!{"tag" => "*", "expression_type" => "Channel"}, 
                    hashmap!{"tag" => "*", "expression_type" => "Time"}, 
                    hashmap!{"tag" => "*", "expression_type" => "Group"}, 
                    hashmap!{"tag" => "*", "expression_type" => "Resonation"}],

        //Links which can attach to channel expression as child: CHANNEL-EXPRESSION-OBJECT -> DOWN-LINK
        down_links: vec![hashmap!{"tag" => "expression", "expression_type" => "ExpressionPost"},
                        hashmap!{"tag" => "*", "expression_type" => "ExpressionPost"}, //Option for any tag on link from channel -> expression allows for querying through tree structures
                        hashmap!{"tag" => "resonation", "expression_type" => "Resonation"},
                        hashmap!{"tag" => "*", "expression_type" => "Resonation"},
                        hashmap!{"tag" => "user", "expression_type" => "User"},
                        hashmap!{"tag" => "*", "expression_type" => "User"},
                        hashmap!{"tag" => "*", "expression_type" => "Channel"},
                        hashmap!{"tag" => "time", "expression_type" => "Time"},
                        hashmap!{"tag" => "*", "expression_type" => "Time"}],

        //No contextual links on commit of channel item - contextual links only need to be made if a resonation or expression is being associated with channel
        contextual_links: vec![],
        hooks: vec![hashmap!{"tag" => "channel", "expression_type" => "Time", "function" => "", "direction" => "reverse"}, //Anytime expression is committed the time of the expression creation should be linked to relevant time object(s)
                    hashmap!{"tag" => "den", "expression_type" => "User", "function" => "link_expression", "direction" => "reverse"}, //Link user channel will only run if channel is of privacy type != public, in our case this would make it a den
                    hashmap!{"tag" => "owner", "expression_type" => "User", "function" => "link_expression", "direction" => "forward"}]  
    };
    channel_expression_link_definitions
}

pub fn get_post_expression_definitions() -> ExpressionLinkDefinition {
    let post_expression_link_definitions: ExpressionLinkDefinition = ExpressionLinkDefinition { 
        up_links: vec![hashmap!{"tag" => "expression", "expression_type" => "User"}, 
                    hashmap!{"tag" => "expression", "expression_type" => "Channel"},
                    hashmap!{"tag" => "*", "expression_type" => "Channel"}, 
                    hashmap!{"tag" => "expression", "expression_type" => "Resonation"}, 
                    hashmap!{"tag" => "*", "expression_type" => "Resonation"}, 
                    hashmap!{"tag" => "expression", "expression_type" => "Time"},
                    hashmap!{"tag" => "*", "expression_type" => "Time"}, 
                    hashmap!{"tag" => "expression", "expression_type" => "Group"}],

        down_links: vec![hashmap!{"tag" => "owner", "expression_type" => "User"}, 
                        hashmap!{"tag" => "comment", "expression_type" => "ExpressionPost"}, 
                        hashmap!{"tag" => "*", "expression_type" => "Resonation"}],

        contextual_links: vec![hashmap!{"tag" => "*", "expression_type" => "*", "function" => "create_expression_paths"}],  //Link to any other channels in expression commit and to relevant user den

        hooks: vec![hashmap!{"tag" => "expression", "expression_type" => "Time", "function" => "time_to_expression", "direction" => "reverse"},
                    hashmap!{"tag" => "expression", "expression_type" => "Time", "function" => "time_to_expression", "direction" => "reverse"},
                    hashmap!{"tag" => "expression", "expression_type" => "Channel", "function" => "create_query_points", "direction" => "reverse"}] //To any associated channels 
                    // hashmap!{"tag" => "expression", "expression_type" => "User", "function" => "expression_to_user", "direction" => "reverse"}] //To user - might not be necassary here, traditionally hooks are only used for creating objects not linking, linking might happen in a contextual link function
    };
    post_expression_link_definitions
}

pub fn get_group_definitions() -> ExpressionLinkDefinition {
    let group_expression_link_definitions: ExpressionLinkDefinition = ExpressionLinkDefinition { 
        up_links: vec![hashmap!{"tag" => "pack", "expression_type" => "User"},
                    hashmap!{"tag" => "member", "expression_type" => "User"},
                    hashmap!{"tag" => "owner", "expression_type" => "User"},
                    hashmap!{"tag" => "*", "expression_type" => "Time"}],

        down_links: vec![hashmap!{"tag" => "expression", "expression_type" => "ExpressionPost"},
                        hashmap!{"tag" => "*", "expression_type" => "Resonation"},
                        hashmap!{"tag" => "*", "expression_type" => "Channel"},
                        hashmap!{"tag" => "*", "expression_type" => "Time"},
                        hashmap!{"tag" => "member", "expression_type" => "User"},
                        hashmap!{"tag" => "owner", "expression_type" => "User"}],

        contextual_links: vec![],
        //Currently pack tag is created between User -> Group, upon any commit of a group - in the future non pack groups might be possible then there should be a way to ensure this tag is not created
        hooks: vec![hashmap!{"tag" => "group", "expression_type" => "Time", "function" => "time_to_expression", "direction" => "reverse"},
                    hashmap!{"tag" => "pack", "expression_type" => "Time", "function" => "time_to_expression", "direction" => "reverse"},
                    hashmap!{"tag" => "pack", "expression_type" => "User", "function" => "link_expression", "direction" => "both"}]
    };
    group_expression_link_definitions
}

pub fn get_resonation_definitions() -> ExpressionLinkDefinition {
    let resonation_link_definitions: ExpressionLinkDefinition = ExpressionLinkDefinition { 
        up_links: vec![hashmap!{"tag" => "resonation", "expression_type" => "ExpressionPost"},
                    hashmap!{"tag" => "resonation", "expression_type" => "Channel"},
                    hashmap!{"tag" => "*", "expression_type" => "Channel"},
                    hashmap!{"tag" => "*", "expression_type" => "User"},
                    hashmap!{"tag" => "*", "expression_type" => "Group"},
                    hashmap!{"tag" => "*", "expression_type" => "Time"}],

        down_links: vec![hashmap!{"tag" => "expression", "expression_type" => "ExpressionPost"},
                        hashmap!{"tag" => "*", "expression_type" => "Channel"}],

        contextual_links: vec![hashmap!{"tag" => "*", "expression_type" => "Channel", "function" => "function to handle contextualy link to channels"},  //Link to any other channels in expression commit and to relevant user den
                            hashmap!{"tag" => "*", "expression_type" => "Group", "function" => "function to contextualy link to relevant groups"},  //Link to any packs "groups" which the expression should be inserted into
                            hashmap!{"tag" => "*", "expression_type" => "Time", "function" => "function to contextualy link to time"}],

        hooks: vec![hashmap!{"tag" => "resonation", "expression_type" => "Channel", "function" => "", "direction" => "both"}, //To any associated channels 
                    hashmap!{"tag" => "resonation", "expression_type" => "Channel", "function" => "", "direction" => "reverse"}, //To den
                    hashmap!{"tag" => "resonation", "expression_type" => "Group", "function" => "", "direction" => "reverse"}, //To pack
                    hashmap!{"tag" => "resonation", "expression_type" => "Time", "function" => "", "direction" => "reverse"}, //To timestamp
                    hashmap!{"tag" => "resonation", "expression_type" => "User", "function" => "", "direction" => "reverse"},
                    hashmap!{"tag" => "resonation", "expression_type" => "ExpressionPost", "function" => "", "direction" => "reverse"}]
    };
    resonation_link_definitions
}

pub fn get_time_definitions() -> ExpressionLinkDefinition{
    let time_link_definitions: ExpressionLinkDefinition = ExpressionLinkDefinition {
        up_links: vec![hashmap!{"tag" => "time", "expression_type" => "Group"},
                    hashmap!{"tag" => "time", "expression_type" => "User"},
                    hashmap!{"tag" => "*", "expression_type" => "Channel"},
                    hashmap!{"tag" => "*", "expression_type" => "ExpressionPost"}],

        down_links: vec![hashmap!{"tag" => "user", "expression_type" => "User"},
                        hashmap!{"tag" => "*", "expression_type" => "Channel"},
                        hashmap!{"tag" => "*", "expression_type" => "ExpressionPost"},
                        hashmap!{"tag" => "expression", "expression_type" => "ExpressionPost"},
                        hashmap!{"tag" => "*", "expression_type" => "Resonation"}],

        contextual_links: vec![hashmap!{"tag" => "*", "expression_type" => "Channel", "function" => "function to handle contextualy link to channels"},  //Link to any other channels in expression commit and to relevant user den
                            hashmap!{"tag" => "*", "expression_type" => "Group", "function" => "function to contextualy link to relevant groups"}],

        hooks: vec![]
    };
    time_link_definitions
}