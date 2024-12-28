// src/main.rs
mod components;
mod services;

use components::about_section::AboutSection;
use components::contacts_section::ContactsSection;
use components::header::Header;
use components::projects_section::ProjectsSection;
use services::contact_service::fetch_contacts_data;
use services::project_service::fetch_projects_data;
use services::user_service::fetch_user_data;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component]
fn PersonalWebUI() -> Html {
    let name = use_state(String::new);
    let description = use_state(String::new);
    let about = use_state(String::new);
    let projects = use_state(Vec::new);
    let contacts = use_state(Vec::new);
    let error = use_state(|| None::<String>);

    {
        let name = name.clone();
        let description = description.clone();
        let about = about.clone();
        let projects = projects.clone();
        let contacts = contacts.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                fetch_user_data(
                    name.clone(),
                    description.clone(),
                    about.clone(),
                    error.clone(),
                )
                .await;
                fetch_projects_data(projects.clone(), error.clone()).await;
                fetch_contacts_data(contacts.clone(), error.clone()).await;
            });

            || ()
        });
    }

    html! {
        <div class="min-h-screen bg-gradient-to-b from-blue-100 to-gray-200 flex flex-col items-center py-10 px-4">
            <Header name={(*name).clone()} description={(*description).clone()} />
            <AboutSection about={(*about).clone()} />
            <ProjectsSection projects={(*projects).clone()} error={(*error).clone()} />
            <ContactsSection contacts={(*contacts).clone()} />
        </div>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <PersonalWebUI />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
