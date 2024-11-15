use std::rc::Rc;

use anaso_site_api_models::stela;
use leptos::*;
use leptos_router::*;

use super::SectionCard;

use self::form_input_checkbox::*;
use self::form_input_image::*;
use self::form_input_markdown::*;
use self::form_input_motions::*;
use self::form_input_radio::*;
use self::form_input_subsection::*;
use self::form_input_text::*;

mod form_input_checkbox;
mod form_input_image;
mod form_input_markdown;
mod form_input_motions;
mod form_input_radio;
mod form_input_subsection;
mod form_input_text;

#[component]
pub fn SectionForm(border: bool, section: Rc<stela::SectionForm>) -> impl IntoView {
    let inputs = section
        .inputs
        .iter()
        .map(|input| view! { <FormInput input=input.clone() /> })
        .collect_view();

    view! {
        <SectionCard border=border>
            <Form action="#">
                {section.header.clone().map(|text| view! { <h2>{text}</h2> })}
                {section.subheader.clone().map(|text| view! { <p>{text}</p> })} {inputs}
            </Form>
        </SectionCard>
    }
}

#[component]
fn FormInput(input: stela::FormInput) -> impl IntoView {
    let title = input.title;
    match input.variant {
        stela::FormInputVariant::Text(input) => {
            view! { <FormInputText input=input title=title /> }.into_view()
        }
        stela::FormInputVariant::Checkbox(input) => {
            view! { <FormInputCheckbox input=input title=title /> }.into_view()
        }
        stela::FormInputVariant::Image(input) => {
            view! { <FormInputImage input=input title=title /> }.into_view()
        }
        stela::FormInputVariant::Markdown(input) => {
            view! { <FormInputMarkdown input=input title=title /> }.into_view()
        }
        stela::FormInputVariant::Motions(input) => {
            view! { <FormInputMotions input=input title=title /> }.into_view()
        }
        stela::FormInputVariant::Radio(input) => {
            view! { <FormInputRadio input=input title=title /> }.into_view()
        }
        stela::FormInputVariant::Subsection(input) => {
            view! { <FormInputSubsection input=input title=title /> }.into_view()
        }
        stela::FormInputVariant::Unknown => view! {}.into_view(),
    }
}
