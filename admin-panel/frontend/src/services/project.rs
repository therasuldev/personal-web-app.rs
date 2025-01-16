use crate::models::ProjectForm;
use gloo_net::http::Request;
use serde_json::json;

pub async fn add_project(form: &ProjectForm) -> Result<(), String> {
    Request::post("http://localhost:3000/projects")
        .header("Content-Type", "application/json")
        .json(&json!({
            "name": form.name,
            "description": form.description,
            "link": form.link,
        }))
        .unwrap()
        .send()
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}