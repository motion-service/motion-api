use mongodb::bson;
use serde_json::json;
use tracing::info;
use crate::db::database::Database;
use crate::structs::metadata::Metadata;
use crate::structs::page::{Page, PageBuilder};
use crate::structs::user::User;

pub trait MetadataRepo {
    async fn load_metadata(&self, user: User, uuid: String) -> String;
}

impl MetadataRepo for Database {
    async fn load_metadata(&self, user:
    User, uuid: String) -> String {
        let result_metadata = bson::to_document(&Metadata::new());
        match result_metadata{
            Ok(metadata_doc) => {
                let metadata_id = metadata_doc.get_object_id("_id").unwrap();
                let url = metadata_doc.get_str("url").unwrap().to_string();
                let title = metadata_doc.get_str("title").unwrap().to_string();

                let metadata = Metadata::from((Option::from(title), None, None));

                let page = PageBuilder::new().owner(user.id.to_string()).metadata(metadata).is_shared(false).index(0).build();
                let page_doc = bson::to_document(&page).unwrap();

                self.page_doc_collection.insert_one(page_doc, None).await.unwrap();

                json!({"value" : ""}).to_string()
            }

            Err(error) => {
                info!("Error on find metadata {:?}", error);

                "Cannot found metadata".to_string()
            }
        }
    }
}