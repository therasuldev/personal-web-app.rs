use axum::{extract::State, routing::get, Json, Router};
use dotenv::dotenv;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::{
    env,
    sync::{Arc, Mutex},
};
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    fullname: String,
    description: String,
    about: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    id: i32,
    name: String,
    description: String,
    link: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Contact {
    id: i32,
    name: String,
    link: String,
}

async fn get_user(State(db): State<Arc<Mutex<Connection>>>) -> Json<User> {
    let conn = db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, fullname, description, about FROM users LIMIT 1")
        .unwrap();

    let user = stmt
        .query_row([], |row| {
            Ok(User {
                id: row.get(0)?,
                fullname: row.get(1)?,
                description: row.get(2)?,
                about: row.get(3)?,
            })
        })
        .unwrap();

    Json(user)
}

async fn get_projects(State(db): State<Arc<Mutex<Connection>>>) -> Json<Vec<Project>> {
    let conn = db.lock().unwrap();
    println!("Fetching projects...");
    
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

async fn get_contacts(State(db): State<Arc<Mutex<Connection>>>) -> Json<Vec<Contact>> {
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

// Initialize database connection
fn create_db_connection() -> Connection {
    dotenv().ok(); // Load .env variables
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    Connection::open(db_url).expect("Failed to connect to database")
}

#[tokio::main]
async fn main() {
    // Initialize database connection
    let conn = create_db_connection();

    // Wrap connection in Arc<Mutex> for thread-safe sharing
    let db = Arc::new(Mutex::new(conn));

    // Create router with CORS enabled
    let app = Router::new()
        .route("/user", get(get_user))
        .route("/projects", get(get_projects))
        .route("/contacts", get(get_contacts))
        .layer(CorsLayer::new().allow_origin(Any))
        .with_state(db);

    // Create and bind TCP listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");

    // Start the server
    axum::serve(listener, app).await.unwrap();
}
