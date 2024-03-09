use chrono::{DateTime, Local};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::structs::block::BlockBuilder;
use crate::structs::user::User;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page{
    pub(crate) _id: ObjectId,
    pub(crate) owner: Option<User>,
    pub(crate) blocks: Option<Vec<BlockBuilder>>,
    pub(crate) date: Option<DateTime<Local>>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageData{
    pub(crate) owner: Option<User>,
    pub(crate) blocks: Option<Vec<BlockBuilder>>,
    pub(crate) date: Option<DateTime<Local>>
}

impl Page{

}