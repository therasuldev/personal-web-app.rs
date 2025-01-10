use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkExperience {
    pub id: i32,
    pub company: String,
    pub position: String,
    pub period: String,
    pub description: String,
}
