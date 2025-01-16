use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub id: i32,
    pub name: String,
    pub link: String,
}
