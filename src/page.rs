use serde::{
    Deserialize,
    Serialize,
};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Page {
    pub id: String,
    pub fileName: String,
}
