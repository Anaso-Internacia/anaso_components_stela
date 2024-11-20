use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::*;

use crate::Section;

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
            <div class="stela--page--hero">
                {page.title.clone().map(|text| view! { <h1>{text}</h1> })}
            </div>
            <div class="stela--page--sidebar">
                <div class="stela--page--sidebar-content">
                    <div style="background-color: red; width: 128px">"Sidebar"</div>
                </div>
            </div>
            <div class="stela--page--sections-list">{sections}</div>
        </div>
    }
}
