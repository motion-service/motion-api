use std::str::FromStr;
use std::sync::Arc;
use async_std::prelude::StreamExt;
use mongodb::bson::{doc, Document, oid};
use mongodb::bson::oid::ObjectId;
use tracing::{debug, error, info};
use crate::db::database::Database;
use crate::repository::block_repository::BlockRepository;
use crate::structs::content::{BlockType, Content};
use crate::structs::default_block::DefaultBlock;
use crate::structs::metadata::{BinaryMetadata, Metadata};
use crate::structs::page::{Page, PageBuilder};

pub trait PageRepo {
    async fn create_new_page(&self, uuid: String);
    async fn edit_page(&self, id: String, title: String);
    async fn load_page(&self, uuid: String) -> Option<Document>;
    async fn load_all_pages(&self, uuid: String) -> Vec<Page>;
    async fn find_page(&self, uuid: String) -> (Option<Document>, Document);
}

impl PageRepo for Database {
    async fn create_new_page(&self, uuid: String){
        let page = PageBuilder::new().owner(uuid).metadata(Metadata::new()).build();
        &self.page_collection.insert_one(page, None).await.unwrap();
    }

    async fn edit_page(&self, id: String, title: String) {
        let id = ObjectId::from_str(id.as_str()).unwrap();
        let filter = doc! {"_id": id};
        let update = doc! {"$set": {"metadata.title": title}};

        self.page_collection.update_one(filter, update, None).await.unwrap();
    }

    async fn load_page(&self, uuid: String) -> Option<Document> {
        let mut result = self.page_doc_collection.find_one(
            doc! {
                "metadata.url": uuid
            },
            None
        ).await.unwrap();

        result
    }

    async fn load_all_pages(&self, uuid: String) -> Vec<Page> {
        let mut cursor = self.page_doc_collection.find(doc! {"owner": uuid.clone()}, None).await.unwrap();
        let mut page_map: Vec<Page> = vec![];

        while let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => {
                    let owner = doc.get_str("owner").unwrap();
                    let page_id = doc.get("_id").unwrap().as_object_id().unwrap();

                    let metadata_doc = doc.get("metadata").unwrap().as_document().unwrap();
                    let metadata_title = metadata_doc.get_str("title").unwrap();

                    let metadata_obj = Metadata {
                        title: Option::from(metadata_title.to_string()),
                        parent: None,
                        children: None,
                    };

                    let page = PageBuilder::new().id(page_id).owner(owner.to_string()).metadata(metadata_obj)
                                                        .is_shared(false).index(1).build();
                    page_map.push(page);
                }

                Err(e) => {
                    error!("Error processing page: {:?}", e);
                }
            }
        }

        page_map
    }

    async fn find_page(&self, uuid: String) -> (Option<Document>, Document) {
        let filter = doc! {"_id": ObjectId::from_str(uuid.as_str()).ok().unwrap()};

        (self.page_doc_collection.find_one(filter.clone(), None).await.unwrap(), filter)
    }
}