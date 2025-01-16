use crate::models::project::Project;
use axum::{extract::State, Json};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub async fn get_projects(State(db): State<Arc<Mutex<Connection>>>) -> Json<Vec<Project>> {
    let conn = db.lock().unwrap();

    let mut stmt = conn
        .prepare("SELECT id, name, description, link FROM projects")
        .unwrap();
    let project_iter = stmt
        .query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                link: row.get(3)?,
            })
        })
        .unwrap();

    let mut projects = Vec::new();
    for project in project_iter {
        projects.push(project.unwrap());
    }

    Json(projects)
}
