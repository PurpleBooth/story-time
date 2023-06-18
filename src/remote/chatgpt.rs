use async_trait::async_trait;
use chatgpt::client::ChatGPT;
use miette::IntoDiagnostic;
use miette::Result;
use std::fmt::Debug;

use tracing::instrument;

#[instrument]
async fn generate_text(chatgpt_key: &str, style: &str, prompt: &str) -> Result<String> {
    let client = ChatGPT::new(chatgpt_key).into_diagnostic()?;
    let response = client
        .new_conversation_directed(style)
        .send_message(prompt)
        .await
        .into_diagnostic()?;
    let message = response.message().clone().content;
    Ok(message)
}

#[derive(Debug)]
pub struct Library {
    key: String,
}

#[async_trait]
pub trait Repository {
    async fn generate_text(&self, style: String, prompt: String) -> Result<String>;
}

#[async_trait]
impl Repository for Library {
    #[instrument]
    async fn generate_text(&self, style: String, prompt: String) -> Result<String> {
        generate_text(&self.key, &style, &prompt).await
    }
}

impl Library {
    pub const fn new(key: String) -> Self {
        Self { key }
    }
}
