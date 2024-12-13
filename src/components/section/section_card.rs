use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn SectionCard(
    children: Children,
    #[prop(optional)] border: bool,
    #[prop(default=None)] background_image_light: Option<stela::Image>,
    #[prop(default=None)] background_image_dark: Option<stela::Image>,
    #[prop(optional)] background_blur: Option<f32>,
    /// Sets the `class` attribute on the underlying `<section>` tag, making it easier to style.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let _ = background_image_light;
    let _ = background_image_dark;
    let _ = background_blur;

    let mut classes = "stela--section ".to_string();

    if let Some(class) = class {
        classes += &class;
        classes += " ";
    }

    if border {
        classes += " stela--bordered";
    }

    view! { <section class=classes>{children()}</section> }
}
