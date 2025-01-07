use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AboutSectionProps {
    pub about: String,
}

#[function_component]
pub fn AboutSection(props: &AboutSectionProps) -> Html {
    html! {
        <div class="max-w-4xl w-full mb-4">
            <h2 class="text-2xl font-bold text-gray-800 mb-3">{ "About Me" }</h2>
            <div class="px-6 py-8 bg-white rounded-lg shadow-md">
                <p class="text-lg text-gray-500 leading-relaxed">{&props.about}</p>
            </div>
        </div>
    }
}
