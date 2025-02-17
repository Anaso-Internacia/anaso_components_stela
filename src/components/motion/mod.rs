use anaso_site_api_models::stela;
use api_call::MotionApiCall;
use href::MotionHref;
use leptos::{either::EitherOf5, prelude::*};
use phosphor_leptos::{
    Icon, IconData, IconWeight, BELL, CHAT_CIRCLE, FLAG, GLOBE_HEMISPHERE_WEST, HEART, HOUSE,
    MAGNIFYING_GLASS, PLACEHOLDER, PLUS, PUSH_PIN, SHARE_FAT, SHIELD, SIGN_IN, SIGN_OUT,
    TOGGLE_LEFT, USER_CIRCLE,
};
use share::MotionShare;
use submit::MotionSubmit;

mod api_call;
mod href;
mod share;
mod submit;

#[component]
pub fn Motion(
    motion: stela::Motion,
    children: ChildrenFn,
    #[prop(optional, into)] is_toggled: Option<RwSignal<bool>>,
    #[prop(optional, into)] text: Option<RwSignal<Option<String>>>,
    #[prop(optional, into)] icon: Option<RwSignal<Option<stela::MotionIcon>>>,
    /// Sets the `class` attribute on the underlying element, making it easier to style.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    match motion {
        stela::Motion::ApiCall(api_call) => EitherOf5::A(view! {
            <MotionApiCall api_call=api_call is_toggled=is_toggled text=text icon=icon class=class>
                {children()}
            </MotionApiCall>
        }),

        stela::Motion::Href(href) => EitherOf5::B(view! {
            <MotionHref href=href class=class>
                {children()}
            </MotionHref>
        }),

        stela::Motion::Share(share) => EitherOf5::C(view! {
            <MotionShare share=share class=class>
                {children()}
            </MotionShare>
        }),

        stela::Motion::Submit(submit) => EitherOf5::D(view! {
            <MotionSubmit submit=submit class=class>{children()}</MotionSubmit>
        }),

        stela::Motion::Unknown => EitherOf5::E(()),
    }
}

#[component]
pub fn VisualMotion(motion: stela::VisualMotion) -> impl IntoView {
    let initial_toggle = motion.initial_toggle;

    let is_toggled = RwSignal::new(initial_toggle.unwrap_or_default());
    let text = RwSignal::new(motion.title.clone());
    let icon = RwSignal::new(motion.icon);
    let image = motion.image.clone();
    let image_url = image.map(|image| ["https://ana.so/cdn-cgi/image/quality=80,w=24,h=24/cdn-cgi/imagedelivery/MRTPzGIpYfy00UVryjholQ/", &image.id, "/public"].concat());

    let variant_class = match motion.variant {
        stela::MotionVariant::Button | stela::MotionVariant::Unknown => {
            "stela--motion-variant--button"
        }
        stela::MotionVariant::ButtonBorder => "stela--motion-variant--button-border",
        stela::MotionVariant::Link => "stela--motion-variant--link",
        stela::MotionVariant::LinkHoverButton => "stela--motion-variant--link-hover-button",
        stela::MotionVariant::LinkHoverButtonBorder => {
            "stela--motion-variant--link-hover-button-border"
        }
    };

    let color_class = match motion.color {
        stela::MotionColor::Primary => "stela--motion-color--primary",
        stela::MotionColor::Secondary => "stela--motion-color--secondary",
        stela::MotionColor::Text | stela::MotionColor::Unknown => "stela--motion-color--text",
    };

    view! {
        <Motion
            motion=motion.motion
            is_toggled=is_toggled
            text=text
            icon=icon
            class=["stela--visual-motion ", variant_class, " ", color_class].concat()
        >
            {move || {
                icon.get()
                    .map(|icon| view! { <MotionIcon icon=icon is_toggled=is_toggled.into() /> })
            }}
            {image_url.clone().map(|image_url| view! { <img src=image_url />})}
            {text}
        </Motion>
    }
}

#[component]
fn MotionIcon(icon: stela::MotionIcon, is_toggled: Signal<bool>) -> impl IntoView {
    let (icon, mirrored): (IconData, Signal<bool>) = match icon {
        stela::MotionIcon::Bell => (BELL, false.into()),
        stela::MotionIcon::ChatCircle => (CHAT_CIRCLE, true.into()),
        stela::MotionIcon::Flag => (FLAG, false.into()),
        stela::MotionIcon::GlobeHemisphereWest => (GLOBE_HEMISPHERE_WEST, false.into()),
        stela::MotionIcon::Heart => (HEART, false.into()),
        stela::MotionIcon::House => (HOUSE, false.into()),
        stela::MotionIcon::MagnifyingGlass => (MAGNIFYING_GLASS, false.into()),
        stela::MotionIcon::Plus => (PLUS, false.into()),
        stela::MotionIcon::PushPin => (PUSH_PIN, false.into()),
        stela::MotionIcon::ShareFat => (SHARE_FAT, false.into()),
        stela::MotionIcon::Shield => (SHIELD, false.into()),
        stela::MotionIcon::SignIn => (SIGN_IN, false.into()),
        stela::MotionIcon::SignOut => (SIGN_OUT, false.into()),
        stela::MotionIcon::Toggle => (TOGGLE_LEFT, is_toggled),
        stela::MotionIcon::Unknown => (PLACEHOLDER, false.into()),
        stela::MotionIcon::UserCircle => (USER_CIRCLE, false.into()),
    };

    let weight = Signal::derive(move || match is_toggled.get() {
        true => IconWeight::Fill,
        false => IconWeight::Regular,
    });

    view! { <Icon icon=icon mirrored=mirrored weight=weight size="20px" /> }
}
