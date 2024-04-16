use std::default::Default;
use std::str::FromStr;
use mongodb::bson;
use mongodb::bson::oid;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::structs::content::{BlockType, Content};
use crate::structs::prop::Prop;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultBlock {
    #[serde(rename = "_id")]
    pub(crate) id: ObjectId,
    pub(crate) page_id: Option<String>,
    pub(crate) content: Option<Content>,
    pub(crate) props: Option<Prop>,
    pub(crate) is_editing: Option<bool>,
    pub(crate) index: Option<usize>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultBlockBuilder {
    #[serde(rename = "_id")]
    pub(crate) id: ObjectId,
    pub(crate) page_id: Option<String>,
    pub(crate) content: Option<Content>,
    pub(crate) props: Option<Prop>,
    pub(crate) is_editing: Option<bool>,
    pub(crate) index: Option<usize>
}

impl DefaultBlockBuilder{
    pub fn new() -> Self{
        DefaultBlockBuilder{
            id: Default::default(),
            page_id: None,
            content: None,
            props: None,
            is_editing: None,
            index: None,
        }
    }

    pub fn doc_id(mut self, id: ObjectId) -> Self{
        self.id = id;
        self
    }

    pub fn page_id(mut self, page_id: String) -> Self{
        self.page_id = Some(page_id);
        self
    }

    pub fn content(mut self, content: Content) -> Self{
        self.content = Some(content);
        self
    }

    pub fn props(mut self, props: Prop) -> Self{
        self.props = Some(props);
        self
    }

    pub fn is_editing(mut self, is_editing: bool) -> Self{
        self.is_editing = Some(is_editing);
        self
    }
    pub fn index(mut self, index: usize) -> Self{
        self.index = Some(index);
        self
    }

    pub fn build(self) -> DefaultBlock{
        DefaultBlock{
            id: self.id,
            page_id: self.page_id,
            content: self.content,
            props: self.props,
            is_editing: self.is_editing,
            index: self.index,
        }
    }
}