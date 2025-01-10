use axum::{extract::State, Json};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

use crate::models::user::User;

pub async fn get_me(State(db): State<Arc<Mutex<Connection>>>) -> Result<Json<User>, String> {
    let conn = db.lock().map_err(|_| "Failed to acquire database lock")?;

    let mut stmt = conn
        .prepare("SELECT fullname, description, about FROM users LIMIT 1")
        .map_err(|_| "Failed to prepare SELECT statement")?;

    let user = stmt
        .query_row([], |row| {
            Ok(User {
                fullname: row.get(0)?,
                description: row.get(1)?,
                about: row.get(2)?,
            })
        })
        .map_err(|_| "Failed to fetch user data")?;

    Ok(Json(user))
}

pub async fn edit_me(
    State(db): State<Arc<Mutex<Connection>>>,
    Json(payload): Json<User>,
) -> Result<Json<User>, String> {
    let conn = db.lock().map_err(|_| "Failed to acquire database lock")?;
    let mut stmt = conn
        .prepare("UPDATE users SET fullname = ?, description = ?, about = ?")
        .map_err(|_| "Failed to prepare UPDATE statement")?;

    stmt.execute(&[&payload.fullname, &payload.description, &payload.about])
        .map_err(|_| "Failed to execute UPDATE statement")?;

    let mut stmt = conn
        .prepare("SELECT fullname, description, about FROM users LIMIT 1")
        .map_err(|_| "Failed to prepare SELECT statement")?;

    let user = stmt
        .query_row([], |row| {
            Ok(User {
                fullname: row.get(0)?,
                description: row.get(1)?,
                about: row.get(2)?,
            })
        })
        .map_err(|_| "Failed to fetch updated user data")?;

    Ok(Json(user))
}
