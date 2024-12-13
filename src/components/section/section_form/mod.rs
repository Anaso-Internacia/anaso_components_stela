use std::sync::Arc;

use anaso_site_api_models::stela;
use js_sys::wasm_bindgen::JsCast;
use js_sys::wasm_bindgen::JsValue;
use js_sys::wasm_bindgen::UnwrapThrowExt;
use leptos::*;
use phosphor_leptos::Icon;
use phosphor_leptos::CHECK_CIRCLE;
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
    let error = create_rw_signal(None);

    let action = create_action(move |data: &stela::FormCallData| {
        let data = data.clone();
        async move { form_submit.get_value().submit(data).await }
    });

    let value = action.value();

    let on_submit = move |event: ev::SubmitEvent| {
        event.prevent_default();
        match stela::FormCallData::from_event(&event) {
            Err(_e) => {
                error.try_set(Some(String::from("error making form data")));
            }
            Ok(data) => {
                action.dispatch(data);
            }
        }
    };

    let error_text = Signal::from(move || {
        value
            .get()
            .and_then(|v| v.ok())
            .and_then(|v| v.error.clone())
    });

    let success_text = create_memo(move |_| {
        value
            .get()
            .and_then(|v| v.ok())
            .and_then(|v| v.success.clone())
    });

    view! {
        {move || {
            if let Some(success_text) = success_text.get() {
                view! {
                    <SectionCard border=false class="stela--form-success">
                        <Icon icon=CHECK_CIRCLE size="36px" />
                        <span>{success_text}</span>
                    </SectionCard>
                }
                    .into_view()
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
                            on:submit:undelegated=on_submit
                            class="stela--form"
                        >
                            {section.header.clone().map(|text| view! { <h2>{text}</h2> })}
                            {section.subheader.clone().map(|text| view! { <p>{text}</p> })}
                            <input type="hidden" name="form_name" value=section.form_name.clone() />
                            <input type="hidden" name="extra_data" value="" />
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
                    .into_view()
            }
        }}
    }
}

#[component]
fn FormInput(input: stela::FormInput) -> impl IntoView {
    match input {
        stela::FormInput::Text(input) => view! { <FormInputText input=input /> }.into_view(),
        stela::FormInput::Checkbox(input) => {
            view! { <FormInputCheckbox input=input /> }.into_view()
        }
        stela::FormInput::CfTurnstile(input) => {
            view! { <FormInputCfTurnstile input=input /> }.into_view()
        }
        stela::FormInput::Image(input) => view! { <FormInputImage input=input /> }.into_view(),
        stela::FormInput::Markdown(input) => {
            view! { <FormInputMarkdown input=input /> }.into_view()
        }
        stela::FormInput::Motions(input) => view! { <FormInputMotions input=input /> }.into_view(),
        stela::FormInput::Radio(input) => view! { <FormInputRadio input=input /> }.into_view(),
        stela::FormInput::Subsection(input) => {
            view! { <FormInputSubsection input=input /> }.into_view()
        }
        stela::FormInput::Unknown => view! {}.into_view(),
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

/// Tries to deserialize a type from form data. This can be used for client-side
/// validation during form submission.
pub trait FromFormData
where
    Self: Sized + serde::de::DeserializeOwned,
{
    /// Tries to deserialize the data, given only the `submit` event.
    fn from_event(ev: &web_sys::Event) -> Result<Self, FromFormDataError>;

    /// Tries to deserialize the data, given the actual form data.
    fn from_form_data(form_data: &web_sys::FormData) -> Result<Self, serde_qs::Error>;
}

#[derive(Debug)]
pub enum FromFormDataError {
    MissingForm(ev::Event),
    FormData(JsValue),
    Deserialization(serde_qs::Error),
}

impl<T> FromFormData for T
where
    T: serde::de::DeserializeOwned,
{
    fn from_event(ev: &ev::Event) -> Result<Self, FromFormDataError> {
        let submit_ev = ev.unchecked_ref();
        let form_data = form_data_from_event(submit_ev)?;
        Self::from_form_data(&form_data).map_err(FromFormDataError::Deserialization)
    }

    fn from_form_data(form_data: &web_sys::FormData) -> Result<Self, serde_qs::Error> {
        let data =
            web_sys::UrlSearchParams::new_with_str_sequence_sequence(form_data).unwrap_throw();
        let data = data.to_string().as_string().unwrap_or_default();
        serde_qs::Config::new(5, false).deserialize_str::<Self>(&data)
    }
}
