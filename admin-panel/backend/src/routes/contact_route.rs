use std::sync::{Arc, Mutex};

use axum::{extract::State, Json};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::models::contact::Contact;


#[derive(Debug, Serialize, Deserialize)]
pub struct NewContact {
    name: String,
    link: String,
}

pub async fn add_contact(
    State(db): State<Arc<Mutex<Connection>>>,
    Json(payload): Json<NewContact>,
) -> Result<Json<Contact>, String> {
    let conn = db.lock().map_err(|_| "Failed to acquire database lock")?;

    let mut stmt = conn
        .prepare("INSERT INTO contacts (name, link) VALUES (?, ?)")
        .map_err(|_| "Failed to prepare INSERT statement")?;

    stmt.execute(&[&payload.name, &payload.link])
        .map_err(|_| "Failed to execute INSERT statement")?;

    let new_id = conn.last_insert_rowid() as i32;

    let mut stmt = conn
        .prepare("SELECT id, name, link FROM contacts WHERE id = ?")
        .map_err(|_| "Failed to prepare SELECT statement")?;

    let contact = stmt
        .query_row([new_id], |row| {
            Ok(Contact {
                id: row.get("id")?,
                name: row.get("name")?,
                link: row.get("link")?,
            })
        })
        .map_err(|e| format!("Failed to fetch new contact data: {:?}", e))?;

    Ok(Json(contact))
}
