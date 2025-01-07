use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Contact {
    pub id: i32,
    pub name: String,
    pub link: String,
}

#[derive(Properties, PartialEq)]
pub struct ContactsSectionProps {
    pub contacts: Vec<Contact>,
}

#[function_component]
pub fn ContactsSection(props: &ContactsSectionProps) -> Html {
    html! {
        <div class="max-w-4xl w-full mt-12">
            <h2 class="text-2xl font-semibold text-center mb-3 text-gray-700">{"Contact Me"}</h2>
            <div class="flex justify-center gap-6 flex-wrap">
                {
                    props.contacts.iter().map(|contact| {
                        let (icon_class, link) = match contact.name.as_str() {
                            "LinkedIn" => ("fab fa-linkedin text-4xl text-blue-500", contact.link.clone()),
                            "GitHub" => ("fab fa-github text-4xl text-gray-700", contact.link.clone()),
                            "Email" => ("fas fa-envelope text-4xl text-red-500", format!("mailto:{}", contact.link)),
                            "X" => ("fab fa-twitter text-4xl text-light-blue-500", contact.link.clone()),
                            _ => ("fas fa-globe text-4xl text-gray-400", contact.link.clone()),
                        };

                        html! {
                            <a
                                href={link}
                                class="flex items-center space-x-4 p-4 border border-gray-300 rounded-lg shadow-md hover:shadow-lg transition-all duration-200 ease-in-out transform hover:scale-105 hover:bg-gray-50"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                <i class={icon_class}></i>
                                <span class="text-lg text-gray-800 font-medium hover:text-indigo-600 transition-colors duration-200">
                                    {&contact.name}
                                </span>
                            </a>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
