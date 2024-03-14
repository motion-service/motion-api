
use std::default::Default;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockBuilder {
    pub(crate) _id: ObjectId,
    pub(crate) uuid: Option<String>,
    pub(crate) value: Option<String>,
    pub(crate) is_editing: Option<bool>
}

impl BlockBuilder{
    pub fn new(uuid: String, value: String, is_editing: bool) -> Self{
        BlockBuilder{
            _id: Default::default(),
            uuid: Option::from(uuid),
            value: Option::from(value),
            is_editing: Option::from(is_editing.to_owned())
        }
    }
}