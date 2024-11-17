use anaso_site_api_models::stela;
use leptos::*;
use phosphor_leptos::{
    Icon, IconData, IconWeight, CHAT_CIRCLE, FLAG, GLOBE_HEMISPHERE_WEST, HEART, PLACEHOLDER, PLUS,
    PUSH_PIN, SHARE_FAT, SHIELD, SIGN_IN, SIGN_OUT, TOGGLE_LEFT, USER_CIRCLE,
};

#[component]
pub fn VisualMotion(motion: stela::VisualMotion) -> impl IntoView {
    let initial_toggle = motion.initial_toggle;
    let (is_toggled, set_is_toggled) = create_signal(initial_toggle.unwrap_or_default());

    // TODO:
    // /// The motion to perform when clicked.
    // pub motion: Motion,

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
        <a
            href="#"
            on:click=move |_| {
                if initial_toggle.is_some() {
                    set_is_toggled.update(|v| *v ^= true)
                }
            }
            class="stela--visual-motion"
            class=variant_class
            class=color_class
        >
            {motion
                .icon
                .map(|icon| view! { <MotionIcon icon=icon is_toggled=is_toggled.into() /> })}
            {motion.title}
        </a>
    }
}

#[component]
fn MotionIcon(icon: stela::MotionIcon, is_toggled: Signal<bool>) -> impl IntoView {
    let (icon, mirrored): (IconData, MaybeSignal<bool>) = match icon {
        stela::MotionIcon::ChatCircle => (CHAT_CIRCLE, true.into()),
        stela::MotionIcon::Heart => (HEART, false.into()),
        stela::MotionIcon::Shield => (SHIELD, false.into()),
        stela::MotionIcon::PushPin => (PUSH_PIN, false.into()),
        stela::MotionIcon::ShareFat => (SHARE_FAT, false.into()),
        stela::MotionIcon::Toggle => (TOGGLE_LEFT, is_toggled.into()),
        stela::MotionIcon::Unknown => (PLACEHOLDER, false.into()),
        stela::MotionIcon::GlobeHemisphereWest => (GLOBE_HEMISPHERE_WEST, false.into()),
        stela::MotionIcon::Plus => (PLUS, false.into()),
        stela::MotionIcon::SignIn => (SIGN_IN, false.into()),
        stela::MotionIcon::SignOut => (SIGN_OUT, false.into()),
        stela::MotionIcon::Flag => (FLAG, false.into()),
        stela::MotionIcon::UserCircle => (USER_CIRCLE, false.into()),
    };

    let weight = Signal::from(move || match is_toggled.get() {
        true => IconWeight::Fill,
        false => IconWeight::Regular,
    });

    view! { <Icon icon=icon mirrored=mirrored weight=weight size="20px" /> }
}
