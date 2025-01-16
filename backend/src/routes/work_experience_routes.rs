use crate::models::work_experience::WorkExperience;
use axum::{extract::State, Json};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub async fn get_work_experience(
    State(db): State<Arc<Mutex<Connection>>>,
) -> Json<Vec<WorkExperience>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, company, position, period, description FROM work_experience ORDER BY id DESC")
        .unwrap();
    let experience_iter = stmt
        .query_map([], |row| {
            Ok(WorkExperience {
                id: row.get(0)?,
                company: row.get(1)?,
                position: row.get(2)?,
                period: row.get(3)?,
                description: row.get(4)?,
            })
        })
        .unwrap();

    let mut experiences = Vec::new();
    for experience in experience_iter {
        experiences.push(experience.unwrap());
    }

    Json(experiences)
}
