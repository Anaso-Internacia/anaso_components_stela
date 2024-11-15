use std::rc::Rc;

use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn SectionTiles(section: Rc<stela::SectionTiles>, border: bool) -> impl IntoView {
    view! {
        <p>"tiles"</p>
    }
}
