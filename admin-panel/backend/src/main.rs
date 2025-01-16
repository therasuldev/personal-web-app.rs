use axum::{
    routing::{post, put},
    Router,
};
use db::create_db_connection;
use routes::{contact_route::add_contact, project_route::add_project, user_route::edit_me, work_experience_route::add_work_experience};
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
        .route("/me/edit", put(edit_me))
        .route("/work-experience", post(add_work_experience))
        .route("/projects", post(add_project))
        .route("/contacts", post(add_contact))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(db);

        let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        println!("Server running on http://localhost:{}", port);

    axum::serve(listener, app).await.unwrap();
}
