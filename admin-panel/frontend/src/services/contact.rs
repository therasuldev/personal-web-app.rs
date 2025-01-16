use crate::models::ContactForm;
use gloo_net::http::Request;
use serde_json::json;

pub async fn add_contact(form: &ContactForm) -> Result<(), String> {
    Request::post("http://localhost:3000/contacts")
        .header("Content-Type", "application/json")
        .json(&json!({
            "name": form.name,
            "link": form.link,
        }))
        .unwrap()
        .send()
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}