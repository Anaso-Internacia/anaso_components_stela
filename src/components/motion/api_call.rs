use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::{prelude::*, task::spawn_local};
use leptos_router::components::*;

use crate::MotionInteractionContext;

#[component]
pub fn MotionApiCall(
    api_call: Arc<stela::MotionApiCall>,
    children: ChildrenFn,
    is_toggled: Option<RwSignal<bool>>,
    text: Option<RwSignal<Option<String>>>,
    icon: Option<RwSignal<Option<stela::MotionIcon>>>, // TODO: api will send a new icon
    /// Sets the `class` attribute on the underlying element, making it easier to style.
    class: Option<String>,
) -> impl IntoView {
    let _ = icon;

    let motion_interaction_context = use_context::<MotionInteractionContext>().unwrap();
    let redirect = RwSignal::new(None);

    view! {
        <a
            href=["/motion_interaction/", &urlencoding::encode(&api_call.data)].concat()
            class=class
            on:click={
                let data = api_call.data.clone();
                move |ev| {
                    ev.prevent_default();
                    let data = data.clone();
                    let old_toggle = if let Some(is_toggled) = is_toggled {
                        let old = is_toggled.get_untracked();
                        is_toggled.update(|v| *v ^= true);
                        old
                    } else {
                        false
                    };
                    spawn_local(async move {
                        let api = motion_interaction_context.get_value();
                        let res = api.emit(data, Some(old_toggle)).await;
                        match res {
                            Ok(res) => {
                                if let Some(is_toggled) = is_toggled {
                                    is_toggled.try_set(res.new_toggle.unwrap_or(old_toggle));
                                }
                                if let Some(text) = text {
                                    if let Some(new_text) = &res.new_text {
                                        text.try_set(Some(new_text.clone()));
                                    }
                                }
                                if let Some(new_redirect) = &res.redirect {
                                    redirect.try_set(Some(new_redirect.clone()));
                                }
                            }
                            Err(_) => {
                                if let Some(is_toggled) = is_toggled {
                                    is_toggled.try_set(old_toggle);
                                }
                            }
                        }
                    });
                }
            }
        >
            {move || redirect.get().map(|redirect| view! { <Redirect path=redirect /> })}
            {children()}
        </a>
    }
}
