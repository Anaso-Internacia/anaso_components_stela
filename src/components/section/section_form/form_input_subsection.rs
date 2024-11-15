use std::rc::Rc;

use anaso_site_api_models::stela;
use leptos::*;

use super::FormInput;

#[component]
pub fn FormInputSubsection(
    input: Rc<stela::FormInputSubsection>,
    title: Option<Rc<str>>,
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
