use axum::{
    routing::{get, post, put},
    Router,
};
use db::create_db_connection;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

use routes::{
    contact_routes::{add_contact, get_contacts},
    project_routes::{add_project, get_projects},
    user_routes::{edit_me, get_me},
    work_experience_routes::{add_work_experience, get_work_experience},
};

mod db;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let conn = create_db_connection();
    let db = Arc::new(Mutex::new(conn));

    let app = Router::new()
        .route("/me", get(get_me))
        .route("/work-experience", get(get_work_experience))
        .route("/projects", get(get_projects))
        .route("/contacts", get(get_contacts))
        .route("/me/edit", put(edit_me))
        .route("/work-experience", post(add_work_experience))
        .route("/projects", post(add_project))
        .route("/contacts", post(add_contact))
        .layer(CorsLayer::new().allow_origin(Any))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
