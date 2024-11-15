use std::rc::Rc;

use anaso_site_api_models::stela;
use leptos::*;

use crate::Section;

#[component]
pub fn Page(page: Rc<stela::Page>) -> impl IntoView {
    let sections = page
        .sections
        .iter()
        .map(|section| {
            view! {
                <Section border=section.bordered.unwrap_or_default() section=section.section.clone() />
            }
        })
        .collect_view();

    view! {
        {page.title.clone().map(|text| view! {<h1>{text}</h1>})}
        {sections}
    }
}
