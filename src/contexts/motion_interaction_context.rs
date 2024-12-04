use std::{ops::Deref, sync::Arc};

use anaso_site_api_models::stela::MotionApiCallResponse;
use leptos::*;

#[derive(Clone, Copy)]
pub struct MotionInteractionContext(pub StoredValue<Arc<dyn MotionInteractionContextTrait>>);

impl Deref for MotionInteractionContext {
    type Target = StoredValue<Arc<dyn MotionInteractionContextTrait>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait::async_trait]
pub trait MotionInteractionContextTrait {
    async fn emit(
        &self,
        data: String,
        toggle_state: Option<bool>,
    ) -> Result<Arc<MotionApiCallResponse>, ServerFnError>;
}
