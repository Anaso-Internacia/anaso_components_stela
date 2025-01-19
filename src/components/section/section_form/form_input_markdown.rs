use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn FormInputMarkdown(input: Arc<stela::FormInputMarkdown>) -> impl IntoView {
    view! {
        <textarea
            id="anaso-bleki-body-md"
            name=input.name.clone()
            lang="eo"
            placeholder="Enhavo"
            minlength=input.length_min
            maxlength=input.length_max
            rows="10"
        />
        <Link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/easymde/dist/easymde.min.css" />
        <script
            id="easymde-api"
            src="https://cdn.jsdelivr.net/npm/easymde/dist/easymde.min.js"
        ></script>
        <script id="bleki-markdown-editor-api" src="/bleki_markdown_editor.js"></script>
    }
}
