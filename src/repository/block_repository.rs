use std::str::FromStr;
use async_std::prelude::StreamExt;
use mongodb::bson;
use mongodb::bson::{Bson, doc, Document, oid, to_bson, to_document};
use mongodb::bson::document::ValueAccessResult;
use mongodb::bson::oid::ObjectId;

use tracing::info;
use crate::db::database::Database;
use crate::repository::page_repository::PageRepo;
use crate::structs::block_data::BlockData;
use crate::structs::content::{BlockType, Content, ContentBuilder};
use crate::structs::default_block::{DefaultBlock, DefaultBlockBuilder};
use crate::structs::prop::Prop;
use crate::structs::styles;
use crate::structs::styles::{Style, StyleBuilder, TextDecorationType};
use crate::structs::words::{Word, WordBuilder};

pub trait BlockRepository {
    async fn add_new_block(&self, uuid: String);
    async fn delete_block(&self, index: usize, uuid: String);
    async fn edit_block_styles(&self, block_data: BlockData);
    async fn load_all_block(&self, uuid: String) -> Vec<DefaultBlock>;
}

impl BlockRepository for Database {
    async fn add_new_block(&self, uuid: String){
        let (page, _) = self.find_page(uuid.clone()).await;

        match page {
            None => {
                // 페이지를 찾을 수 없는 경우 처리할 내용

            }
            Some(page_doc) => {
                let page_doc_id = page_doc.get_object_id("_id").unwrap().to_string();
                let style = StyleBuilder::new().build();

                let word = WordBuilder::new().text(String::from("")).style(Vec::new()).build();
                let content = ContentBuilder::new().block_type(BlockType::Text).words(word).build();

                let default_block = DefaultBlockBuilder::new().page_id(page_doc_id).content(content)
                                                                    .props(Prop::default()).is_editing(false).index(0).build();

                self.block_doc_collection.insert_one(to_document(&default_block).unwrap(),None).await.unwrap();
            }
        }
    }

    async fn delete_block(&self, index: usize, uuid: String) {
        let (page, filter) = self.find_page(uuid.clone()).await;

        match page {
            None => {
                // 페이지를 찾을 수 없는 경우 처리할 내용

            }

            Some(mut page_doc) => {
                let mut blocks = page_doc.get_array_mut("blocks").unwrap();
                let update = doc! {"$set": {"blocks": blocks.clone()}};

                blocks.remove(index);

                self.page_collection.update_one(filter, update, None).await.unwrap();
                info!("Added new block in {:?}", uuid);
            }
        }
    }

    async fn edit_block_styles(&self, block_data: BlockData) {
        todo!()
    }

    async fn load_all_block(&self, uuid: String) -> Vec<DefaultBlock> {
        let mut block_map: Vec<DefaultBlock> = Vec::new();
        let mut cursor = self.block_doc_collection.find(doc! {"pageId": uuid}, None).await.unwrap();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => {
                    let object_id = doc.get_str("pageId").unwrap();

                    let content_bson = doc.get("content").unwrap();
                    let content_doc = content_bson.as_document().unwrap();
                    let content = Content::from(content_doc);

                    let props_bson  = doc.get("props").unwrap().as_document().unwrap();
                    let prop = Prop::from(props_bson);

                    let default_block = DefaultBlockBuilder::new().page_id(object_id.to_string()).content(content).props(Prop::default()).is_editing(false).build();

                    block_map.push(default_block);
                }
                _ => {

                }
            }
        }
        block_map
    }
}