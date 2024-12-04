use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn FormInputCheckbox(input: Arc<stela::FormInputCheckbox>) -> impl IntoView {
    view! {
        <div class="stela--form--checkbox">
            <input
                type="checkbox"
                id=input.name.clone()
                name=input.name.clone()
                prop:checked=input.default_checked
            />
            {input.title.clone().map(|text| view! { <label for=input.name.clone()>{text}</label> })}
        </div>
    }
}
