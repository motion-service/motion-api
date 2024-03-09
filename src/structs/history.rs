use chrono::{DateTime, Local};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::structs::page::Page;
use crate::structs::user::User;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionTypes {
    #[default]
    None,
    Create,
    Delete,
    Copy,
    Duplicate,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct History{
    pub(crate) _id: ObjectId,
    pub(crate) action_page: Option<Page>,
    pub(crate) action_type: Option<ActionTypes>,
    pub(crate) by: Option<User>,
}