mod components;
mod services;
mod models;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::models::{ContactForm, ProjectForm, UserForm, WorkExperienceForm};
use crate::components::{ContactFormComponent, ProjectFormComponent, UserFormComponent, WorkExperienceFormComponent};
use services::{add_contact, add_project, update_user, add_work_experience};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    AdminPanel,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
pub fn AdminPanelPage() -> Html {
    let user_form = use_state(UserForm::default);
    let work_exp_form = use_state(WorkExperienceForm::default);
    let project_form = use_state(ProjectForm::default);
    let contact_form = use_state(ContactForm::default);
    let status_message = use_state(String::default);

    let on_user_submit = {
        let user_form = user_form.clone();
        let status_message = status_message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let form = (*user_form).clone();
            let status_message = status_message.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match update_user(&form).await {
                    Ok(_) => status_message.set("User information updated successfully!".to_string()),
                    Err(e) => status_message.set(format!("Error updating user: {}", e)),
                }
            });
        })
    };

    let on_work_exp_submit = {
        let work_exp_form = work_exp_form.clone();
        let status_message = status_message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let form = (*work_exp_form).clone();
            let status_message = status_message.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match add_work_experience(&form).await {
                    Ok(_) => status_message.set("Work experience added successfully!".to_string()),
                    Err(e) => status_message.set(format!("Error adding work experience: {}", e)),
                }
            });
        })
    };

    let on_project_submit = {
        let project_form = project_form.clone();
        let status_message = status_message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let form = (*project_form).clone();
            let status_message = status_message.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match add_project(&form).await {
                    Ok(_) => status_message.set("Project added successfully!".to_string()),
                    Err(e) => status_message.set(format!("Error adding project: {}", e)),
                }
            });
        })
    };

    let on_contact_submit = {
        let contact_form = contact_form.clone();
        let status_message = status_message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let form = (*contact_form).clone();
            let status_message = status_message.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match add_contact(&form).await {
                    Ok(_) => status_message.set("Contact added successfully!".to_string()),
                    Err(e) => status_message.set(format!("Error adding contact: {}", e)),
                }
            });
        })
    };

    html! {
        <div class="container mx-auto p-4">
            <h1 class="text-3xl font-bold mb-6">{ "Admin Panel" }</h1>

            if !(*status_message).is_empty() {
                <div class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4">
                    { &*status_message }
                </div>
            }

            <UserFormComponent form={user_form} on_submit={on_user_submit} />
            <WorkExperienceFormComponent form={work_exp_form} on_submit={on_work_exp_submit} />
            <ProjectFormComponent form={project_form} on_submit={on_project_submit} />
            <ContactFormComponent form={contact_form} on_submit={on_contact_submit} />
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
        Route::AdminPanel => html! { <AdminPanelPage /> },
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