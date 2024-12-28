use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AboutSectionProps {
    pub about: String,
}

#[function_component]
pub fn AboutSection(props: &AboutSectionProps) -> Html {
    html! {
        <div class="max-w-4xl w-full mb-12">
            <h2 class="text-3xl font-bold text-gray-800 mb-4">{ "About Me" }</h2>
            <div class="px-6 py-8 bg-white rounded-lg shadow-md">
                <p class="text-lg text-gray-700 leading-relaxed">{&props.about}</p>
            </div>
        </div>
    }
}
