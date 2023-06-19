use std::fmt::{Debug, Display, Formatter};

use async_trait::async_trait;
use chatgpt::client;
use miette::{IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Debug)]
pub struct ChatGPT {
    client: client::ChatGPT,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(transparent)]
pub struct Key(String);

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Key {
    fn from(v: String) -> Self {
        Self(v)
    }
}

impl From<Key> for String {
    fn from(v: Key) -> Self {
        v.0
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(transparent)]
pub struct Direction(String);

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Direction {
    fn from(v: String) -> Self {
        Self(v)
    }
}

impl From<Direction> for String {
    fn from(v: Direction) -> Self {
        v.0
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(transparent)]
pub struct Prompt(String);

impl Display for Prompt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Prompt {
    fn from(v: String) -> Self {
        Self(v)
    }
}

impl From<Prompt> for String {
    fn from(v: Prompt) -> Self {
        v.0
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(transparent)]
pub struct Message(String);

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Message {
    fn from(v: String) -> Self {
        Self(v)
    }
}

impl From<Message> for String {
    fn from(v: Message) -> Self {
        v.0
    }
}

#[async_trait]
pub trait Repository {
    async fn generate_text<
        D: Into<Direction> + Debug + Sync + Send,
        P: Into<Prompt> + Debug + Sync + Send,
    >(
        &self,
        direction: D,
        prompt: P,
    ) -> Result<Message>;
}

#[async_trait]
impl Repository for ChatGPT {
    #[instrument]
    async fn generate_text<D, P>(&self, direction: D, prompt: P) -> Result<Message>
    where
        D: Into<Direction> + Debug + Sync + Send,
        P: Into<Prompt> + Debug + Sync + Send,
    {
        let response = self
            .client
            .new_conversation_directed(String::from(direction.into()))
            .send_message(String::from(prompt.into()))
            .await
            .into_diagnostic()?;
        let message = response.message().clone().content;
        Ok(message.into())
    }
}

impl ChatGPT {
    // Instrument panic is false positive
    #[allow(clippy::panic_in_result_fn)]
    #[instrument]
    pub fn try_new<T: Into<Key> + Debug>(key: T) -> Result<Self> {
        let client = client::ChatGPT::new(key.into()).into_diagnostic()?;

        Ok(Self { client })
    }
}

#[cfg(test)]
mod tests {
    use super::{Direction, Key, Message, Prompt};

    #[test]
    fn direction_is_a_string_in_json() {
        let direction: Direction = "test".to_string().into();
        assert_eq!(
            serde_json::to_string(&direction).expect("Failed to serialize"),
            "\"test\"".to_string()
        );
    }

    #[test]
    fn direction_can_be_made_from_string() {
        let direction: Direction = "test".to_string().into();
        assert_eq!(direction, Direction("test".to_string()));
    }

    #[test]
    fn direction_implements_display() {
        let direction = Direction("test".to_string());
        assert_eq!(direction.to_string(), "test");
    }

    #[test]
    fn direction_implements_into_string() {
        let direction: String = Direction("test".to_string()).into();
        assert_eq!(direction, "test".to_string());
    }

    #[test]
    fn message_is_a_string_in_json() {
        let message: Message = "test".to_string().into();
        assert_eq!(
            serde_json::to_string(&message).expect("Failed to serialize"),
            "\"test\"".to_string()
        );
    }

    #[test]
    fn message_can_be_made_from_string() {
        let message: Message = "test".to_string().into();
        assert_eq!(message, Message("test".to_string()));
    }

    #[test]
    fn message_implements_display() {
        let message = Message("test".to_string());
        assert_eq!(message.to_string(), "test");
    }

    #[test]
    fn message_implements_into_string() {
        let message: String = Message("test".to_string()).into();
        assert_eq!(message, "test".to_string());
    }

    #[test]
    fn prompt_can_be_made_from_string() {
        let prompt: Prompt = "test".to_string().into();
        assert_eq!(prompt, Prompt("test".to_string()));
    }

    #[test]
    fn prompt_implements_display() {
        let prompt = Prompt("test".to_string());
        assert_eq!(prompt.to_string(), "test");
    }

    #[test]
    fn prompt_implements_into_string() {
        let prompt: String = Prompt("test".to_string()).into();
        assert_eq!(prompt, "test".to_string());
    }

    #[test]
    fn prompt_is_a_string_in_json() {
        let prompt: Prompt = "test".to_string().into();
        assert_eq!(
            serde_json::to_string(&prompt).expect("Failed to serialize"),
            "\"test\"".to_string()
        );
    }

    #[test]
    fn key_can_be_made_from_string() {
        let key: Key = "test".to_string().into();
        assert_eq!(key, Key("test".to_string()));
    }

    #[test]
    fn key_implements_display() {
        let key = Key("test".to_string());
        assert_eq!(key.to_string(), "test");
    }

    #[test]
    fn key_implements_into_string() {
        let key: String = Key("test".to_string()).into();
        assert_eq!(key, "test".to_string());
    }

    #[test]
    fn key_is_a_string_in_json() {
        let key: Key = "test".to_string().into();
        assert_eq!(
            serde_json::to_string(&key).expect("Failed to serialize"),
            "\"test\"".to_string()
        );
    }
}
