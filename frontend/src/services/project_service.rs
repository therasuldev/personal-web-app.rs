// src/services/project_service.rs
use gloo_net::http::Request;
use yew::prelude::*;

use crate::components::projects_section::Project;

pub async fn fetch_projects_data(
    projects: UseStateHandle<Vec<Project>>,
    error: UseStateHandle<Option<String>>,
) {
    match Request::get("http://localhost:3000/projects").send().await {
        Ok(response) => match response.json::<Vec<Project>>().await {
            Ok(data) => projects.set(data),
            Err(e) => error.set(Some(format!("Failed to parse projects: {}", e))),
        },
        Err(e) => error.set(Some(format!("Failed to fetch projects: {}", e))),
    }
}
