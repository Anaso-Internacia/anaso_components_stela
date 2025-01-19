use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::prelude::*;

use crate::VisualMotion;

#[component]
pub fn FormInputMotions(input: Arc<stela::FormInputMotions>) -> impl IntoView {
    let motions = input
        .motions
        .iter()
        .map(|motion| view! { <VisualMotion motion=motion.clone() /> })
        .collect_view();
    view! { <div class="stela--motion-bar">{motions}</div> }
}
