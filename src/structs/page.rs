use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::structs::default_block::DefaultBlock;
use crate::structs::metadata::Metadata;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    #[serde(rename = "_id")]
    pub(crate) id: ObjectId,
    pub(crate) owner: Option<String>,
    pub(crate) metadata: Option<Metadata>,
    pub(crate) date: Option<String>,
    pub(crate) is_shared: Option<bool>,
    pub(crate) index: Option<i32>
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageBuilder {
    #[serde(rename = "_id")]
    pub(crate) id: ObjectId,
    pub(crate) owner: Option<String>,
    pub(crate) metadata: Option<Metadata>,
    pub(crate) date: Option<String>,
    pub(crate) is_shared: Option<bool>,
    pub(crate) index: Option<i32>
}

impl PageBuilder{
    pub fn new() -> Self{
        PageBuilder{
            id: Default::default(),
            owner: None,
            metadata: None,
            date: None,
            is_shared: None,
            index: None,
        }
    }

    pub fn id(mut self, id: ObjectId) -> Self{
        self.id = id;
        self
    }

    pub fn owner(mut self, owner: String) -> Self{
        self.owner = Some(owner);
        self
    }
    pub fn metadata(mut self, metadata: Metadata) -> Self{
        self.metadata = Some(metadata);
        self
    }
    pub fn date(mut self, date: String) -> Self{
        self.date = Some(date);
        self
    }
    pub fn is_shared(mut self, is_shared: bool) -> Self{
        self.is_shared = Some(is_shared);
        self
    }

    pub fn index(mut self, index: i32) -> Self{
        self.index = Some(index);
        self
    }

    pub fn build(self) -> Page{
        Page{
            id: self.id,
            owner: self.owner,
            metadata: self.metadata,
            date: self.date,
            is_shared: self.is_shared,
            index: self.index,
        }
    }
}

// impl Page{
//     pub fn new(owner: String) -> Self{
//         let metadata = Metadata::new();
//
//         Page {
//             id: Default::default(),
//             owner: Some(owner),
//             metadata: Some(metadata),
//             blocks: Option::from(vec![]), // Use default value if blocks is None
//             date: Some(Default::default()),
//             is_shared: Some(false),
//             index: Some(1)
//         }
//     }
// }

// impl From<(ObjectId, String, Metadata, Option<Vec<DefaultBlock>>, String, bool, i32)> for Page {
//     fn from((object_id, owner, metadata, blocks, date, is_shared, index): (ObjectId, String, Metadata, Option<Vec<DefaultBlock>>, String, bool, i32)) -> Self {
//         Page {
//             id: object_id,
//             owner: Some(owner),
//             metadata: Some(metadata),
//             blocks: Option::from(blocks.unwrap_or_default()), // Use default value if blocks is None
//             date: Some(date),
//             is_shared: Some(is_shared),
//             index: Some(index)
//         }
//     }
// }
//
// impl From<(String, Metadata, Option<Vec<DefaultBlock>>, String, bool, i32)> for Page{
//     fn from((owner, metadata, blocks, date, is_shared, index): (String, Metadata, Option<Vec<DefaultBlock>>, String, bool, i32)) -> Self {
//
//         Page{
//             id: Default::default(),
//             owner: Option::from(owner),
//             metadata: Option::from(metadata),
//             blocks: blocks.to_owned(),
//             date: Option::from(date),
//             is_shared: Option::from(is_shared),
//             index: Option::from(index)
//         }
//     }
// }