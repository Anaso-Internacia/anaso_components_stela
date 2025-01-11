use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn FormInputImage(input: Arc<stela::FormInputImage>) -> impl IntoView {
    view! { <input type="file" id="imageUpload" name=input.name.clone() accept="image/*" placeholder=input.title.clone() /> }
}
