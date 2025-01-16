mod components;
mod services;

use components::{
    about_section::AboutSection, contacts_section::ContactsSection, header::Header,
    projects_section::ProjectsSection, work_experience_section::WorkExperienceSection,
};
use services::{
    contact_service::fetch_contacts_data, project_service::fetch_projects_data,
    user_service::fetch_user_data, work_experience::fetch_work_experience_data,
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
fn HomePage() -> Html {
    let name = use_state(String::new);
    let description = use_state(String::new);
    let about = use_state(String::new);
    let projects = use_state(Vec::new);
    let contacts = use_state(Vec::new);
    let experiences = use_state(Vec::new);
    let error = use_state(|| None::<String>);

    {
        let name = name.clone();
        let description = description.clone();
        let about = about.clone();
        let projects = projects.clone();
        let contacts = contacts.clone();
        let experiences = experiences.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                fetch_user_data(name, description, about, error.clone()).await;
                fetch_projects_data(projects, error.clone()).await;
                fetch_contacts_data(contacts, error.clone()).await;
                fetch_work_experience_data(experiences, error.clone()).await;
            });
            || ()
        });
    };

    html! {
        <div class="min-h-screen bg-gradient-to-b from-blue-100 to-gray-200 flex flex-col items-center py-10 px-4">
            <Header name={(*name).clone()} description={(*description).clone()} />
            <AboutSection about={(*about).clone()} />
            <WorkExperienceSection experiences={(*experiences).clone()} />
            <ProjectsSection projects={(*projects).clone()} error={(*error).clone()} />
            <ContactsSection contacts={(*contacts).clone()} />
        </div>
    }
}

#[function_component]
fn NotFound() -> Html {
    html! {
        <div>
            <h1>{ "404" }</h1>
            <p>{ "Page not found" }</p>
        </div>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
