use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::prelude::*;

use crate::components::section::section_form::FormInput;

#[component]
pub fn FormInputTabs(input: Arc<stela::FormInputTabs>) -> impl IntoView {
    let selected_tab = RwSignal::new(0);

    let tabs = input
        .tabs
        .iter()
        .enumerate()
        .map(|(i, tab)| view! {
            <button
                type="button"
                class:active=move || selected_tab.get() == i
                on:click=move |_| selected_tab.set(i)
            >
                {tab.title.clone()}
            </button>
        })
        .collect_view();

    view! {
        <div class="stela--form--input-tabs">{tabs}</div>
        {move || {
            input
                .tabs
                .get(selected_tab.get())
                .map(|tab| view! { <FormInput input=tab.input.clone() /> }.into_any())
        }}
    }
}
