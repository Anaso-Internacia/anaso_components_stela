use std::rc::Rc;

use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn SectionPost(section: Rc<stela::SectionPost>, border: bool) -> impl IntoView {
    view! {
        <p>"post"</p>
    }
}
