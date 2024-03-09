use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Account{
//     pub(crate) name: Option<String>,
//     pub(crate) email: Option<String>,
//     pub(crate) image: Option<String>
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account{
    #[serde(rename = "_id")]
    pub(crate) id: ObjectId,
    pub(crate) nickname: Option<String>,
    pub(crate) password: Option<String>,
}


impl Account {
    pub fn new (nickname: Option<String>, password: Option<String>)-> Self{
        Account{
            id: Default::default(),
            nickname: nickname.to_owned(),
            password: password.to_owned()
        }
    }
}
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct AccountDoc{
//     #[serde(rename = "_id")]
//     pub(crate) id: ObjectId,
//     pub(crate) name: Option<String>,
//     pub(crate) email: Option<String>,
//     pub(crate) image: Option<String>
// }