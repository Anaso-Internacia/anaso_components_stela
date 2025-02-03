use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::{either::Either, prelude::*};

use crate::{components::section::section_card::SectionCard, Motion, VisualMotion};

#[component]
pub fn SectionPost(section: Arc<stela::SectionPost>, border: bool) -> impl IntoView {
    let motion = section.motion.clone();
    let motions_tl = section
        .motions_tl
        .iter()
        .map(|motion| view! { <VisualMotion motion=motion.clone() /> })
        .collect_view();
    let motions_tr = section
        .motions_tr
        .iter()
        .map(|motion| view! { <VisualMotion motion=motion.clone() /> })
        .collect_view();
    let motions_bl = section
        .motions_bl
        .iter()
        .map(|motion| view! { <VisualMotion motion=motion.clone() /> })
        .collect_view();
    let motions_br = section
        .motions_br
        .iter()
        .map(|motion| view! { <VisualMotion motion=motion.clone() /> })
        .collect_view();

    view! {
        <SectionCard
            border=border
            background_image_dark=section.background.clone()
            background_image_light=section.background.clone()
        >
            <div class="stela--post">
                <div class="stela--section--padded">
                    {section
                        .title
                        .clone()
                        .map(|text| {
                            view! {
                                <h2>
                                    {if let Some(motion) = motion {
                                        Either::Left(
                                            view! { <Motion motion=motion>{text.clone()}</Motion> },
                                        )
                                    } else {
                                        Either::Right(view! { {text} })
                                    }}
                                </h2>
                            }
                        })} <div class="stela--post--motion-bar">
                        <div>{motions_tl}</div>
                        <div>{motions_tr}</div>
                    </div>
                </div>
                {section
                    .image
                    .clone()
                    .map(|image| {
                        view! {
                            <img src=[
                                "https://ana.so/cdn-cgi/image/quality=80,w=672/cdn-cgi/imagedelivery/MRTPzGIpYfy00UVryjholQ/",
                                &image.id,
                                "/public",
                            ]
                                .concat() />
                        }
                    })}
                <div class="stela--section--padded">
                    <div class="stela--post--content">
                        {section.body_html.clone().map(|html| view! { <div inner_html=html /> })}
                    </div>
                    <div class="stela--post--motion-bar">
                        <div>{motions_bl}</div>
                        <div>{motions_br}</div>
                    </div>
                </div>
            </div>
        </SectionCard>
    }
}
