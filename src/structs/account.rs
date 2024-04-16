use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account{
    pub(crate) name: Option<String>,
    pub(crate) password: Option<String>,
}

impl Account {
    pub fn new(name: Option<String>, email: Option<String>) -> Self{
        Account{
            name: name.to_owned(),
            password: email.to_owned()
        }
    }
}