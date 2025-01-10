use std::sync::{Arc, Mutex};

use axum::{extract::State, Json};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::models::work_experience::WorkExperience;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewWorkExperience {
    company: String,
    position: String,
    period: String,
    description: String,
}

pub async fn add_work_experience(
    State(db): State<Arc<Mutex<Connection>>>,
    Json(payload): Json<NewWorkExperience>,
) -> Result<Json<WorkExperience>, String> {
    let conn = db.lock().map_err(|_| "Failed to acquire database lock")?;

    // INSERT işlemi
    let mut stmt = conn
        .prepare("INSERT INTO work_experience (company, position, period, description) VALUES (?, ?, ?, ?)")
        .map_err(|_| "Failed to prepare INSERT statement")?;

    stmt.execute(&[
        &payload.company,
        &payload.position,
        &payload.period,
        &payload.description,
    ])
    .map_err(|_| "Failed to execute INSERT statement")?;

    // Yeni eklenen satırın ID'sini al
    let new_id = conn.last_insert_rowid() as i32;

    // Yeni eklenen kaydı SELECT ile al
    let mut stmt = conn
        .prepare(
            "SELECT id, company, position, period, description FROM work_experience WHERE id = ?",
        )
        .map_err(|_| "Failed to prepare SELECT statement")?;

    let experience = stmt
        .query_row([new_id], |row| {
            Ok(WorkExperience {
                id: row.get("id")?,
                company: row.get("company")?,
                position: row.get("position")?,
                period: row.get("period")?,
                description: row.get("description")?,
            })
        })
        .map_err(|e| format!("Failed to fetch new work experience data: {:?}", e))?;

    Ok(Json(experience))
}

pub async fn get_work_experience(
    State(db): State<Arc<Mutex<Connection>>>,
) -> Json<Vec<WorkExperience>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, company, position, period, description FROM work_experience")
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
