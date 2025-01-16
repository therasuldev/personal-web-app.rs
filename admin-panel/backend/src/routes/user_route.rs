use crate::models::user::User;
use axum::{extract::State, Json};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub async fn edit_me(
    State(db): State<Arc<Mutex<Connection>>>,
    Json(payload): Json<User>,
) -> Result<Json<User>, String> {
    let conn = db.lock().map_err(|_| "Failed to acquire database lock")?;

    // Fetch the current user data
    let mut stmt = conn
        .prepare("SELECT fullname, description, about FROM users LIMIT 1")
        .map_err(|_| "Failed to prepare SELECT statement")?;

    let current_user = stmt
        .query_row([], |row| {
            Ok(User {
                fullname: row.get(0)?,
                description: row.get(1)?,
                about: row.get(2)?,
            })
        })
        .map_err(|_| "Failed to fetch current user data")?;

    // Use existing values if payload fields are empty
    let updated_fullname = if payload.fullname.is_empty() {
        current_user.fullname
    } else {
        payload.fullname.clone()
    };

    let updated_description = if payload.description.is_empty() {
        current_user.description
    } else {
        payload.description.clone()
    };

    let updated_about = if payload.about.is_empty() {
        current_user.about
    } else {
        payload.about.clone()
    };

    // Perform the update with the resolved values
    let mut stmt = conn
        .prepare("UPDATE users SET fullname = ?, description = ?, about = ?")
        .map_err(|_| "Failed to prepare UPDATE statement")?;

    stmt.execute(&[&updated_fullname, &updated_description, &updated_about])
        .map_err(|_| "Failed to execute UPDATE statement")?;

    // Return the updated user data
    Ok(Json(User {
        fullname: updated_fullname,
        description: updated_description,
        about: updated_about,
    }))
}
