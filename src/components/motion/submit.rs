use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn MotionSubmit(
    submit: Arc<stela::MotionSubmit>,
    children: ChildrenFn,
    /// Sets the `class` attribute on the underlying element, making it easier to style.
    class: Option<String>,
) -> impl IntoView {
    let _ = submit;
    view! {
            <button type="submit" name="submit" value="" class=class>
                {children()}
            </button>
    }
}
