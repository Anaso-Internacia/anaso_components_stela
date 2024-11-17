use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::*;

use super::FormInput;

#[component]
pub fn FormInputSubsection(
    input: Arc<stela::FormInputSubsection>,
    title: Option<String>,
) -> impl IntoView {
    let inputs = input
        .inputs
        .iter()
        .map(|input| view! { <FormInput input=input.clone() /> })
        .collect_view();

    view! {
        <h4>{title}</h4>
        {inputs}
    }
}
