use std::{ops::Deref, sync::Arc};

use anaso_site_api_models::stela;
use leptos::prelude::*;
use server_fn::codec::MultipartData;

#[derive(Clone, Copy)]
pub struct FormSubmitContext(pub StoredValue<Arc<dyn FormSubmitContextTrait>, LocalStorage>);

impl Deref for FormSubmitContext {
    type Target = StoredValue<Arc<dyn FormSubmitContextTrait>, LocalStorage>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait::async_trait]
pub trait FormSubmitContextTrait {
    fn url(&self) -> &'static str;
    async fn submit(&self, data: MultipartData) -> Result<stela::FormResponse, ServerFnError>;
}
