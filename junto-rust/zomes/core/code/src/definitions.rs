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
    profile_picture: String
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
    parent: String
}  

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Resonation {

}

//Its possible this struct may not be useful going forward perhaps this can just be definted in the !entry of the the given expression and then retrieved later
//For now its here - even if just to have some code which defines the structure of the applications data trees - possible that this diagram could be used to create a visualized tree
pub struct ExpressionLinkDefinition {
    //Describes possible links that can be made from any given expression object
    //This is used to define routes that the network can/must take in order to provide a searchable library of expression
    //vector schema: [{"tag": "linkTag", "type": "typeoflinkingitem", *optional* "direction": "", *optional* "function": "", *optional* "validation": ""}] 
    //possible that hashmap could also provide references to validation functions
    //direction: forward = current expression object -> Link item. Reverse = Link item -> current expression object
    //* = tag can be anything, used to allow searching through data trees - will be used to search through multiple channels at the same time

    up_links: Vec<HashMap<String, String>>, //Defines expression type(s) which object can be down linked from
    down_links: Vec<HashMap<String, String>>, //Defines expression type(s) which object can be linked to current expression object
    contextual_links: Vec<HashMap<String, String>>, //Defines expression type(s) which must be linked to given expression upon entry (building contextual paths for searching)
    hooks: Vec<HashMap<String, String>> //Defines expression type(s) which must be committed when given expression is committed - might not be necassary to have for any other data other than user - because usually expression object exist independant of each other without children
    //example for hook would be if channel is included in expression post it must be linked to all expression items in hook vector
    //or user pack created upon user creation

    //Contextual links probably wont only be run just on the commit of the given object but also on the re-reference of the object upon another commit
    //Example would be expression channel which has already been commited being reference again on another expression post - contextual links may still need to be made to appropriate resonations/groups/users etc
}

pub const USER_EXPRESSION_LINK_DEFINITIONS = ExpressionLinkDefinition {  
    up_links: vec![&map!{"tag" => "user", "type": "ExpressionPost"},
                   &map!{"tag" => "member", "type": "Group"},
                   &map!{"tag" => "owner", "type": "Group"}, 
                   &map!{"tag" => "user", "type": "Channel"},
                   &map!{"tag" => "user", "type": "Time"},],
   
    down_links: vec![&map!{"tag" => "expression", "type": "ExpressionPost"}, 
                     &map!{"tag" => "resonation", "type": "Resonation"}, 
                     &map!{"tag" => "*", "type": "Channel"}, 
                     &map!{"tag" => "den", "type": "Channel"}, 
                     &map!{"tag" => "time", "type" => "Time"},
                     &map!{"tag" => "pack", "type": "Group"}],

    contextual_links: vec![],
    hooks: vec![&map!{"tag" => "user", "type": "Time", "function": "functionToBeExecutedUponCommit", "direction": "reverse"}, //Might need to define some data attribute which explains direction of the link
                &map!{"tag" => "pack", "type": "Group", "function": "", "direction": "forward"}, //Example is: time goes Time -> User where as pack would go User -> Pack
                &map!{"tag" => "den", "type": "Channel", "function": "", "direction": "forward"}]
}

pub const CHANNEL_EXPRESSION_LINK_DEFINITIONS = ExpressionLinkDefinition { 
    up_links: vec![&map!{"tag" => "*", "type": "User"}, 
                   &map!{"tag" => "*", "type": "Channel"}, 
                   &map!{"tag" => "*", "type": "Time"}, 
                   &map!{"tag" => "*", "type": "Group"}, 
                   &map!{"tag" => "*", "type": "Resonation"}],

    down_links: vec![&map!{"tag" => "expression", "type": "ExpressionPost"},
                     &map!{"tag" => "*", "type": "ExpressionPost"}, //Option for any tag on link from channel -> expression allows for querying through tree structures
                     &map!{"tag" => "resonation", "type": "Resonation"},
                     &map!{"tag" => "*", "type": "Resonation"},
                     &map!{"tag" => "user", "type": "User"},
                     &map!{"tag" => "*", "type": "User"},
                     &map!{"tag" => "*", "type": "Channel"},
                     &map!{"tag" => "time", "type": "Time"},
                     &map!{"tag" => "*", "type": "Time"}],

    contextual_links: vec![&map!{"tag" => "*", "type": "Channel", "function": "function to handle contextualy link to channels"},  //Link to any other channels in expression commit and to relevant user den
                           &map!{"tag" => "*", "type": "Group", "function": "function to contextualy link to relevant groups"},  //Link to any packs "groups" which the expression should be inserted into
                           &map!{"tag" => "*", "type": "Time", "function": "function to contextualy link to time"},
                           &map!{"tag" => "*", "type": "Resonation", "function": "function to contextualy link to associated resonations"}], //Link to time of expression

    hooks: vec![&map!{"tag" => "*", "type": "Time", "function": "", "direction": "reverse"}]  //Anytime expression is committed the time of the expression creation should be linked to relevant time object(s)
}

pub const POST_EXPRESSION_LINK_DEFINITIONS = ExpressionLinkDefinition { 
    up_links: vec![&map!{"tag" => "expression", "type": "User"}, 
                   &map!{"tag" => "expression", "type": "Channel"},
                   &map!{"tag" => "*", "type": "Channel"}, 
                   &map!{"tag" => "expression", "type": "Resonation"}, 
                   &map!{"tag" => "*", "type": "Resonation"}, 
                   &map!{"tag" => "expression", "type": "Time"},
                   &map!{"tag" => "*", "type": "Time"}, 
                   &map!{"tag" => "expression", "type": "Group"}],

    down_links: vec![&map!{"tag" => "user", "type": "User"}, 
                     &map!{"tag" => "comment", "type": "ExpressionPost"}, 
                     &map!{"tag" => "resonation", "type": "Resonation"}],

    contextual_links: vec![],
    hooks: vec![&map!{"tag" => "*", "type": "Channel", "function": "", "direction": "reverse"}, //To any associated channels 
                &map!{"tag" => "expression", "type": "Channel", "function": "", "direction": "reverse"}, //To den
                &map!{"tag" => "expression", "type": "Resonation", "function": "", "direction": "reverse"},
                &map!{"tag" => "expression", "type": "Group", "function": "", "direction": "reverse"}, //To pack
                &map!{"tag" => "expression", "type": "Time", "function": "", "direction": "reverse"}, //To timestamp
                &map!{"tag" => "expression", "type": "User", "function": "", "direction": "reverse"}] //To user
}

pub const GROUP_EXPRESSION_LINK_DEFINITIONS = ExpressionLinkDefinition { 
    up_links: vec![&map!{"tag" => "pack", "type": "User"},
                   &map!{"tag" => "member", "type": "User"},
                   &map!{"tag" => "owner", "type": "User"},
                   &map!{"tag" => "*", "type": "Time"}],

    down_links: vec![&map!{"tag" => "expression", "type": "ExpressionPost"},
                     &map!{"tag" => "resonation", "type": "Resonation"},
                     &map!{"tag" => "*", "type": "Channel"},
                     &map!{"tag" => "*", "type": "Time"},
                     &map!{"tag" => "member", "type": "User"},
                     &map!{"tag" => "owner", "type": "User"}],

    contextual_links: vec![],
    hooks: vec![&map!{"tag" => "group", "type": "Time", "function": "", "direction": "reverse"},
                &map!{"tag" => "pack", "type": "User", "function": "", "direction": "reverse"},
                &map!{"tag" => "owner", "type": "User", "function": "", "direction": "both"}]
}

pub const RESONATION_EXPRESSION_LINK_DEFINITIONS = ExpressionLinkDefinition { 
    up_links: vec![&map!{"tag" => "resonation", "type": "ExpressionPost"},
                   &map!{"tag" => "resonation", "type": "Channel"},
                   &map!{"tag" => "*", "type": "Channel"},
                   &map!{"tag" => "resonation", "type": "User"},
                   &map!{"tag" => "resonation", "type": "Group"},
                   &map!{"tag" => "resonation", "type": "Time"}],

    down_links: vec![&map!{"tag" => "expression", "type": "ExpressionPost"},
                     &map!{"tag" => "*", "type": "Channel"}],

    contextual_links: vec![&map!{"tag" => "*", "type": "Channel", "function": "function to handle contextualy link to channels"},  //Link to any other channels in expression commit and to relevant user den
                           &map!{"tag" => "*", "type": "Group", "function": "function to contextualy link to relevant groups"},  //Link to any packs "groups" which the expression should be inserted into
                           &map!{"tag" => "*", "type": "Time", "function": "function to contextualy link to time"}],

    hooks: vec![&map!{"tag" => "*", "type": "Channel", "function": "", "direction": "both"}, //To any associated channels 
                &map!{"tag" => "resonation", "type": "Channel", "function": "", "direction": "reverse"}, //To den
                &map!{"tag" => "resonation", "type": "Group", "function": "", "direction": "reverse"}, //To pack
                &map!{"tag" => "resonation", "type": "Time", "function": "", "direction": "reverse"}, //To timestamp
                &map!{"tag" => "resonation", "type": "User", "function": "", "direction": "reverse"},
                &map!{"tag" => "resonation", "type": "ExpressionPost", "function": "", "direction": "reverse"}]
}

pub const TIME_LINK_DEFINITIONS = ExpressionLinkDefinition {
    up_links: vec![&map!{"tag" => "time", "type": "Group"},
                   &map!{"tag" => "time", "type": "User"},
                   &map!{"tag" => "time", "type": "Channel"},
                   &map!{"tag" => "*", "type": "ExpressionPost"}],

    down_links: vec![&map!{"tag" => "user", "type": "User"},
                     &map!{"tag" => "*", "type": "Channel"},
                     &map!{"tag" => "*", "type": "ExpressionPost"},
                     &map!{"tag" => "expression", "type": "ExpressionPost"},
                     &map!{"tag" => "*", "type": "Resonation"}],

    contextual_links: vec![&map!{"tag" => "*", "type": "Channel", "function": "function to handle contextualy link to channels"},  //Link to any other channels in expression commit and to relevant user den
                           &map!{"tag" => "*", "type": "Group", "function": "function to contextualy link to relevant groups"}],

    hooks: vec![&map!{"tag" => "*", "type": "Channel", "function": "", "direction": "both"}, //To any associated channels 
                &map!{"tag" => "resonation", "type": "Channel", "function": "", "direction": "both"}, //To den
                &map!{"tag" => "resonation", "type": "Group", "function": "", "direction": "both"}, //To pack
                &map!{"tag" => "resonation", "type": "Time", "function": "", "direction": "both"}, //To timestamp
                &map!{"tag" => "resonation", "type": "User", "function": "", "direction": "both"},
                &map!{"tag" => "resonation", "type": "ExpressionPost", "function": "", "direction": "both"}]]
}