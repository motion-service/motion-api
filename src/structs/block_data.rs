use serde::{Deserialize, Serialize};
use crate::structs::styles::TextDecorationType;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockData {
    pub(crate) uuid: Option<String>,
    pub(crate) status: Option<bool>,
    #[serde(rename = "block_type")]
    pub(crate) block_type: Option<TextDecorationType>,
    pub(crate) text: Option<String>,
    pub(crate) index: Option<usize>
}

impl From<(Option<String>, Option<bool>, Option<String>, Option<usize>)> for BlockData{
    fn from((uuid, status, text, index): (Option<String>, Option<bool>, Option<String>, Option<usize>)) -> Self {
        BlockData{
            uuid: uuid.to_owned(),
            status: status.to_owned(),
            block_type: None,
            text: text.to_owned(),
            index: index.to_owned(),
        }
    }
}