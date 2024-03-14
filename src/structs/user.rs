use chrono::{DateTime, Local};
use mongodb::bson::Document;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::clean_string;
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


impl From<Document> for User{
    fn from(doc: Document) -> Self {
        let user_id = doc.get("_id").unwrap();
        let uuid = clean_string!(doc.get("uuid").unwrap().to_string());

        let account = doc.get("account").unwrap().as_document().unwrap();
        let account_id = account.get("_id").unwrap();
        let account_nickname = clean_string!(account.get("nickname").unwrap().to_string());
        let account_password = clean_string!(account.get("password").unwrap().to_string());

        let ip = clean_string!(doc.get("ip").unwrap().to_string());
        let date = doc.get("date").unwrap().to_string();

        let new_account: Account = Account{
            id: account_id.as_object_id().unwrap(),
            nickname: Option::from(account_nickname),
            password: Option::from(account_password),
        };

        User{
            id: user_id.as_object_id().unwrap(),
            uuid: Option::from(uuid.to_owned()),
            account: Option::from(new_account),
            ip: Option::from(ip),
            date: None,
        }
    }
}