use std::rc::Rc;

use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn SectionSponsor(section: Rc<stela::SectionSponsor>, border: bool) -> impl IntoView {
    view! {
        <p>"sponsor"</p>
    }
}
