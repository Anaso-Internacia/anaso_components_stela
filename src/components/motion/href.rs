use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn MotionHref(
    href: Arc<stela::MotionHref>,
    children: ChildrenFn,
    /// Sets the `class` attribute on the underlying element, making it easier to style.
    class: Option<String>,
) -> impl IntoView {
    view! {
        <A
            href=href.uri.clone()
            attr:class=class
            attr:target=if href.new_tab == Some(true) { "_blank" } else { "" }
            attr:rel=if href.new_tab == Some(true) { "noopener noreferrer" } else { "" }
        >
            {children()}
        </A>
    }
}
