use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::prelude::*;

#[component]
pub fn SectionTiles(section: Arc<stela::SectionTiles>, border: bool) -> impl IntoView {
    let _ = section;
    let _ = border;
    view! { <p>"tiles"</p> }
}
