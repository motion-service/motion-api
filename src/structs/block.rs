
use std::default::Default;
use std::result;
use axum::http::Error;
use chrono::{DateTime, Local};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};
use crate::structs::user::User;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockBuilder {
    pub(crate) _id: ObjectId,
    pub(crate) uuid: Option<String>,
    pub(crate) value: Option<String>,
    pub(crate) isEditing: Option<bool>
}

pub type Result<T> = result::Result<T, Error>;

//TODO Smart Menu
#[derive(Debug, Clone,Eq, PartialEq, Serialize, Deserialize,Hash)]
enum BlockType {
    Default, List, Table, Image, Audio,
}

use BlockType::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Block(BlockType);

impl BlockType{
    pub const DEFAULT: Block = Block(Default);

    pub const LIST: Block = Block(List);

    pub const TABLE: Block = Block(Table);

    pub const IMAGE: Block = Block(Image);

    pub const AUDIO: Block = Block(Audio);
}

impl BlockBuilder {
    pub fn new() -> BlockBuilder{
        BlockBuilder::default()
    }
}