use crate::models::WorkExperienceForm;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub form: UseStateHandle<WorkExperienceForm>,
    pub on_submit: Callback<SubmitEvent>,
}

#[function_component]
pub fn WorkExperienceFormComponent(props: &Props) -> Html {
    let form = &props.form;

    html! {
        <div class="mb-8 p-6 bg-white rounded-lg shadow">
            <h2 class="text-xl font-semibold mb-4">{ "Add Work Experience" }</h2>
            <form onsubmit={props.on_submit.clone()}>
                <div class="mb-4">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="company">
                        { "Company" }
                    </label>
                    <input
                        type="text"
                        id="company"
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700"
                        value={form.company.clone()}
                        onchange={let form = form.clone(); move |e: Event| {
                            let target: HtmlInputElement = e.target_unchecked_into();
                            let mut new_form = (*form).clone();
                            new_form.company = target.value();
                            form.set(new_form);
                        }}
                    />
                </div>
                <div class="mb-4">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="position">
                        { "Position" }
                    </label>
                    <input
                        type="text"
                        id="position"
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700"
                        value={form.position.clone()}
                        onchange={let form = form.clone(); move |e: Event| {
                            let target: HtmlInputElement = e.target_unchecked_into();
                            let mut new_form = (*form).clone();
                            new_form.position = target.value();
                            form.set(new_form);
                        }}
                    />
                </div>
                <div class="mb-4">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="period">
                        { "Period" }
                    </label>
                    <input
                        type="text"
                        id="period"
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700"
                        value={form.period.clone()}
                        onchange={let form = form.clone(); move |e: Event| {
                            let target: HtmlInputElement = e.target_unchecked_into();
                            let mut new_form = (*form).clone();
                            new_form.period = target.value();
                            form.set(new_form);
                        }}
                    />
                </div>
                <div class="mb-4">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="work_description">
                        { "Description" }
                    </label>
                    <textarea
                        id="work_description"
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
                <button
                    type="submit"
                    class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded"
                >
                    { "Add Work Experience" }
                </button>
            </form>
        </div>
    }
}
