use hdk::{
    holochain_core_types::cas::content::Address,
    holochain_core_types::dna::zome::entry_types::Sharing,
    holochain_core_types::entry::{entry_type::EntryType, Entry},
    holochain_core_types::error::HolochainError,
    holochain_core_types::json::JsonString,
    holochain_core_types::hash::HashString,
};

use std::collections::HashMap;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct User {
    parent: String, //Parent field allows data objects to be contextual to given data trees
    first_name: String,
    last_name: String,
    bio: String,
    profile_picture: String,
    verified: bool
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Channel {
    parent: String
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct ExpressionPost { 
    parent: String
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Group {
    parent: String
}

//Possible that Time could be handles by Channel Expression Object
#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Time {
    parent: String,
    timestamp: String
}  

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Resonation {

}

//Its possible this struct may not be useful going forward perhaps this can just be definted in the !entry of the the given expression and then retrieved later
//For now its here - even if just to have some code which defines the structure of the applications data trees - possible that this diagram could be used to create a visualized tree
pub struct ExpressionLinkDefinition {
    //Describes possible links that can be made from any given expression object
    //This is used to define routes that the network can/must take in order to provide a searchable library of expression
    //vector schema: [{"tag": "linkTag", "expression_type": "expression_typeoflinkingitem", *optional* "direction": "", *optional* "function": "", *optional* "validation": ""}] 
    //possible that hashmap could also provide references to validation functions
    //direction: forward = current expression object -> Link item. Reverse = Link item -> current expression object
    //* = tag can be anything, used to allow searching through data trees - will be used to search through multiple channels at the same time

    up_links: Vec<HashMap<String, String>>, //Defines expression expression_type(s) which object can be down linked from
    down_links: Vec<HashMap<String, String>>, //Defines expression expression_type(s) which object can be linked to current expression object
    contextual_links: Vec<HashMap<String, String>>, //Defines expression expression_type(s) which must be linked to given expression upon entry (building contextual paths for searching)
    hooks: Vec<HashMap<String, String>> //Defines expression expression_type(s) which must be committed when given expression is committed - might not be necassary to have for any other data other than user - because usually expression object exist independant of each other without children
    //example for hook would be if channel is included in expression post it must be linked to all expression items in hook vector
    //or user pack created upon user creation

    //Contextual links probably wont only be run just on the commit of the given object but also on the re-reference of the object upon another commit
    //Example would be expression channel which has already been commited being reference again on another expression post - contextual links may still need to be made to appropriate resonations/groups/users etc
}

pub const USER_EXPRESSION_LINK_DEFINITIONS: ExpressionLinkDefinition = ExpressionLinkDefinition {  
    //Links which user expression can received: UP-LINK -> USER-EXPRESSION-OBJECT
    up_links: vec![&map!{"tag" => "user", "expression_type" => "ExpressionPost"},
                   &map!{"tag" => "member", "expression_type" => "Group"},
                   &map!{"tag" => "owner", "expression_type" => "Group"}, 
                   &map!{"tag" => "user", "expression_type" => "Channel"},
                   &map!{"tag" => "user", "expression_type" => "Time"}],
   
    //Links which can attach to user expression as child: USER-EXPRESSION-OBJECT -> DOWN-LINK
    down_links: vec![&map!{"tag" => "expression", "expression_type" => "ExpressionPost"}, 
                     &map!{"tag" => "resonation", "expression_type" => "Resonation"}, 
                     &map!{"tag" => "*", "expression_type" => "Channel"}, 
                     &map!{"tag" => "den", "expression_type" => "Channel"}, 
                     &map!{"tag" => "time", "expression_type" => "Time"},
                     &map!{"tag" => "pack", "expression_type" => "Group"}],

    //Links to be made to create searchable trees to object or child object(s) - complex link structures usually consisting of many links
    contextual_links: vec![],

    //Links which have to be made upon user expression object commit - some of the objects to be linked to wont exist - they must be created in accordance with schema - these are basic links with not more than one link - unlike contextual links
    hooks: vec![&map!{"tag" => "user", "expression_type" => "Time", "function" => "", "direction" => "reverse"}, //Might need to define some data attribute which explains direction of the link
                &map!{"tag" => "pack", "expression_type" => "Group", "function" => "", "direction" => "forward"}, //Example is => time goes Time -> User where as pack would go User -> Pack
                &map!{"tag" => "den", "expression_type" => "Channel", "function" => "", "direction" => "forward"}]
};

pub const CHANNEL_EXPRESSION_LINK_DEFINITIONS: ExpressionLinkDefinition = ExpressionLinkDefinition { 
    //Links which channel expression can received: UP-LINK -> CHANNEL-EXPRESSION-OBJECT
    up_links: vec![&map!{"tag" => "*", "expression_type" => "User"}, 
                   &map!{"tag" => "*", "expression_type" => "Channel"}, 
                   &map!{"tag" => "*", "expression_type" => "Time"}, 
                   &map!{"tag" => "*", "expression_type" => "Group"}, 
                   &map!{"tag" => "*", "expression_type" => "Resonation"}],

    //Links which can attach to channel expression as child: CHANNEL-EXPRESSION-OBJECT -> DOWN-LINK
    down_links: vec![&map!{"tag" => "expression", "expression_type" => "ExpressionPost"},
                     &map!{"tag" => "*", "expression_type" => "ExpressionPost"}, //Option for any tag on link from channel -> expression allows for querying through tree structures
                     &map!{"tag" => "resonation", "expression_type" => "Resonation"},
                     &map!{"tag" => "*", "expression_type" => "Resonation"},
                     &map!{"tag" => "user", "expression_type" => "User"},
                     &map!{"tag" => "*", "expression_type" => "User"},
                     &map!{"tag" => "*", "expression_type" => "Channel"},
                     &map!{"tag" => "time", "expression_type" => "Time"},
                     &map!{"tag" => "*", "expression_type" => "Time"}],

    //No contextual links on commit of channel item - contextual links only need to be made if a resonation or expression is being associated with channel
    contextual_links: vec![],
    hooks: vec![&map!{"tag" => "channel", "expression_type" => "Time", "function" => "", "direction" => "reverse"}]  //Anytime expression is committed the time of the expression creation should be linked to relevant time object(s)
};

pub const POST_EXPRESSION_LINK_DEFINITIONS: ExpressionLinkDefinition = ExpressionLinkDefinition { 
    up_links: vec![&map!{"tag" => "expression", "expression_type" => "User"}, 
                   &map!{"tag" => "expression", "expression_type" => "Channel"},
                   &map!{"tag" => "*", "expression_type" => "Channel"}, 
                   &map!{"tag" => "expression", "expression_type" => "Resonation"}, 
                   &map!{"tag" => "*", "expression_type" => "Resonation"}, 
                   &map!{"tag" => "expression", "expression_type" => "Time"},
                   &map!{"tag" => "*", "expression_type" => "Time"}, 
                   &map!{"tag" => "expression", "expression_type" => "Group"}],

    down_links: vec![&map!{"tag" => "user", "expression_type" => "User"}, 
                     &map!{"tag" => "comment", "expression_type" => "ExpressionPost"}, 
                     &map!{"tag" => "resonation", "expression_type" => "Resonation"}],

    contextual_links: vec![&map!{"tag" => "*", "expression_type" => "Channel", "function" => "function to handle contextualy link to channels"},  //Link to any other channels in expression commit and to relevant user den
                           &map!{"tag" => "*", "expression_type" => "Group", "function" => "function to contextualy link to relevant groups"},  //Link to any packs "groups" which the expression should be inserted into
                           &map!{"tag" => "*", "expression_type" => "Time", "function" => "function to contextualy link to time"},
                           &map!{"tag" => "*", "expression_type" => "Resonation", "function" => "function to contextualy link to associated resonations"}],

    hooks: vec![&map!{"tag" => "expression", "expression_type" => "Channel", "function" => "", "direction" => "reverse"}, //To any associated channels 
                &map!{"tag" => "expression", "expression_type" => "Channel", "function" => "", "direction" => "reverse"}, //To den
                &map!{"tag" => "expression", "expression_type" => "Resonation", "function" => "", "direction" => "reverse"},
                &map!{"tag" => "expression", "expression_type" => "Group", "function" => "", "direction" => "reverse"}, //To pack
                &map!{"tag" => "expression", "expression_type" => "Time", "function" => "", "direction" => "reverse"}, //To timestamp
                &map!{"tag" => "expression", "expression_type" => "User", "function" => "", "direction" => "reverse"}] //To user
};

pub const GROUP_EXPRESSION_LINK_DEFINITIONS: ExpressionLinkDefinition = ExpressionLinkDefinition { 
    up_links: vec![&map!{"tag" => "pack", "expression_type" => "User"},
                   &map!{"tag" => "member", "expression_type" => "User"},
                   &map!{"tag" => "owner", "expression_type" => "User"},
                   &map!{"tag" => "*", "expression_type" => "Time"}],

    down_links: vec![&map!{"tag" => "expression", "expression_type" => "ExpressionPost"},
                     &map!{"tag" => "resonation", "expression_type" => "Resonation"},
                     &map!{"tag" => "*", "expression_type" => "Channel"},
                     &map!{"tag" => "*", "expression_type" => "Time"},
                     &map!{"tag" => "member", "expression_type" => "User"},
                     &map!{"tag" => "owner", "expression_type" => "User"}],

    contextual_links: vec![],
    hooks: vec![&map!{"tag" => "group", "expression_type" => "Time", "function" => "", "direction" => "reverse"},
                &map!{"tag" => "pack", "expression_type" => "User", "function" => "", "direction" => "reverse"},
                &map!{"tag" => "owner", "expression_type" => "User", "function" => "", "direction" => "both"}]
};

pub const RESONATION_EXPRESSION_LINK_DEFINITIONS: ExpressionLinkDefinition = ExpressionLinkDefinition { 
    up_links: vec![&map!{"tag" => "resonation", "expression_type" => "ExpressionPost"},
                   &map!{"tag" => "resonation", "expression_type" => "Channel"},
                   &map!{"tag" => "*", "expression_type" => "Channel"},
                   &map!{"tag" => "resonation", "expression_type" => "User"},
                   &map!{"tag" => "resonation", "expression_type" => "Group"},
                   &map!{"tag" => "resonation", "expression_type" => "Time"}],

    down_links: vec![&map!{"tag" => "expression", "expression_type" => "ExpressionPost"},
                     &map!{"tag" => "*", "expression_type" => "Channel"}],

    contextual_links: vec![&map!{"tag" => "*", "expression_type" => "Channel", "function" => "function to handle contextualy link to channels"},  //Link to any other channels in expression commit and to relevant user den
                           &map!{"tag" => "*", "expression_type" => "Group", "function" => "function to contextualy link to relevant groups"},  //Link to any packs "groups" which the expression should be inserted into
                           &map!{"tag" => "*", "expression_type" => "Time", "function" => "function to contextualy link to time"}],

    hooks: vec![&map!{"tag" => "resonation", "expression_type" => "Channel", "function" => "", "direction" => "both"}, //To any associated channels 
                &map!{"tag" => "resonation", "expression_type" => "Channel", "function" => "", "direction" => "reverse"}, //To den
                &map!{"tag" => "resonation", "expression_type" => "Group", "function" => "", "direction" => "reverse"}, //To pack
                &map!{"tag" => "resonation", "expression_type" => "Time", "function" => "", "direction" => "reverse"}, //To timestamp
                &map!{"tag" => "resonation", "expression_type" => "User", "function" => "", "direction" => "reverse"},
                &map!{"tag" => "resonation", "expression_type" => "ExpressionPost", "function" => "", "direction" => "reverse"}]
};

pub const TIME_LINK_DEFINITIONS: ExpressionLinkDefinition = ExpressionLinkDefinition {
    up_links: vec![&map!{"tag" => "time", "expression_type" => "Group"},
                   &map!{"tag" => "time", "expression_type" => "User"},
                   &map!{"tag" => "time", "expression_type" => "Channel"},
                   &map!{"tag" => "*", "expression_type" => "ExpressionPost"}],

    down_links: vec![&map!{"tag" => "user", "expression_type" => "User"},
                     &map!{"tag" => "*", "expression_type" => "Channel"},
                     &map!{"tag" => "*", "expression_type" => "ExpressionPost"},
                     &map!{"tag" => "expression", "expression_type" => "ExpressionPost"},
                     &map!{"tag" => "*", "expression_type" => "Resonation"}],

    contextual_links: vec![&map!{"tag" => "*", "expression_type" => "Channel", "function" => "function to handle contextualy link to channels"},  //Link to any other channels in expression commit and to relevant user den
                           &map!{"tag" => "*", "expression_type" => "Group", "function" => "function to contextualy link to relevant groups"}],

    hooks: vec![]
};