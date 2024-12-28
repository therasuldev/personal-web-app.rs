use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
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
        <div class="mb-8">
            <h2 class="text-3xl font-semibold text-center mb-8 text-gray-700">{"Contact Me"}</h2>
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
                            <div key={contact.id} class="flex items-center space-x-4 p-4 border border-gray-300 rounded-lg shadow-md hover:shadow-lg transition-all duration-200 ease-in-out transform hover:scale-105 hover:bg-gray-50">
                                <i class={icon_class}></i>
                                <a
                                    href={link}
                                    class="text-lg text-gray-800 font-medium hover:text-indigo-600 transition-colors duration-200"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                >
                                    {&contact.name}
                                </a>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
