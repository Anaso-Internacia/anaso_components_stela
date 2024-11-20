use anaso_site_api_models::stela;
use leptos::*;
use leptos_router::*;
use phosphor_leptos::{
    Icon, IconData, IconWeight, CHAT_CIRCLE, FLAG, GLOBE_HEMISPHERE_WEST, HEART, PLACEHOLDER, PLUS,
    PUSH_PIN, SHARE_FAT, SHIELD, SIGN_IN, SIGN_OUT, TOGGLE_LEFT, USER_CIRCLE,
};

use crate::MotionInteractionContext;

#[component]
pub fn VisualMotion(motion: stela::VisualMotion) -> impl IntoView {
    let initial_toggle = motion.initial_toggle;

    let motion_interaction_context = use_context::<MotionInteractionContext>().unwrap();

    let (is_toggled, set_is_toggled) = create_signal(initial_toggle.unwrap_or_default());

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

    match motion.motion {
        stela::Motion::ApiCall(api_call) => view! {
            <a
                href=["/motion_interaction/", &urlencoding::encode(&api_call.data)].concat()
                on:click:undelegated={
                    let data = api_call.data.clone();
                    move |ev| {
                        ev.prevent_default();
                        let data = data.clone();
                        let old_toggle = is_toggled.get();
                        if initial_toggle.is_some() {
                            set_is_toggled.update(|v| *v ^= true)
                        }
                        spawn_local(async move {
                            let api = motion_interaction_context.get_value();
                            let res = api.emit(data).await;
                            match res {
                                Ok(res) => {
                                    if let Some(new_toggle) = res.new_toggle {
                                        set_is_toggled.try_set(new_toggle);
                                    }
                                }
                                Err(_) => {
                                    set_is_toggled.try_set(old_toggle);
                                }
                            }
                        });
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
        .into_view(),
        stela::Motion::Href(href) => view! {
            <A
                href=href.uri.clone()
                class=["stela--visual-motion ", variant_class, " ", color_class].concat()
            >
                {motion
                    .icon
                    .map(|icon| view! { <MotionIcon icon=icon is_toggled=is_toggled.into() /> })}
                {motion.title}
            </A>
        }
        .into_view(),
        stela::Motion::Share(share) => {
            let (can_share, set_can_share) = create_signal(false);

            let share_data = web_sys::ShareData::new();
            if let Some(title) = share.title.as_ref() {
                share_data.set_title(title);
            }
            if let Some(text) = share.text.as_ref() {
                share_data.set_text(text);
            }
            if let Some(url) = share.url.as_ref() {
                share_data.set_url(url);
            }

            create_effect({
                let share_data = share_data.clone();
                move |_| {
                    set_can_share.set(if let Some(window) = web_sys::window() {
                        let navigator = window.navigator();
                        let has_share = js_sys::Object::get_prototype_of(&navigator)
                            .has_own_property(&String::from("canShare").into());
                        if has_share {
                            navigator.can_share_with_data(&share_data)
                        } else {
                            false
                        }
                    } else {
                        false
                    })
                }
            });

            let icon = motion.icon.clone();
            let title = motion.title.clone();

            view! {
                {
                    let share_data = share_data.clone();
                    move || {
                        if can_share.get() {
                            view! {
                                <button
                                    class="stela--visual-motion"
                                    class=variant_class
                                    class=color_class
                                    on:click={let share_data = share_data.clone(); move |_| {
                                        let share_data = share_data.clone();
                                        spawn_local(async move {
                                            if let Some(window) = web_sys::window() {
                                                let navigator = window.navigator();
                                                let promise = navigator.share_with_data(&share_data);
                                                let _res = wasm_bindgen_futures::JsFuture::from(promise)
                                                    .await;
                                            }
                                        });
                                    }}
                                >
                                    {
                                        icon
                                        .clone()
                                        .map(|icon| {
                                            view! {
                                                <MotionIcon icon=icon is_toggled=is_toggled.into() />
                                            }
                                        })}
                                    {title.clone()}
                                </button>
                            }
                                .into_view()
                        } else {
                            view! {}.into_view()
                        }
                    }
                }
            }
            .into_view()
        }
        stela::Motion::Submit(_) => view! {
            <button
                type="submit"
                name="submit"
                value=""
                class="stela--visual-motion"
                class=variant_class
                class=color_class
            >
                {motion
                    .icon
                    .map(|icon| view! { <MotionIcon icon=icon is_toggled=is_toggled.into() /> })}
                {motion.title}
            </button>
        }
        .into_view(),
        stela::Motion::Unknown => view! {}.into_view(),
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
