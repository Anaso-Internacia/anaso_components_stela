use std::rc::Rc;

use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn FormInputRadio(input: Rc<stela::FormInputRadio>, title: Option<Rc<str>>) -> impl IntoView {
    view! { <input name=input.name.clone() placeholder=title /> }
}
