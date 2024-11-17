use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn FormInputImage(input: Arc<stela::FormInputImage>, title: Option<String>) -> impl IntoView {
    view! { <input name=input.name.clone() placeholder=title /> }
}
