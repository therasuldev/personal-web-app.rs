// src/services/contact_service.rs
use gloo_net::http::Request;
use yew::prelude::*;

use crate::components::contacts_section::Contact;

pub async fn fetch_contacts_data(
    contacts: UseStateHandle<Vec<Contact>>,
    error: UseStateHandle<Option<String>>,
) {
    match Request::get("http://localhost:3000/contacts").send().await {
        Ok(response) => match response.json::<Vec<Contact>>().await {
            Ok(data) => contacts.set(data),
            Err(e) => error.set(Some(format!("Failed to parse contacts: {}", e))),
        },
        Err(e) => error.set(Some(format!("Failed to fetch contacts: {}", e))),
    }
}
