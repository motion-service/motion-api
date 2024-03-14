use std::collections::HashMap;
use std::env::args;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex, MutexGuard};
use async_std::stream::StreamExt;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::Local;
use futures_util::{TryFutureExt, TryStreamExt};
use mongodb::bson::{Bson, doc, Document, Uuid};
use mongodb::{options::ClientOptions, Client, Collection, Cursor, bson};
use mongodb::{error::Error};
use mongodb::bson::Bson::ObjectId;
use serde_json::{json,Value};
use time::OffsetDateTime;
use tracing::{error, info};
use crate::{AppState, clean_string};
use crate::db::binary::BinaryUuid;

use crate::error::db_error::DbError;
use crate::error::db_error::DbError::MongoError;
use crate::structs::account::Account;
use crate::structs::metadata::Metadata;
use crate::structs::page::Page;
use crate::structs::user::User;

#[derive(Clone, Debug)]
pub struct DB {
    pub user_collection: Collection<User>,
    pub user_doc_collection: Collection<Document>,
    pub page_collection: Collection<Page>,
    pub page_doc_collection: Collection<Document>
}

//TODO https://doc.rust-lang.org/book/ch01-01-installation.html

pub type Result<T> = std::result::Result<T, DbError>;

impl DB {
    pub async fn init() -> Result<Self> {
        let mongodb_uri = std::env::var("DATABASE_URL").unwrap();
        let database_name = std::env::var("MONGO_INIT_DB_DATABASE").unwrap();
        let user_collection_name = std::env::var("MONGODB_USER_COLLECTION").unwrap();
        let page_collection_name = std::env::var("MONGODB_PAGE_COLLECTION").unwrap();

        let mut client_options = ClientOptions::parse(mongodb_uri).await?;
        client_options.app_name = Some(database_name.to_string());

        let client = Client::with_options(client_options)?;
        let database = client.database(database_name.as_str());

        let user_collection = database.collection(user_collection_name.as_str());
        let collection = database.collection::<Document>(user_collection_name.as_str());

        let page_collection = database.collection(page_collection_name.as_str());
        let page_doc_collection = database.collection::<Document>(page_collection_name.as_str());

        info!("✅ Database connected successfully");

        Ok(Self {
            user_collection: user_collection.clone(),
            user_doc_collection: collection,
            page_collection: page_collection.clone(),
            page_doc_collection: page_doc_collection.clone(),
        })
    }
    pub async fn load_all_user(&self) -> Vec<User> {
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

    pub async fn edit_page(&self, url: Uuid) -> (StatusCode, String){
        let filter = doc! {"metadata.url": url.clone()};
        let doc_result = self.page_doc_collection.find_one(filter.clone(), None).await.expect("TODO: panic message");

        match doc_result {
            None => {

                (
                    StatusCode::NOT_FOUND,
                    "Cannot found page".to_string()
                )
            }
            Some(_) => {
                let filter = doc! {"metadata.url": url.clone()};
                let doc_result = self.page_doc_collection.find_one(filter.clone(), None).await.expect("TODO: panic message");

                match doc_result {
                    None => {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Found Page".to_string()
                        )
                    }
                    Some(_) => {
                        (
                            StatusCode::OK,
                            "Found Page".to_string()
                        )
                    }
                }
            }
        }
    }

    async fn load_metadata(&self, user: User, uuid: String) -> (StatusCode, String){
        let result_metadata = bson::to_document(&Metadata::new(uuid.to_owned()));
        match result_metadata{
            Ok(metadata) => {
                self.page_doc_collection.insert_one(doc! {
                                            "owner": user.uuid.unwrap(),
                                            "metadata": metadata,
                                            "isShared": false,
                                            "created": "now"
                                        }, None).await.unwrap();

                (
                    StatusCode::OK,
                    json!({"test":""}).to_string()
                )
            }
            Err(error) => {
                info!("Error on find metadata{:?}", error);

                (
                    StatusCode::FAILED_DEPENDENCY,
                    "Cannot found metadata".to_string()
                )
            }
        }
    }

    async fn load_user(&self, uuid_string: String) -> Option<User> {
        self.user_collection.find_one(doc! {"uuid": uuid_string}, None).await.unwrap()
    }

    pub async fn create_new_page(&self, uuid: String) -> (StatusCode, String){
        let uuid_string = uuid.clone();
        let parse_result = uuid::Uuid::parse_str(&uuid_string.clone());

        match parse_result {
            Ok(_) => {
                let user = self.load_user(uuid).await.or_else(|| {None}).unwrap();
                let uuid = uuid::Uuid::new_v4().to_string();

                self.load_metadata(user, uuid.to_owned()).await
            }

            Err(_) => {
                (
                    StatusCode::NO_CONTENT,
                    "Something went wrong:".to_string()
                )
            }
        }
    }

    pub async fn load_page(&self, uuid: String) -> Option<Document> {
        let mut result = self.page_doc_collection.find_one(
            doc! {
            "metadata.url": uuid
        },
            None
        ).await.unwrap();

        result
    }

    pub async fn load_all_pages(&self) -> HashMap<String, Vec<Page>> {
        let mut cursor = self.page_doc_collection.find(doc! {}, None).await.unwrap();
        let mut page_map: HashMap<String, Vec<Page>> = HashMap::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => {
                    let owner = clean_string!(doc.get("owner").unwrap().to_string());
                    let page_id = doc.get("_id").unwrap().as_object_id().unwrap();
                    let is_shared = doc.get("isShared").unwrap().as_bool();
                    let created = clean_string!(doc.get("created").unwrap().to_string());

                    let metadata_doc = doc.get("metadata").unwrap().as_document().unwrap();
                    let metadata_id = metadata_doc.get("_id").unwrap().as_object_id().unwrap();


                    let metadata_url = clean_string!(metadata_doc.get("url").unwrap().to_string());
                    let metadata_title = clean_string!(metadata_doc.get("title").unwrap().to_string());

                    let metadata_obj = Metadata {
                        id: metadata_id,
                        url: Option::from(metadata_url),
                        title: Option::from(metadata_title),
                        parent: None,
                        children: None,
                    };

                    let page = Page {
                        _id: page_id,
                        owner: Option::from(owner.to_owned()),
                        metadata: Option::from(metadata_obj),
                        date: Option::from(created.to_owned()),
                        is_shared: is_shared.to_owned(),
                    };

                    let pages_for_owner = page_map.entry(owner.to_owned()).or_insert(Vec::new());
                    pages_for_owner.push(page);
                }
                Err(e) => {
                    error!("Error processing page: {:?}", e);
                    // Handle error if needed
                }
            }
        }

        page_map
    }
}