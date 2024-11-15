use anaso_site_api_models::stela;
use leptos::*;

use self::section_card::SectionCard;

pub use self::section_form::*;
pub use self::section_hero::*;
pub use self::section_post::*;
pub use self::section_sponsor::*;
pub use self::section_tiles::*;

mod section_card;
mod section_form;
mod section_hero;
mod section_post;
mod section_sponsor;
mod section_tiles;

#[component]
pub fn Section(border: bool, section: stela::Section) -> impl IntoView {
    match section {
        stela::Section::Form(section) => {
            view! { <SectionForm section=section border=border /> }.into_view()
        }
        stela::Section::Hero(section) => {
            view! { <SectionHero section=section border=border /> }.into_view()
        }
        stela::Section::Post(section) => {
            view! { <SectionPost section=section border=border /> }.into_view()
        }
        stela::Section::Sponsor(section) => {
            view! { <SectionSponsor section=section border=border /> }.into_view()
        }
        stela::Section::Tiles(section) => {
            view! { <SectionTiles section=section border=border /> }.into_view()
        }
        stela::Section::Unknown => view! {}.into_view(),
    }
}