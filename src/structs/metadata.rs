use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata{
    pub(crate) title: Option<String>,
    pub(crate) parent: Option<Vec<String>>,
    pub(crate) children: Option<Vec<String>>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinaryMetadata{
    pub(crate) id: Option<String>,
    pub(crate) title: Option<String>,
}

impl From<(Option<String>, Option<Vec<String>>, Option<Vec<String>>)> for Metadata{
    fn from((title,parent, children): (Option<String>, Option<Vec<String>>, Option<Vec<String>>)) -> Self {
        Metadata{
            title: title.to_owned(),
            parent: parent.to_owned(),
            children: children.to_owned()
        }
    }
}

impl Metadata{
    pub fn new() -> Self{
        Metadata{
            title: Option::from(String::from("Untitled")),
            parent: Default::default(),
            children: Default::default()
        }
    }
}