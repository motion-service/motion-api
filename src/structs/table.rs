use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::structs::metadata::Metadata;
use crate::structs::styles::{Color, Style, TextAlignment};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub(crate) rows: Option<Vec<Row>>,
    pub(crate) cells: Option<Vec<Cell>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableStyle{
    pub(crate) width: Option<usize>,
    pub(crate) text_style: Option<Style>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row{
    pub(crate) table_style: TableStyle
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cell{
    pub(crate) table_style: TableStyle
}
