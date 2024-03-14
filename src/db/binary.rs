use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinaryUuid {
    pub(crate) uuid: Option<String>
}

impl BinaryUuid{
    pub fn new(uuid: String) -> Self{
        BinaryUuid{
            uuid: Option::from(uuid)
        }
    }

    pub fn to_string(&self) -> String{
        self.to_owned().uuid.unwrap().to_string()
    }
}