use crate::models::UserForm;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub form: UseStateHandle<UserForm>,
    pub on_submit: Callback<SubmitEvent>,
}

#[function_component]
pub fn UserFormComponent(props: &Props) -> Html {
    let form = &props.form;
    
    html! {
        <div class="mb-8 p-6 bg-white rounded-lg shadow">
            <h2 class="text-xl font-semibold mb-4">{ "Edit User Information" }</h2>
            <form onsubmit={props.on_submit.clone()}>
                <div class="mb-4">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="fullname">
                        { "Full Name" }
                    </label>
                    <input
                        type="text"
                        id="fullname"
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700"
                        value={form.fullname.clone()}
                        onchange={let form = form.clone(); move |e: Event| {
                            let target: HtmlInputElement = e.target_unchecked_into();
                            let mut new_form = (*form).clone();
                            new_form.fullname = target.value();
                            form.set(new_form);
                        }}
                    />
                </div>
                <div class="mb-4">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="description">
                        { "Description" }
                    </label>
                    <input
                        type="text"
                        id="description"
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
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="about">
                        { "About" }
                    </label>
                    <input
                        type="text"
                        id="about"
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700"
                        value={form.about.clone()}
                        onchange={let form = form.clone(); move |e: Event| {
                            let target: HtmlInputElement = e.target_unchecked_into();
                            let mut new_form = (*form).clone();
                            new_form.about = target.value();
                            form.set(new_form);
                        }}
                    />
                </div>
                <button
                    type="submit"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                >
                    { "Update User Information" }
                </button>
            </form>
        </div>
    }
}