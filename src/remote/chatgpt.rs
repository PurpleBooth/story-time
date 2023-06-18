use async_trait::async_trait;
use chatgpt::client;
use miette::IntoDiagnostic;
use miette::Result;
use std::fmt::Debug;

use tracing::instrument;

#[derive(Debug)]
pub struct ChatGPT {
    client: client::ChatGPT,
}

#[async_trait]
pub trait Repository {
    async fn generate_text(&self, style: String, prompt: String) -> Result<String>;
}

#[async_trait]
impl Repository for ChatGPT {
    #[instrument]
    async fn generate_text(&self, style: String, prompt: String) -> Result<String> {
        let response = self
            .client
            .new_conversation_directed(style)
            .send_message(prompt)
            .await
            .into_diagnostic()?;
        let message = response.message().clone().content;
        Ok(message)
    }
}

impl ChatGPT {
    // Instrument panic is false positive
    #[allow(clippy::panic_in_result_fn)]
    #[instrument]
    pub fn try_new(key: String) -> Result<Self> {
        let client = client::ChatGPT::new(key).into_diagnostic()?;

        Ok(Self { client })
    }
}
