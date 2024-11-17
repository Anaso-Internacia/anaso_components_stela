use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::*;

use crate::VisualMotion;

use super::SectionCard;

#[component]
pub fn SectionHero(section: Arc<stela::SectionHero>, border: bool) -> impl IntoView {
    let hero: Arc<stela::Hero> = section.hero.clone();

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
        <SectionCard
            border=border
            background_image_light=hero.background_image_light.clone()
            background_image_dark=hero.background_image_dark.clone()
            background_blur=5.0
        >
            {hero.title.clone().map(|text| view! { <h2>{text}</h2> })}
            {hero.description.clone().map(|text| view! { <p>{text}</p> })}
            <div>{motions}</div>
        </SectionCard>
    }
}
