use chrono::{DateTime, Local};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::structs::user::User;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Area{
    pub(crate) _id: ObjectId,
    pub(crate) ban_members: Option<User>,
    pub(crate) ban_guests: Option<User>,
    pub(crate) date: Option<DateTime<Local>>
}