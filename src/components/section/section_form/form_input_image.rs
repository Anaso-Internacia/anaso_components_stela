use std::rc::Rc;

use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn FormInputImage(input: Rc<stela::FormInputImage>, title: Option<Rc<str>>) -> impl IntoView {
    view! { <input name=input.name.clone() placeholder=title /> }
}
