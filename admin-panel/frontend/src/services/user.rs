use crate::models::UserForm;
use gloo_net::http::Request;
use serde_json::json;

pub async fn update_user(form: &UserForm) -> Result<(), String> {
    Request::put("http://localhost:3000/me/edit")
        .header("Content-Type", "application/json")
        .json(&json!({
            "fullname": form.fullname,
            "description": form.description,
            "about": form.about,
        }))
        .unwrap()
        .send()
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}
