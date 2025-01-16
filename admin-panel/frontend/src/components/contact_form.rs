use crate::models::ContactForm;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub form: UseStateHandle<ContactForm>,
    pub on_submit: Callback<SubmitEvent>,
}

#[function_component]
pub fn ContactFormComponent(props: &Props) -> Html {
    let form = &props.form;

    html! {
        <div class="mb-8 p-6 bg-white rounded-lg shadow">
            <h2 class="text-xl font-semibold mb-4">{ "Add Contact" }</h2>
            <form onsubmit={props.on_submit.clone()}>
                <div class="mb-4">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="contact_name">
                        { "Contact Name" }
                    </label>
                    <input
                        type="text"
                        id="contact_name"
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
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="contact_link">
                        { "Contact Link" }
                    </label>
                    <input
                        type="text"
                        id="contact_link"
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
                    class="bg-indigo-500 hover:bg-indigo-700 text-white font-bold py-2 px-4 rounded"
                >
                    { "Add Contact" }
                </button>
            </form>
        </div>
    }
}
