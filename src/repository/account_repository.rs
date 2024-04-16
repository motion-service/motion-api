use std::str::FromStr;
use async_std::prelude::StreamExt;
use mongodb::bson::{doc, oid};
use tracing::info;
use crate::db::database::Database;
use crate::structs::account::Account;
use crate::structs::user::User;

pub trait AccountRepository {
    async fn is_already_existed_account(&self, account: Account) -> Option<User>;
    async fn register_account(&self, user: User);
    async fn delete_account(&self, account: Account);
    async fn load_all_user(&self) -> Vec<User>;
    async fn load_user(&self, uuid_string: String) -> Option<User>;
}

impl AccountRepository for Database {
    async fn is_already_existed_account(&self, account: Account) -> Option<User> {
        let name = account.name;
        match name {
            None => {
                None
            }
            Some(name) => {
                self.user_collection.find_one(doc! {"account.name": name}, None).await.unwrap()
            }
        }
    }

    async fn register_account(&self, user: User) {
        self.user_collection.insert_one(user, None).await.unwrap();
    }

    async fn delete_account(&self, account: Account) {
        self.user_collection.delete_one(doc! {"account.name": account.name.unwrap()}, None).await.unwrap();
    }

    async fn load_all_user(&self) -> Vec<User> {
        let mut cursor = self.user_doc_collection.find(doc!{}, None).await.unwrap();
        let mut users: Vec<User> = vec![];

        while let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => {
                    users.push(User::from(doc));
                }

                Err(e) => {
                    info!("error occurred while loading all users error : {:?} ", e)
                }
            }
        }

        users
    }

    async fn load_user(&self, uuid_string: String) -> Option<User>{
        let id = oid::ObjectId::from_str(uuid_string.clone().as_str()).unwrap();
        self.user_collection.find_one(doc! {"_id": id}, None).await.unwrap()
    }
}