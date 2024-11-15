use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn SectionCard(
    children: Children,
    #[prop(optional)] border: bool,
    #[prop(default=None)] background_image_light: Option<stela::Image>,
    #[prop(default=None)] background_image_dark: Option<stela::Image>,
    #[prop(optional)] background_blur: Option<f32>,
) -> impl IntoView {
    view! { <div>{children()}</div> }
}
