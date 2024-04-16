use mongodb::bson::{Bson, Document};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prop{
    pub(crate) text_color: Option<String>,
    pub(crate) background_color: Option<String>,
    pub(crate) level: Option<String>,
}

impl From<&Document> for Prop{
    fn from(bson_doc: &Document) -> Self {

        let text_color = bson_doc.get_str("text_color").unwrap_or("");
        let background_color = bson_doc.get_str("background_color").unwrap_or("");
        let level = bson_doc.get_str("level").unwrap_or("");

        Prop{
            text_color: Some(text_color.to_string()),
            background_color: Some(background_color.to_string()),
            level: Some(level.to_string()),
        }
    }
}