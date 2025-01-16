use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub link: String,
}
