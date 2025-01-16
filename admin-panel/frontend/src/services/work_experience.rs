use crate::models::WorkExperienceForm;
use gloo_net::http::Request;
use serde_json::json;

pub async fn add_work_experience(form: &WorkExperienceForm) -> Result<(), String> {
    Request::post("http://localhost:3000/work-experience")
        .header("Content-Type", "application/json")
        .json(&json!({
            "company": form.company,
            "position": form.position,
            "period": form.period,
            "description": form.description,
        }))
        .unwrap()
        .send()
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}