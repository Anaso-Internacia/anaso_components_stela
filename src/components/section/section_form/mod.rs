use std::sync::Arc;

use anaso_site_api_models::stela;
use js_sys::wasm_bindgen::JsCast;
use leptos::either::EitherOf9;
use leptos::ev;
use leptos::prelude::*;
use leptos_router::components::Redirect;
use phosphor_leptos::Icon;
use phosphor_leptos::CHECK_CIRCLE;
use server_fn::codec::MultipartData;
use web_sys::HtmlButtonElement;
use web_sys::HtmlFormElement;
use web_sys::HtmlInputElement;

use crate::FormSubmitContext;

use super::SectionCard;

use self::form_input_cf_turnstile::*;
use self::form_input_checkbox::*;
use self::form_input_image::*;
use self::form_input_markdown::*;
use self::form_input_motions::*;
use self::form_input_radio::*;
use self::form_input_subsection::*;
use self::form_input_text::*;

mod form_input_cf_turnstile;
mod form_input_checkbox;
mod form_input_image;
mod form_input_markdown;
mod form_input_motions;
mod form_input_radio;
mod form_input_subsection;
mod form_input_text;

#[component]
pub fn SectionForm(border: bool, section: Arc<stela::SectionForm>) -> impl IntoView {
    let form_submit = use_context::<FormSubmitContext>().unwrap();
    let error = RwSignal::new(None);

    let action = Action::new_local(move |data: &web_sys::FormData| {
        let data: MultipartData = data.clone().into();
        async move { form_submit.0.get_value().submit(data).await }
    });

    let value = action.value();

    let on_submit = move |event: ev::SubmitEvent| {
        event.prevent_default();
        match form_data_from_event(&event) {
            Err(_e) => {
                error.try_set(Some(String::from("error making form data")));
            }
            Ok(data) => {
                action.dispatch_local(data);
            }
        }
    };

    let error_text = Signal::derive(move || {
        value
            .get()
            .and_then(|v| v.ok())
            .and_then(|v| v.error.clone())
    });

    let success_text = Memo::new(move |_| {
        value
            .get()
            .and_then(|v| v.ok())
            .and_then(|v| v.success.clone())
    });

    let redirect = Memo::new(move |_| {
        value
            .get()
            .and_then(|v| v.ok())
            .and_then(|v| v.redirect.clone())
    });

    view! {
        {move || {
            redirect.get().map(|redirect| {
                view! {
                    <Redirect path=redirect />
                }
            })
        }}
        {move || {
            if let Some(success_text) = success_text.get() {
                view! {
                    <SectionCard border=false class="stela--form-success">
                        <div class="stela--section--padded">
                            <Icon icon=CHECK_CIRCLE size="36px" />
                            <span>{success_text}</span>
                        </div>
                    </SectionCard>
                }
            } else {
                let section = section.clone();
                let inputs = section
                    .inputs
                    .iter()
                    .map(|input| view! { <FormInput input=input.clone() /> })
                    .collect_view();
                view! {
                    <SectionCard border=border>
                        <form
                            action=form_submit.get_value().url()
                            method="POST"
                            on:submit=on_submit
                            class="stela--form stela--section--padded"
                            enctype="multipart/form-data"
                        >
                            {section.header.clone().map(|text| view! { <h2>{text}</h2> })}
                            {section.subheader.clone().map(|text| view! { <p>{text}</p> })}
                            <input type="hidden" name="form_name" value=section.form_name.clone() />
                            <input type="hidden" name="extra_data" value=section.extra_data.clone() />
                            {move || {
                                error_text
                                    .get()
                                    .map(|error_text| {
                                        view! { <p class="stela--form--error">{error_text}</p> }
                                    })
                            }}
                            {inputs}
                        </form>
                    </SectionCard>
                }
            }
        }}
    }
}

#[component]
fn FormInput(input: stela::FormInput) -> impl IntoView {
    match input {
        stela::FormInput::Text(input) => {
            EitherOf9::A(view! { <FormInputText input=input /> }.into_view())
        }
        stela::FormInput::Checkbox(input) => {
            EitherOf9::B(view! { <FormInputCheckbox input=input /> }.into_view())
        }
        stela::FormInput::CfTurnstile(input) => {
            EitherOf9::C(view! { <FormInputCfTurnstile input=input /> }.into_view())
        }
        stela::FormInput::Image(input) => {
            EitherOf9::D(view! { <FormInputImage input=input /> }.into_view())
        }
        stela::FormInput::Markdown(input) => {
            EitherOf9::E(view! { <FormInputMarkdown input=input /> }.into_view())
        }
        stela::FormInput::Motions(input) => {
            EitherOf9::F(view! { <FormInputMotions input=input /> }.into_view())
        }
        stela::FormInput::Radio(input) => {
            EitherOf9::G(view! { <FormInputRadio input=input /> }.into_view())
        }
        stela::FormInput::Subsection(input) => {
            EitherOf9::H(view! { <FormInputSubsection input=input /> }.into_view())
        }
        stela::FormInput::Tabs(_) => todo!(),
        stela::FormInput::Unknown => EitherOf9::I(().into_view()),
    }
}

fn form_data_from_event(ev: &ev::SubmitEvent) -> Result<web_sys::FormData, FromFormDataError> {
    let submitter = ev.submitter();
    let mut submitter_name_value = None;
    let opt_form = match &submitter {
        Some(el) => {
            if let Some(form) = el.dyn_ref::<HtmlFormElement>() {
                Some(form.clone())
            } else if let Some(input) = el.dyn_ref::<HtmlInputElement>() {
                submitter_name_value = Some((input.name(), input.value()));
                Some(ev.target().unwrap().unchecked_into())
            } else if let Some(button) = el.dyn_ref::<HtmlButtonElement>() {
                submitter_name_value = Some((button.name(), button.value()));
                Some(ev.target().unwrap().unchecked_into())
            } else {
                None
            }
        }
        None => ev.target().map(|form| form.unchecked_into()),
    };
    match opt_form.as_ref().map(web_sys::FormData::new_with_form) {
        None => Err(FromFormDataError::MissingForm(ev.clone().into())),
        Some(Err(e)) => Err(FromFormDataError::FormData(e)),
        Some(Ok(form_data)) => {
            if let Some((name, value)) = submitter_name_value {
                form_data
                    .append_with_str(&name, &value)
                    .map_err(FromFormDataError::FormData)?;
            }
            Ok(form_data)
        }
    }
}
