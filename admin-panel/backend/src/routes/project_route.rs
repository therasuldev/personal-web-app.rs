use std::sync::{Arc, Mutex};

use axum::{extract::State, Json};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::models::project::Project;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewProject {
    name: String,
    description: String,
    link: String,
}

pub async fn add_project(
    State(db): State<Arc<Mutex<Connection>>>,
    Json(payload): Json<NewProject>,
) -> Result<Json<Project>, String> {
    let conn = db.lock().map_err(|_| "Failed to acquire database lock")?;

    let mut stmt = conn
        .prepare("INSERT INTO projects (name, description, link) VALUES (?, ?, ?)")
        .map_err(|_| "Failed to prepare INSERT statement")?;

    stmt.execute(&[&payload.name, &payload.description, &payload.link])
        .map_err(|_| "Failed to execute INSERT statement")?;

    let new_id = conn.last_insert_rowid() as i32;

    let mut stmt = conn
        .prepare("SELECT id, name, description, link FROM projects WHERE id = ?")
        .map_err(|_| "Failed to prepare SELECT statement")?;

    let project = stmt
        .query_row([new_id], |row| {
            Ok(Project {
                id: row.get("id")?,
                name: row.get("name")?,
                description: row.get("description")?,
                link: row.get("link")?,
            })
        })
        .map_err(|e| format!("Failed to fetch new project data: {:?}", e))?;

    Ok(Json(project))
}
