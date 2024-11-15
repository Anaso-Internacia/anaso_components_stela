use std::rc::Rc;

use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn FormInputMotions(
    input: Rc<stela::FormInputMotions>,
    title: Option<Rc<str>>,
) -> impl IntoView {
    view! { <p>"motions"</p> }
}
