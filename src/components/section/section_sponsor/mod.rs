use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn SectionSponsor(section: Arc<stela::SectionSponsor>, border: bool) -> impl IntoView {
    view! { <p>"sponsor"</p> }
}
