use axum::{extract::State, Json};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
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
