use crate::models::ProjectForm;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub form: UseStateHandle<ProjectForm>,
    pub on_submit: Callback<SubmitEvent>,
}

#[function_component]
pub fn ProjectFormComponent(props: &Props) -> Html {
    let form = &props.form;

    html! {
        <div class="mb-8 p-6 bg-white rounded-lg shadow">
            <h2 class="text-xl font-semibold mb-4">{ "Add Project" }</h2>
            <form onsubmit={props.on_submit.clone()}>
                <div class="mb-4">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="project_name">
                        { "Project Name" }
                    </label>
                    <input
                        type="text"
                        id="project_name"
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700"
                        value={form.name.clone()}
                        onchange={let form = form.clone(); move |e: Event| {
                            let target: HtmlInputElement = e.target_unchecked_into();
                            let mut new_form = (*form).clone();
                            new_form.name = target.value();
                            form.set(new_form);
                        }}
                    />
                </div>
                <div class="mb-4">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="project_description">
                        { "Description" }
                    </label>
                    <textarea
                        id="project_description"
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700"
                        value={form.description.clone()}
                        onchange={let form = form.clone(); move |e: Event| {
                            let target: HtmlInputElement = e.target_unchecked_into();
                            let mut new_form = (*form).clone();
                            new_form.description = target.value();
                            form.set(new_form);
                        }}
                    />
                </div>
                <div class="mb-4">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="project_link">
                        { "Project Link" }
                    </label>
                    <input
                        type="text"
                        id="project_link"
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700"
                        value={form.link.clone()}
                        onchange={let form = form.clone(); move |e: Event| {
                            let target: HtmlInputElement = e.target_unchecked_into();
                            let mut new_form = (*form).clone();
                            new_form.link = target.value();
                            form.set(new_form);
                        }}
                    />
                </div>
                <button
                    type="submit"
                    class="bg-purple-500 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded"
                >
                    { "Add Project" }
                </button>
            </form>
        </div>
    }
}
