use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Page {
    pub id: String,
    pub fileName: String,
}
