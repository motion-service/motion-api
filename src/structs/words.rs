use chrono::{DateTime, Local};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::structs::styles::Style;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Word{
    pub(crate) text: Option<String>,
    pub(crate) style: Option<Vec<Style>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WordBuilder{
    pub(crate) text: Option<String>,
    pub(crate) style: Option<Vec<Style>>,
}

impl WordBuilder{
    pub fn new() -> Self{
        WordBuilder{
            text: None,
            style: None,
        }
    }

    pub fn text(mut self, text: String) -> Self{
        self.text = Some(text);
        self
    }

    pub fn style(mut self, style: Vec<Style>) -> Self{
        self.style = Some(style);
        self
    }

    pub fn build(self) -> Word{
        Word{
            text: self.text,
            style: self.style,
        }
    }
}