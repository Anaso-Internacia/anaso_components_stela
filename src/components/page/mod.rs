use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::prelude::*;
use leptos_meta::*;

use crate::{Section, VisualMotion};

#[component]
pub fn Page(page: Arc<stela::Page>) -> impl IntoView {
    let sections = page
        .sections
        .iter()
        .map(|section| {
            view! { <Section border=section.bordered.unwrap_or_default() section=section.section.clone() /> }
        })
        .collect_view();

    let meta_tags = page.social.clone().map(|social| {
        // TODO: After leptos 0.7, uncomment lines with `itemprop`
        view! {
            {social
                .title
                .as_ref()
                .map(|title| {
                    view! {
                        // <Meta itemprop="name" content=title.clone()/>
                        <Meta property="og:title" content=title.clone() />
                        <Meta name="twitter:title" content=title.clone() />
                    }
                })}
            {social
                .description
                .as_ref()
                .map(|desc| {
                    view! {
                        <Meta name="description" content=desc.clone() />
                        // <Meta itemprop="description" content=desc.clone()/>
                        <Meta property="og:description" content=desc.clone() />
                        <Meta name="twitter:description" content=desc.clone() />
                    }
                })}
            {social
                .image
                .as_ref()
                .map(|image| {
                    view! {
                        // <Meta itemprop="image" content=image.clone()/>
                        <Meta property="og:image" content=image.clone() />
                        <Meta name="twitter:image" content=image.clone() />
                    }
                })}
            {social
                .url
                .as_ref()
                .map(|url| view! { <Meta property="og:url" content=url.clone() /> })}
            {social
                .twitter_card
                .as_ref()
                .map(|card| view! { <Meta name="twitter:card" content=card.clone() /> })}
        }
    });

    view! {
        <div class="stela--page">
            {page.title.clone().map(|title| view! { <Title text=title /> })} {meta_tags}
            {page
                .hero
                .clone()
                .map(|hero| {
                    let motions = hero
                        .motions
                        .clone()
                        .into_iter()
                        .map(|motion| {
                            // 
                            view! { <VisualMotion motion=motion /> }
                        })
                        .collect_view();
                    view! {
                        <div class="stela--page--hero">
                            {hero.title.clone().map(|text| view! { <h1>{text}</h1> })}
                            {hero.description.clone().map(|text| view! { <p>{text}</p> })}
                            <div class="stela--hero--motion-bar">{motions}</div>
                        </div>
                    }
                })}
            {page
                .sidebar
                .clone()
                .map(|_sidebar| {
                    view! {
                        <div class="stela--page--sidebar">

                            <div class="stela--page--sidebar-content">
                                <div style="background-color: red; width: 128px">"Sidebar"</div>
                            </div>
                        </div>
                    }
                })}<div class="stela--page--sections-list">{sections}</div>
        </div>
    }
}
