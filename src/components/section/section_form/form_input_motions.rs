use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::*;

use crate::VisualMotion;

#[component]
pub fn FormInputMotions(
    input: Arc<stela::FormInputMotions>,
    title: Option<String>,
) -> impl IntoView {
    let motions = input
        .motions
        .iter()
        .map(|motion| view! { <VisualMotion motion=motion.clone() /> })
        .collect_view();
    view! { <div class="stela--motion-bar">{title}{motions}</div> }
}
