use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::structs::metadata::Metadata;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub(crate) rows: Option<Vec<Row>>,
    pub(crate) cells: Option<Vec<Cell>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row(String);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cell(String);