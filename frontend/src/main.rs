use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    fullname: String,
    description: String,
    about: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    id: i32,
    name: String,
    description: String,
    link: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Contact {
    id: i32,
    name: String,
    link: String,
}

#[function_component]
fn PersonalWebUI() -> Html {
    let name = use_state(|| String::new());
    let description = use_state(|| String::new());
    let about = use_state(|| String::new());
    let projects = use_state(Vec::new);
    let contacts = use_state(Vec::new);
    let error = use_state(|| None::<String>);

    // Fetch data using use_effect
    {
        let name = name.clone();
        let description = description.clone();
        let about = about.clone();
        let projects = projects.clone();
        let contacts = contacts.clone();
        let error = error.clone();

        use_effect(move || {
            wasm_bindgen_futures::spawn_local(async move {
                // Fetch user profile data
                match Request::get("http://localhost:3000/user").send().await {
                    Ok(response) => match response.json::<User>().await {
                        Ok(user) => {
                            name.set(user.fullname);
                            description.set(user.description);
                            about.set(user.about);
                        }
                        Err(e) => {
                            error.set(Some(format!("Failed to fetch user data: {}", e)));
                        }
                    },
                    Err(e) => {
                        error.set(Some(format!("Failed to fetch user data: {}", e)));
                    }
                }

                // Fetch projects data
                match Request::get("http://localhost:3000/projects").send().await {
                    Ok(response) => match response.json::<Vec<Project>>().await {
                        Ok(data) => projects.set(data),
                        Err(e) => error.set(Some(format!("Failed to parse projects: {}", e))),
                    },
                    Err(e) => error.set(Some(format!("Failed to fetch projects: {}", e))),
                }

                // Fetch contacts data
                match Request::get("http://localhost:3000/contacts").send().await {
                    Ok(response) => match response.json::<Vec<Contact>>().await {
                        Ok(data) => contacts.set(data),
                        Err(e) => error.set(Some(format!("Failed to fetch contacts: {}", e))),
                    },
                    Err(e) => error.set(Some(format!("Failed to fetch contacts: {}", e))),
                }
            });

            || {}
        });
    }

    html! {
                <div class="min-h-screen bg-gradient-to-b from-blue-100 to-gray-200 flex flex-col items-center py-10 px-4">

                    /* Header Section */
                    <div class="max-w-4xl w-full text-center mb-12">
                        <h1 class="text-5xl font-extrabold text-gray-800 mb-2">{(*name).clone()}</h1>
                        <p class="text-lg text-gray-700 leading-relaxed mb-8">{(*description).clone()}</p>
                    </div>

                    /* About Section */
                    <div class="max-w-4xl w-full mb-12 px-6 py-8 bg-white rounded-lg shadow-md">
                        <h2 class="text-3xl font-bold text-gray-800 mb-4">{"About Me"}</h2>
                        <p class="text-lg text-gray-700 leading-relaxed">{(*about).clone()}</p>
                    </div>

                    /* Projects Section */
                    <div class="max-w-4xl w-full mb-12">
                        <h2 class="text-3xl font-bold text-gray-800 mb-6">{"Projects"}</h2>

                        if let Some(error_message) = (*error).clone() {
                            <div class="w-full bg-red-50 border-l-4 border-red-400 text-red-700 p-4 mb-6 rounded-md">
                                <p class="font-medium">{error_message}</p>
                            </div>
                        }

                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                            {(*projects).iter().map(|project| {
                                html! {
                                    <div key={project.id} class="bg-white rounded-lg shadow-xl hover:shadow-2xl transition-all transform hover:scale-105">
                                        <div class="px-6 py-4">
                                            <h3 class="text-xl font-bold text-gray-800 mb-2">{&project.name}</h3>
                                            <p class="text-gray-600 text-sm mb-4">{&project.description}</p>
                                            <a
                                                href={project.link.clone()}
                                                class="inline-block px-4 py-2 text-sm text-white bg-blue-500 rounded-lg hover:bg-blue-600 transition"
                                                target="_blank"
                                            >
                                                {"View Project"}
                                            </a>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()}
                        </div>
                    </div>

                    /* Contacts Section */
                    <div class="max-w-4xl w-full px-6 py-8 bg-white rounded-lg shadow-md">
                        <h2 class="text-3xl font-bold text-gray-800 mb-4">{"Contact Me"}</h2>
                        <p class="text-lg text-gray-700 mb-4">{"Feel free to reach out to me via the following platforms:"}</p>

                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
        {(*contacts).iter().map(|contact| {
            let icon_class = match contact.name.as_str() {
                "LinkedIn" => "fab fa-linkedin text-2xl text-blue-600", // LinkedIn icon
                "GitHub" => "fab fa-github text-2xl text-gray-800", // GitHub icon
                "Twitter" => "fab fa-twitter text-2xl text-blue-400", // Twitter icon
                _ => "fas fa-globe text-2xl text-gray-600", // Default globe icon for other contacts
            };

            html! {
                <div class="flex items-center space-x-4">
                    <i class={icon_class}></i>
                    <a href={contact.link.clone()} class="text-lg text-gray-800">{&contact.name}</a>
                </div>
            }
        }).collect::<Html>()}
    </div>


                    </div>

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
