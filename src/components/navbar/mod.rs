use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::prelude::*;
use phosphor_leptos::{Icon, MAGNIFYING_GLASS};

use crate::{Motion, VisualMotion};

#[component]
pub fn Navbar(navbar: Arc<stela::Navbar>) -> impl IntoView {
    let right_side_motions = navbar
        .right_side_motions
        .iter()
        .map(|motion| view! { <VisualMotion motion=motion.clone() /> })
        .collect_view();

    let image_base = "https://ana.so/cdn-cgi/image/quality=80,h=25/cdn-cgi/imagedelivery/MRTPzGIpYfy00UVryjholQ/";
    let icon_url = [image_base, &navbar.left_side_icon_image.id, "/public"].concat();

    let search_text = navbar.search_text.clone();

    view! {
        <div class="stela--navbar">
            <div class="safe-area-inset-top" />
            <div class="stela--navbar--content">
                <div class="stela--navbar--left-side">
                    <Motion motion=navbar
                        .left_side_motion
                        .clone()>
                        {
                            let icon_url = icon_url.clone();
                            move || {
                                view! {
                                    <img src=icon_url.clone() class="stela--navbar--primary-icon" />
                                }
                            }
                        }
                    </Motion>
                </div>
                {navbar
                    .search_motion
                    .clone()
                    .map(|motion| {
                        view! {
                            <Motion motion=motion>
                                {
                                    let search_text = search_text.clone();
                                    move || {
                                        view! {
                                            <div class="stela--navbar--search-button">
                                                <Icon
                                                    icon=MAGNIFYING_GLASS
                                                    attr:class="stela--navbar--search-icon"
                                                />
                                                {search_text
                                                    .clone()
                                                    .map(|text| {
                                                        view! {
                                                            <span class="stela--navbar--search-text">{text}</span>
                                                        }
                                                    })}
                                            </div>
                                        }
                                    }
                                }
                            </Motion>
                        }
                    })}
                <div class="stela--navbar--right-side">{right_side_motions}</div>
            </div>
        </div>
    }
}
