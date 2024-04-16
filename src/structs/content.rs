use std::ops::Range;
use mongodb::bson::{Bson, Document};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::structs::styles::{Color, Style, StyleBuilder};

use crate::structs::words::{Word, WordBuilder};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content{
    pub(crate) block_type: Option<BlockType>,
    pub(crate) words: Option<Word>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentBuilder{
    pub(crate) block_type: Option<BlockType>,
    pub(crate) words: Option<Word>,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlockType{
    Table, Text, Paragraph
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Err{
    NoneType
}

impl BlockType{
    pub fn from_str(input: &str) -> Result<BlockType, Err>{
        match input {
            "Text" => Ok(BlockType::Text),
            "Table" => Ok(BlockType::Table),
            "Paragraph" => Ok(BlockType::Paragraph),

            _ => {
                Err(Err::NoneType)
            }
        }
    }
}

impl From<(&Document)> for Content{
    fn from(value: (&Document)) -> Self {
        let blockType = value.get_str("blockType").ok().unwrap();

        let block_type = BlockType::from_str(blockType).ok();
        let word_doc = value.get("words").unwrap().as_document().unwrap();
        let text = word_doc.get_str("text").ok();

        let word = {
            let mut styles:Vec<Style> = Vec::new();
            let style = word_doc.get_array("style").ok().unwrap();
            style.iter().for_each(|style_bson|{
                let style_doc = style_bson.as_document().unwrap();

                let range_doc = style_doc.get("range").unwrap().as_document().unwrap();
                let start = range_doc.get_i64("start").ok().unwrap();
                let end = range_doc.get_i64("end").ok().unwrap();

                let bold = style_doc.get_bool("bold").ok();
                let italic = style_doc.get_bool("italic").ok();
                let underline = style_doc.get_bool("underline").ok();
                let strike = style_doc.get_bool("strike").ok();

                let range =  start as usize..end as usize;
                let range = Some(Range::from(range));

                styles.push(StyleBuilder::new().bold(bold).italic(italic).underline(underline).strike(strike).range(range).build());
            });

            WordBuilder::new().text(text.unwrap().to_string()).style(styles).build()
        };

        Content{
            block_type: Some(block_type.to_owned().unwrap()),
            words: Some(word),
        }
    }
}

impl ContentBuilder{
    pub fn new() -> Self{
        ContentBuilder{
            block_type: None,
            words: None,
        }
    }

    pub fn block_type(mut self, block_type: BlockType) -> Self{
        self.block_type = Some(block_type);
        self
    }

    pub fn words(mut self, word: Word) -> Self{
        self.words = Some(word);
        self
    }

    pub fn build(self) -> Content {
        Content{
            block_type: self.block_type,
            words: self.words,
        }
    }
}