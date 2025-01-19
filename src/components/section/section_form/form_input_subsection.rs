use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::prelude::*;

use super::FormInput;

#[component]
pub fn FormInputSubsection(input: Arc<stela::FormInputSubsection>) -> impl IntoView {
    let inputs = input
        .inputs
        .iter()
        .map(|input| view! { <FormInput input=input.clone() /> }.into_any())
        .collect_view();

    view! {
        <h4>{input.title.clone()}</h4>
        {inputs}
    }
}
