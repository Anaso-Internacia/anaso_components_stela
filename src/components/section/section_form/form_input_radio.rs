use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::prelude::*;

#[component]
pub fn FormInputRadio(input: Arc<stela::FormInputRadio>) -> impl IntoView {
    view! { <input name=input.name.clone() placeholder=input.title.clone() /> }
}
