use crate::models::contact::Contact;
use axum::{extract::State, Json};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub async fn get_contacts(State(db): State<Arc<Mutex<Connection>>>) -> Json<Vec<Contact>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, name, link FROM contacts").unwrap();
    let contact_iter = stmt
        .query_map([], |row| {
            Ok(Contact {
                id: row.get(0)?,
                name: row.get(1)?,
                link: row.get(2)?,
            })
        })
        .unwrap();

    let mut contacts = Vec::new();
    for contact in contact_iter {
        contacts.push(contact.unwrap());
    }

    Json(contacts)
}
