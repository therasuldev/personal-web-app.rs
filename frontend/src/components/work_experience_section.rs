use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkExperience {
    company: String,
    position: String,
    period: String,
    description: String,
}

#[derive(Properties, PartialEq)]
pub struct WorkExperienceSectionProps {
    pub experiences: Vec<WorkExperience>,
}

#[function_component]
pub fn WorkExperienceSection(props: &WorkExperienceSectionProps) -> Html {
    html! {
        <section class="w-full max-w-4xl mx-auto mt-8 mb-12">
            <h2 class="text-2xl font-bold text-gray-800 mb-3 text-left">
                {"Work Experience"}
            </h2>
            <div class="space-y-4">
                {props.experiences.iter().map(|exp| {
                    html! {
                        <div class="bg-white rounded-lg shadow-md p-6 transform transition-all duration-300 hover:shadow-xl hover:-translate-y-1">
                            <div class="flex justify-between items-center mb-4">
                                <div class="flex items-center">
                                    <h3 class="text-xl font-semibold text-gray-800 mr-4">
                                        {&exp.company}
                                    </h3>
                                    <p class="text-sm text-blue-600 font-bold">
                                        {&exp.position}
                                    </p>
                                </div>
                                <span class="text-sm text-gray-600 bg-gray-100 px-3 py-1 rounded-full">
                                    {&exp.period}
                                </span>
                            </div>
                            <div class="text-gray-500 mt-2">
                                {&exp.description}
                            </div>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        </section>
    }
}
