use chatgpt::prelude::Url;
use std::fmt::{Debug, Display, Formatter};

use super::super::io::audio::Audio;
use super::super::io::audio::VecU8A;
use async_trait::async_trait;
use miette::IntoDiagnostic;
use miette::Result;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Debug)]
pub struct Reqwest {
    client: reqwest::Client,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(transparent)]
pub struct Voice(String);

impl Display for Voice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Voice {
    fn from(v: String) -> Self {
        Self(v)
    }
}

impl From<Voice> for String {
    fn from(v: Voice) -> Self {
        v.0
    }
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

impl From<crate::chatgpt::Message> for Message {
    fn from(v: crate::chatgpt::Message) -> Self {
        Self(v.to_string())
    }
}

impl From<Message> for String {
    fn from(v: Message) -> Self {
        v.0
    }
}

#[async_trait]
pub trait Repository<T: Audio> {
    async fn text_to_speech<
        V: Into<Voice> + Debug + Sync + Send,
        M: Into<Message> + Debug + Sync + Send,
    >(
        &self,
        voice: V,
        message: M,
    ) -> Result<T>;
}

#[async_trait]
impl Repository<VecU8A> for Reqwest {
    #[instrument]
    async fn text_to_speech<
        V: Into<Voice> + Debug + Sync + Send,
        M: Into<Message> + Debug + Sync + Send,
    >(
        &self,
        voice: V,
        message: M,
    ) -> Result<VecU8A> {
        let client = &self.client;
        let mut url =
            Url::parse("https://api.elevenlabs.io/v1/text-to-speech").into_diagnostic()?;
        url.path_segments_mut()
            .expect("Infallible")
            .extend(&[&String::from(voice.into())]);

        let stream = client
            .post(url)
            .header("accept", "audio/mpeg")
            .json(&serde_json::json!({
                "text": &message.into(),
                "model_id": "eleven_monolingual_v1",
            }))
            .send()
            .await
            .into_diagnostic()?
            .error_for_status()
            .into_diagnostic()?
            .bytes()
            .await
            .into_diagnostic()?;
        Ok(stream.to_vec().into())
    }
}

impl Reqwest {
    /// Create a new instance of the Eleven Labs API client.
    // Instrument panic is false positive
    #[allow(clippy::panic_in_result_fn)]
    #[instrument]
    pub fn try_new<T: Into<Key> + Debug>(key: T) -> Result<Self> {
        let mut headers = HeaderMap::new();
        let key = key.into().to_string();
        headers.insert("xi-api-key", key.try_into().into_diagnostic()?);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .into_diagnostic()?;

        Ok(Self { client })
    }
}

#[cfg(test)]
mod tests {
    use crate::remote::elevenlabs::{Key, Message, Voice};

    #[test]
    fn message_can_be_made_from_chatgpt_message() {
        let message: Message = crate::chatgpt::Message::from("test".to_string()).into();
        assert_eq!(message, Message("test".to_string()));
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
    fn message_is_a_string_in_json() {
        let message: Message = "test".to_string().into();
        assert_eq!(
            serde_json::to_string(&message).expect("Failed to serialize"),
            "\"test\"".to_string()
        );
    }

    #[test]
    fn voice_can_be_made_from_string() {
        let voice: Voice = "test".to_string().into();
        assert_eq!(voice, Voice("test".to_string()));
    }

    #[test]
    fn voice_implements_display() {
        let voice = Voice("test".to_string());
        assert_eq!(voice.to_string(), "test");
    }

    #[test]
    fn voice_implements_into_string() {
        let voice: String = Voice("test".to_string()).into();
        assert_eq!(voice, "test".to_string());
    }

    #[test]
    fn voice_is_a_string_in_json() {
        let key: Key = "test".to_string().into();
        assert_eq!(
            serde_json::to_string(&key).expect("Failed to serialize"),
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
