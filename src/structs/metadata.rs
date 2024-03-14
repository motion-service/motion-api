use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::binary::BinaryUuid;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata{
    #[serde(rename = "_id")]
    pub(crate) id: ObjectId,
    pub(crate) url: Option<String>,
    pub(crate) title: Option<String>,
    pub(crate) parent: Option<Vec<String>>,
    pub(crate) children: Option<Vec<String>>
}

impl Metadata{
    pub fn new(url: String) -> Self{
        Metadata{
            id: Default::default(),
            url:  Option::from(url),
            title: Option::from(String::from("Untitled")),
            parent: Default::default(),
            children: Default::default()
        }
    }
}