use hdk::{
    holochain_core_types::{
        cas::content::Address, 
        error::HolochainError,
        json::JsonString,
        hash::HashString
    }
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, PartialEq, Clone)]
pub enum Privacy {
    Public, //Viewable by everyone
    Shared, //Viewable by selected people
    Private //Viewable by only owner
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, PartialEq, Clone)]
pub enum TagType {
    Tag,
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
    LongForm{
        title: String,
        body: String
    },
    ShortForm{
        background: String,
        body: String
    },
    PhotoForm{
        image: String,
        caption: String
    },
    EventForm{
        title: String,
        date: String,
        location: String,
        details: String
    },
    BulletForm{
        title: String,
        bullets: Vec<String>
    }
}

//This anchor will serve as a global index entry to link users, types, times and tags from
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Anchor {
    pub anchor_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Bucket {
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson, PartialEq)]
pub struct Config {
    pub config_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct User {
    pub parent: HashString, //Parent HashString allows user object to be unique for a given username
    pub first_name: String,
    pub last_name: String,
    pub bio: String,
    pub profile_picture: String,
    pub verified: bool
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq, Eq, Hash)]
pub struct UserName {
    pub username: String
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Collection {
    pub parent: HashString,
    pub name: String,
    pub privacy: Privacy //Privacy enum 
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Perspective {
    pub parent: HashString,
    pub name: String
}

// Attribute - channel, time and type

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Tag {
    //Attribute of an expressions post - topics & type
    pub value: String,
    pub privacy: Privacy, //Privacy enum 
    pub tag_type: TagType
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct ExpressionPost { 
    //pub parent: HashString,
    pub expression_type: String,
    pub expression: Expression
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Group {
    pub name: String,
    pub owner: Address,
    pub privacy: Privacy 
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Time {
    pub time: String,
    pub time_type: TimeType
}  