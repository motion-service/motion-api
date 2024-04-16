
use std::collections::HashMap;

use mongodb::{options::ClientOptions, Client, Collection, Cursor, bson};
use mongodb::bson::Document;

use serde_json::{json,Value};
use tracing::{info};
use crate::{AppState};

use crate::structs::default_block::DefaultBlock;
use crate::structs::page::Page;
use crate::structs::user::User;

#[derive(Clone, Debug)]
pub struct Database {
    pub user_collection: Collection<User>,
    pub user_doc_collection: Collection<Document>,
    pub page_collection: Collection<Page>,
    pub page_doc_collection: Collection<Document>,
    pub block_collection: Collection<DefaultBlock>,
    pub block_doc_collection: Collection<Document>
}

impl Database {
    pub async fn init() -> Self{
        let mongodb_uri = std::env::var("DATABASE_URL").unwrap();
        let database_name = std::env::var("MONGO_INIT_DB_DATABASE").unwrap();
        let user_collection_name = std::env::var("MONGODB_USER_COLLECTION").unwrap();
        let page_collection_name = std::env::var("MONGODB_PAGE_COLLECTION").unwrap();
        let block_collection_name = std::env::var("MONGODB_BLOCK_COLLECTION").unwrap();

        let mut client_options = ClientOptions::parse(mongodb_uri).await.unwrap();
        client_options.app_name = Some(database_name.to_string());

        let client = Client::with_options(client_options).unwrap();
        let database = client.database(database_name.as_str());

        let user_collection = database.collection(user_collection_name.as_str());
        let collection = database.collection::<Document>(user_collection_name.as_str());

        let page_collection = database.collection(page_collection_name.as_str());
        let page_doc_collection = database.collection::<Document>(page_collection_name.as_str());

        let block_collection = database.collection(block_collection_name.as_str());
        let block_doc_collection = database.collection::<Document>(block_collection_name.as_str());

        info!("✅ Database connected successfully");

        Self {
            user_collection: user_collection.clone(),
            user_doc_collection: collection,
            page_collection: page_collection.clone(),
            page_doc_collection: page_doc_collection.clone(),
            block_collection: block_collection.clone(),
            block_doc_collection: block_doc_collection.clone()
        }
    }
}