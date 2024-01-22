use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelMetaData {
    pub offset: u16,
    pub is_local: bool
}