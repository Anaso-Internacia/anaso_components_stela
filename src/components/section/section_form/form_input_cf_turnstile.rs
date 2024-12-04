use std::sync::Arc;

use anaso_site_api_models::stela;
use leptos::*;

#[component]
pub fn FormInputCfTurnstile(input: Arc<stela::FormInputCfTurnstile>) -> impl IntoView {
    view! {
        <div
            class=input.class.clone()
            data-sitekey=input.sitekey.clone()
            data-response-field-name=input.response_field_name.clone()
            data-size=input.size.clone()
            data-language=input.language.clone()
        ></div>
        <script>"delete window.turnstile;"</script>
        <script
            id="cloudflare-challenge-api"
            src="https://challenges.cloudflare.com/turnstile/v0/api.js"
            async_="true"
            defer="true"
        />
    }
}
