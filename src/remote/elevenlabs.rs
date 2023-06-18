use chatgpt::prelude::Url;

use super::super::io::audio::Audio;
use super::super::io::audio::VecU8A;
use async_trait::async_trait;
use miette::IntoDiagnostic;
use miette::Result;
use reqwest::header::HeaderMap;
use tracing::instrument;

#[instrument]
pub async fn text_to_speech<I: Audio + From<Vec<u8>>>(
    client: &reqwest::Client,
    voice: &str,
    message: &str,
) -> Result<I> {
    let mut url = Url::parse("https://api.elevenlabs.io/v1/text-to-speech").into_diagnostic()?;
    url.path_segments_mut()
        .expect("Infallible")
        .extend(&[voice]);

    let stream = client
        .post(url)
        .header("accept", "audio/mpeg")
        .json(&serde_json::json!({
            "text": &message,
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

#[derive(Debug)]
pub struct Reqwest {
    client: reqwest::Client,
}

#[async_trait]
pub trait Repository<T: Audio> {
    async fn text_to_speech(&self, voice: String, message: String) -> Result<T>;
}

#[async_trait]
impl Repository<VecU8A> for Reqwest {
    #[instrument]
    async fn text_to_speech(&self, voice: String, message: String) -> Result<VecU8A> {
        text_to_speech(&self.client, &voice, &message).await
    }
}

impl Reqwest {
    /// Create a new instance of the Eleven Labs API client.
    // Instrument panic is false positive
    #[allow(clippy::panic_in_result_fn)]
    #[instrument]
    pub fn try_new(key: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("xi-api-key", key.try_into().into_diagnostic()?);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .into_diagnostic()?;

        Ok(Self { client })
    }
}
