use std::sync::Arc;

use anaso_site_api_models::stela;
use js_sys::wasm_bindgen::JsCast;
use leptos::{ev, prelude::*};
use web_sys::HtmlInputElement;

#[component]
pub fn FormInputText(input: Arc<stela::FormInputText>) -> impl IntoView {
    let esperanto = input.esperanto;
    let eo_button_enabled = RwSignal::new(false);
    let esperantize = RwSignal::new(input.esperanto);

    Effect::new(move |_| eo_button_enabled.set(esperanto));

    let on_btn_click = move |_| {
        esperantize.update(|v| *v ^= true);
    };

    let btn_class = Signal::derive(move || {
        if esperantize.get() {
            "eo-button active"
        } else {
            "eo-button"
        }
    });

    let on_input = move |e: ev::InputEvent| {
        // Converting esperanto
        if !esperantize.get() {
            return;
        }
        // Inserting text
        if e.input_type() != "insertText" {
            return;
        }
        // Have data
        let Some(data) = e.data() else {
            return;
        };
        // Pressing x
        if data != "x" && data != "X" {
            return;
        }
        // In an element
        let Some(target) = e.target() else {
            return;
        };
        // That is an input
        let Some(element): Option<&HtmlInputElement> = target.dyn_ref() else {
            return;
        };
        // With an index
        let Ok(Some(index)) = element.selection_start() else {
            return;
        };
        // That matches the end index
        if Ok(Some(index)) != element.selection_end() {
            return;
        }

        e.prevent_default();

        let index = (index as usize).saturating_sub(1);
        let old = element.value();
        let mut new = String::with_capacity(old.len());
        let mut offset = 1;
        for (i, c) in old.chars().enumerate() {
            if i == index {
                match c {
                    'c' => new.push('ĉ'),
                    'g' => new.push('ĝ'),
                    'h' => new.push('ĥ'),
                    'j' => new.push('ĵ'),
                    's' => new.push('ŝ'),
                    'u' => new.push('ŭ'),
                    'C' => new.push('Ĉ'),
                    'G' => new.push('Ĝ'),
                    'H' => new.push('Ĥ'),
                    'J' => new.push('Ĵ'),
                    'S' => new.push('Ŝ'),
                    'U' => new.push('Ŭ'),
                    _ => {
                        new.push(c);
                        offset += data.len() as u32;
                        new.push_str(&data);
                    }
                }
            } else {
                new.push(c);
            }
        }
        element.set_value(&new);
        let _ = element.set_selection_range(index as u32 + offset, index as u32 + offset);
    };

    view! {
        <div class="stela--form--input-text-row">
            <input
                type="text"
                name=input.name.clone()
                value=input.initial_value.clone()
                placeholder=input.title.clone()
                on:beforeinput=on_input
            />
            {move || {
                if eo_button_enabled.get() {
                    Some(
                        view! {
                            <button class=btn_class type="button" on:click=on_btn_click>
                                "X̂"
                            </button>
                        },
                    )
                } else {
                    None
                }
            }}
        </div>
    }
}
