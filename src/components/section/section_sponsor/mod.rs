use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn SectionSponsor(section: Arc<stela::SectionSponsor>, border: bool) -> impl IntoView {
    let _ = section;
    let _ = border;
    view! { <p>"sponsor"</p> }
}
