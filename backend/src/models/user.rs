use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub fullname: String,
    pub description: String,
    pub about: String,
}
