use chrono::{DateTime, Local};
use mongodb::bson::Document;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::structs::account::Account;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User{
    #[serde(rename = "_id")]
    pub(crate) id: ObjectId,
    pub(crate) account: Option<Account>,
    pub(crate) ip: Option<String>,
    pub(crate) date: Option<DateTime<Local>>,
}

impl User{
    pub fn new(account: Account, ip: String) -> Self{
        User{
            id: Default::default(),
            account: Option::from(account),
            ip: Option::from(ip),
            date: Option::from(None)
        }
    }
}

impl From<Document> for User{
    fn from(doc: Document) -> Self {
        let user_id = doc.get("_id").unwrap();

        let account = doc.get("account").unwrap().as_document().unwrap();
        let account_nickname = account.get_str("name").unwrap().to_string();
        let account_password = account.get_str("password").unwrap().to_string();

        let ip = doc.get_str("ip").unwrap().to_string();

        let account = Account::new(Option::from(account_nickname), Option::from(account_password));

        User{
            id: user_id.as_object_id().unwrap(),
            account: Option::from(account),
            ip: Option::from(ip),
            date: None,
        }
    }
}