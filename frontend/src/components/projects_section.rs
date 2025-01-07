use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub link: String,
}

#[derive(Properties, PartialEq)]
pub struct ProjectsSectionProps {
    pub projects: Vec<Project>,
    pub error: Option<String>,
}

#[function_component]
pub fn ProjectsSection(props: &ProjectsSectionProps) -> Html {
    html! {
        <div class="max-w-4xl w-full mb-4">
            <h2 class="text-2xl font-bold text-gray-800 mb-3">{"Top projects"}</h2>
            {
                if let Some(error_message) = &props.error {
                    html! {
                        <div class="w-full bg-red-50 border-l-4 border-red-400 text-red-700 p-4 mb-6 rounded-md">
                            <p class="font-medium">{error_message}</p>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {
                    props.projects.iter().map(|project| {
                        html! {
                            <div key={project.id} class="bg-white rounded-lg shadow-xl hover:shadow-2xl transition-all transform hover:scale-105 flex flex-col">
                                <div class="px-6 py-4 flex-grow">
                                    <h3 class="text-base font-bold text-gray-800 mb-2">{&project.name}</h3>
                                    <p class="text-gray-500 text-sm mb-4"
                                       style="display: -webkit-box; -webkit-line-clamp: 5; -webkit-box-orient: vertical; overflow: hidden; text-overflow: ellipsis;">
                                        {&project.description}
                                    </p>
                                </div>
                                <a
                                    href={project.link.clone()}
                                    class="inline-block px-4 py-2 text-sm text-white bg-blue-500 rounded-lg hover:bg-blue-600 transition mx-auto mb-4"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                >
                                    {"View Project"}
                                </a>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
