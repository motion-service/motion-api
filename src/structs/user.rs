use chrono::{DateTime, Local};
use mongodb::bson::Document;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::structs::account::Account;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id")]
    pub(crate) id: ObjectId,
    pub(crate) account: Option<Account>,
    pub(crate) status: Option<StatusType>,
    pub(crate) latest_page: Option<ObjectId>,
    pub(crate) ip: Option<String>,
    pub(crate) date: Option<DateTime<Local>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBuilder {
    #[serde(rename = "_id")]
    pub(crate) id: Option<ObjectId>,
    pub(crate) account: Option<Account>,
    pub(crate) status: Option<StatusType>,
    pub(crate) latest_page: Option<ObjectId>,
    pub(crate) ip: Option<String>,
    pub(crate) date: Option<DateTime<Local>>,
}

impl UserBuilder {
    pub fn new() -> UserBuilder {
        UserBuilder {
            id: None,
            account: None,
            status: None,
            latest_page: None,
            ip: None,
            date: None,
        }
    }
    pub fn id(mut self, id: Option<ObjectId>) -> UserBuilder {
        self.id = id;
        self
    }
    pub fn account(mut self, account: Option<Account>) -> UserBuilder {
        self.account = account;
        self
    }
    pub fn status(mut self, status_type: Option<StatusType>) -> UserBuilder {
        self.status = status_type;
        self
    }
    pub fn latest_page(mut self, id: Option<ObjectId>) -> UserBuilder {
        self.latest_page = id;
        self
    }
    pub fn ip(mut self, ip: Option<String>) -> UserBuilder {
        self.ip = ip;
        self
    }
    pub fn date(mut self, date: Option<DateTime<Local>>) -> UserBuilder {
        self.date = date;
        self
    }
    pub fn build(self) -> User {
        let id = if self.id.is_none() {
            Some(Default::default())
        } else {
            self.id
        };

        User {
            id: id.to_owned().unwrap(),
            account: self.account,
            status: self.status,
            latest_page: self.latest_page,
            ip: self.ip,
            date: self.date,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatusType {
    Online,
    Offline,
    Busy,
}

impl User {
    pub fn new(account: Account, ip: String) -> Self {
        User {
            id: Default::default(),
            account: Option::from(account),
            status: None,
            latest_page: None,
            ip: Option::from(ip),
            date: Option::from(None),
        }
    }
}

impl From<Document> for User {
    fn from(doc: Document) -> Self {
        let user_id = doc.get("_id").unwrap();

        let account = doc.get("account").unwrap().as_document().unwrap();
        let account_nickname = account.get_str("name").unwrap().to_string();
        let account_password = account.get_str("password").unwrap().to_string();

        let ip = doc.get_str("ip").unwrap().to_string();

        let account = Account::new(Option::from(account_nickname), Option::from(account_password));

        User {
            id: user_id.as_object_id().unwrap(),
            account: Option::from(account),
            status: None,
            latest_page: None,
            ip: Option::from(ip),
            date: None,
        }
    }
}