#![allow(non_snake_case)]
#![allow(dead_code)]
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Page {
    pub id: String,
    pub fileName: String,
}
