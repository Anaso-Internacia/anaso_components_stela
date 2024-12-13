use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::*;
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

    view! {
        <div class="stela--page">
            {page.title.clone().map(|title| view! { <Title text=title /> })}
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
