use gloo_net::http::Request;
use yew::UseStateHandle;

use crate::components::work_experience_section::WorkExperience;

// Example of how to fetch work experience data
pub async fn fetch_work_experience_data(
    experiences: UseStateHandle<Vec<WorkExperience>>,
    error: UseStateHandle<Option<String>>,
) {
    match Request::get("http://localhost:3000/work-experience").send().await {
        Ok(response) => match response.json::<Vec<WorkExperience>>().await {
            Ok(data) => experiences.set(data),
            Err(e) => error.set(Some(format!("Failed to parse work experience: {}", e))),
        },
        Err(e) => error.set(Some(format!("Failed to fetch work experience: {}", e))),
    }
}