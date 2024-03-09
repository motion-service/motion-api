use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata{
    #[serde(rename = "_id")]
    pub(crate) id: ObjectId,
    pub(crate) url: Option<Uuid>,
    pub(crate) title: Option<String>,
    pub(crate) parent: Option<Vec<String>>,
    pub(crate) children: Option<Vec<String>>
}

impl Metadata{
    pub fn new(url: Uuid) -> Self{
        Metadata{
            id: Default::default(),
            url:  Option::from(url),
            title: Option::from(String::from("Untitled")),
            parent: Default::default(),
            children: Default::default()
        }
    }
}