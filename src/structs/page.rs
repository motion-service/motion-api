use chrono::{DateTime, Local};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::structs::metadata::Metadata;
use crate::structs::user::User;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page{
    pub(crate) _id: ObjectId,
    pub(crate) owner: Option<String>,
    pub(crate) metadata: Option<Metadata>,
    pub(crate) date: Option<String>,
    pub(crate) is_shared: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageData{
    pub(crate) owner: Option<User>,
    pub(crate) date: Option<DateTime<Local>>
}

impl Page{

}