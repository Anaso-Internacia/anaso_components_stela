use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::{prelude::*, task::spawn_local};

#[component]
pub fn MotionShare(
    share: Arc<stela::MotionShare>,
    children: ChildrenFn,
    /// Sets the `class` attribute on the underlying element, making it easier to style.
    class: Option<String>,
) -> impl IntoView {
    let (can_share, set_can_share) = signal(false);

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

    Effect::new({
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

    let share_data = StoredValue::new_local(share_data);

    view! {
        {
            let class = class.clone();
            move || {
                if can_share.get() {
                    Some(view! {
                        <button
                            class=class.clone()
                            on:click={
                                move |_| {
                                    spawn_local(async move {
                                        if let Some(window) = web_sys::window() {
                                            let navigator = window.navigator();
                                            let promise = navigator.share_with_data(&share_data.get_value());
                                            let _res = wasm_bindgen_futures::JsFuture::from(promise)
                                                .await;
                                        }
                                    });
                                }
                            }
                        >
                            {children()}
                        </button>
                    }
                    )
                } else {
                    None
                }
            }
        }
    }
}
