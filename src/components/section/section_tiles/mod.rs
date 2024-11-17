use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn SectionTiles(section: Arc<stela::SectionTiles>, border: bool) -> impl IntoView {
    view! { <p>"tiles"</p> }
}
