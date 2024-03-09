use chrono::{DateTime, Local};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::structs::account::Account;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User{
    #[serde(rename = "_id")]
    pub(crate) id: ObjectId,
    pub(crate) uuid: Option<String>,
    pub(crate) account: Option<Account>,
    pub(crate) ip: Option<String>,
    pub(crate) date: Option<DateTime<Local>>,
}

impl User{
    pub fn new(uuid: String, account: Account, ip: String) -> Self{
        User{
            id: Default::default(),
            uuid: Option::from(uuid),
            account: Option::from(account),
            ip: Option::from(ip),
            date: Option::from(None)
        }
    }
}