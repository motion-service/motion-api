use std::collections::HashMap;
use std::ops::Range;
use mongodb::bson::{doc, to_bson};
use mongodb::bson::oid::ObjectId;
use range_compare::{RangeCmpResult, RangeExt};
use rangemap::RangeMap;
use tracing::info;
use crate::db::database::Database;
use crate::repository::block_repository::BlockRepository;
use crate::structs::content;
use crate::structs::content::{BlockType, Content};
use crate::structs::default_block::DefaultBlock;
use crate::structs::styles::{Style, ToStyle};

pub trait RepoStyle{
    // async fn update_style(&self, page_uuid: ObjectId, index: u32, start: usize, end: usize, style: Styles, text: String);
    // async fn load_style(&self, page_uuid: ObjectId, index: u32, start: usize, end: usize, style: Styles, text: String) -> Vec<Content>;
    async fn remove_style(&self, uuid: ObjectId) -> Vec<DefaultBlock>;
    async fn add_style(&self, uuid: ObjectId) -> Vec<DefaultBlock>;
    async fn for_each(&self, uuid: ObjectId) -> Vec<Style>;

}

impl RepoStyle for Database{
    async fn remove_style(&self, uuid: ObjectId) -> Vec<DefaultBlock> {
        todo!()
    }

    async fn add_style(&self, uuid: ObjectId) -> Vec<DefaultBlock> {
        todo!()
    }

    async fn for_each(&self, uuid: ObjectId) -> Vec<Style> {
        todo!()
    }
    // async fn update_style(&self, page_uuid: ObjectId, index: u32, start: usize, end: usize, style: Styles, text: String){
    //     let mut default_block =  self.get_block_at(page_uuid.to_string(), index).await.unwrap();
    //     let contents = default_block.clone().content.unwrap();
    //
    // }
    //
    // async fn load_style(&self, page_uuid: ObjectId, index: u32, start: usize, end: usize, style: Styles, text: String) -> Vec<Content> {
    //     let mut default_block =  self.get_block_at(page_uuid.to_string(), index).await.unwrap();
    //
    //     let mut contents = default_block.clone().content.unwrap();
    //     let mut new_contents = default_block.content.unwrap();
    //
    //     vec![]
    // }
}