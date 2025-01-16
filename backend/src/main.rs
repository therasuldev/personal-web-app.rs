use axum::{routing::get, Router};
use db::create_db_connection;
use routes::{
    contact_routes::get_contacts, project_routes::get_projects, user_routes::get_me,
    work_experience_routes::get_work_experience,
};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

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
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    let port = listener.local_addr().unwrap().port();
    println!("Server running on http://localhost:{}", port);

    axum::serve(listener, app).await.unwrap();
}
