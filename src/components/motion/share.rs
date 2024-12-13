use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn MotionShare(
    share: Arc<stela::MotionShare>,
    children: ChildrenFn,
    /// Sets the `class` attribute on the underlying element, making it easier to style.
    class: Option<String>,
) -> impl IntoView {
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

    view! {
        {
            let share_data = share_data.clone();
            let class = class.clone();
            move || {
                if can_share.get() {
                    view! {
                        <button
                            class=class.clone()
                            on:click={
                                let share_data = share_data.clone();
                                move |_| {
                                    let share_data = share_data.clone();
                                    spawn_local(async move {
                                        if let Some(window) = web_sys::window() {
                                            let navigator = window.navigator();
                                            let promise = navigator.share_with_data(&share_data);
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
                        .into_view()
                } else {
                    view! {}.into_view()
                }
            }
        }
    }
}
