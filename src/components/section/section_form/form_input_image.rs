use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn FormInputImage(input: Arc<stela::FormInputImage>) -> impl IntoView {
    view! { <input name=input.name.clone() placeholder=input.title.clone() /> }
}
