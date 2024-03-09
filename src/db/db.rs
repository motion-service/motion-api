use std::net::IpAddr;
use std::str::FromStr;
use axum::http::StatusCode;
use axum::Json;
use chrono::Local;
use futures_util::{TryFutureExt, TryStreamExt};
use mongodb::bson::{Bson, doc, Document, Uuid};
use mongodb::{options::ClientOptions, Client, Collection, Cursor};
use mongodb::bson::Bson::ObjectId;
use mongodb::bson::oid::Error;
use serde_json::{json,Value};
use tracing::{info};

use crate::error::db_error::DbError;
use crate::structs::account::Account;
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

type Result<T> = std::result::Result<T, DbError>;

impl DB {
    pub async fn init() -> Result<Self> {
        let mongodb_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let database_name =
            std::env::var("MONGO_INIT_DB_DATABASE").expect("MONGO_INITDB_DATABASE must be set.");
        let user_collection_name =
            std::env::var("MONGODB_USER_COLLECTION").expect("MONGODB_NOTE_COLLECTION must be set.");

        let page_collection_name =
            std::env::var("MONGODB_PAGE_COLLECTION").expect("MONGODB_NOTE_COLLECTION must be set.");

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

    pub async fn load_page(&self, uuid: String) -> Option<Document> {
        let mut result = self.page_doc_collection.find_one(
            doc! {
            "metadata.url": uuid
        },
            None
        ).await.unwrap();

        result
    }

    pub async fn load_all_pages(&self, uuid: String) -> Value {
        let mut cursor = self.page_doc_collection.find(
            doc! {
            "owner": uuid
        },
            None
        ).await.unwrap();

        let mut pages: Vec<Option<Bson>> = vec![];

        while let Ok(Some(doc)) = cursor.try_next().await {
            let metadata = doc.get("metadata").cloned(); // Cloning metadata
            pages.push(metadata)
        }

        let json_array: Vec<Value> = pages
            .into_iter()
            .map(|opt_bson| {
                opt_bson.map(|bson| {
                    serde_json::to_value(bson).unwrap_or(json! ({}))
                }).unwrap_or(json!({}))
            })
            .collect();

        json!(json_array)
    }
}