use anaso_site_api_models::stela;
use leptos::prelude::*;

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
    let _ = background_blur;

    let mut classes = "stela--section ".to_string();

    if let Some(class) = class {
        classes += &class;
        classes += " ";
    }

    if border {
        classes += " stela--bordered";
    }

    let mut style = String::new();

    if let Some(bg_light) = background_image_light {
        style.push_str("--bg_light: url(\"https://ana.so/cdn-cgi/image/quality=50,blur=10,w=50/cdn-cgi/imagedelivery/MRTPzGIpYfy00UVryjholQ/");
        style.push_str(&bg_light.id);
        style.push_str("/public\");");
    }
    if let Some(bg_dark) = background_image_dark {
        style.push_str("--bg_dark: url(\"https://ana.so/cdn-cgi/image/quality=50,blur=10,w=50/cdn-cgi/imagedelivery/MRTPzGIpYfy00UVryjholQ/");
        style.push_str(&bg_dark.id);
        style.push_str("/public\");");
    }

    view! {
        <section class=classes style=style>
            {children()}
        </section>
    }
}
