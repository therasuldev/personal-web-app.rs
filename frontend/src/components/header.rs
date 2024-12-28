use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub name: String,
    pub description: String,
}

#[function_component]
pub fn Header(props: &HeaderProps) -> Html {
    html! {
        <div class="max-w-4xl w-full text-center mb-12">
            <h1 class="text-5xl font-extrabold text-gray-800 mb-2">{&props.name}</h1>
            <p class="text-lg text-gray-700 leading-relaxed mb-8">{&props.description}</p>
        </div>
    }
}
