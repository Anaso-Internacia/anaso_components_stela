use std::rc::Rc;

use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn FormInputCheckbox(
    input: Rc<stela::FormInputCheckbox>,
    title: Option<Rc<str>>,
) -> impl IntoView {
    view! {
        <input
            type="checkbox"
            id=input.name.clone()
            name=input.name.clone()
            prop:checked=input.default_checked
        />
        {title.map(|text| view! { <label for=input.name.clone()>{text}</label> })}
    }
}
