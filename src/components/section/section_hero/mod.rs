use std::rc::Rc;

use anaso_site_api_models::stela;
use leptos::*;

use super::SectionCard;

#[component]
pub fn SectionHero(section: Rc<stela::SectionHero>, border: bool) -> impl IntoView {
    let hero: Rc<stela::Hero> = section.hero.clone();
    view! {
        <SectionCard
            border=border
            background_image_light=hero.background_image_light.clone()
            background_image_dark=hero.background_image_dark.clone()
            background_blur=5.0
        >
            {hero.title.clone().map(|text| view! { <h2>{text}</h2> })}
            {hero.description.clone().map(|text| view! { <p>{text}</p> })}
        </SectionCard>
    }
}
