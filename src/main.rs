//! Story time
//!
//! Read aloud something generated by chatgpt
#![warn(
    rust_2018_idioms,
    unused,
    rust_2021_compatibility,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::unwrap_used,
    clippy::missing_assert_message,
    clippy::todo,
    clippy::allow_attributes_without_reason,
    clippy::panic,
    clippy::panicking_unwrap,
    clippy::panic_in_result_fn
)]

use chatgpt::prelude::*;
use clap::Parser;
use miette::IntoDiagnostic;
use miette::Result;
use std::io::Cursor;
use tokio::fs::File;

use std::path::PathBuf;

use tokio::io::AsyncWriteExt;
use tracing::instrument;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Read out a story
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Key for chatgpt
    #[arg(short, long, env)]
    chat_gpt_key: String,
    /// Key for elevenlabs
    #[arg(short, long, env)]
    elevenlabs_key: String,
    /// Prompt
    #[arg(short, long, env)]
    prompt: String,
    /// A style to read in
    #[arg(short, long, env, default_value = "You are reading aloud")]
    style: String,

    #[arg(short, long, env, default_value = "MF3mGyEYCl7XYWbV9V6O")]
    voice: String,

    /// Save to a file rather than reading aloud
    #[arg(short, long, env)]
    output: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    o11y()?;

    let args = Args::parse();

    let chatgpt_key: &str = &args.chat_gpt_key;
    let style: &str = &args.style;
    let prompt: &str = &args.prompt;
    let message = generate_text(chatgpt_key, style, prompt).await?;

    let elevenlabs_key: &str = &args.elevenlabs_key;
    let voice: &str = &args.voice;

    let stream: Vec<u8> = text_to_speech(elevenlabs_key, voice, &message).await?;

    if let Some(path) = args.output {
        let mut file: File = File::create(path).await.into_diagnostic()?;
        file.write_all(&stream).await.into_diagnostic()?;
    } else {
        play_audio(stream)?;
    }

    Ok(())
}

fn o11y() -> Result<()> {
    miette::set_panic_hook();

    let fmt_layer = fmt::layer();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .into_diagnostic()?;

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
    Ok(())
}

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

// Macro has panic in
#[allow(clippy::panic_in_result_fn)]
#[instrument]
fn play_audio(stream: Vec<u8>) -> Result<()> {
    let cursor = Cursor::new(stream);

    let (_stream, stream_handle) = rodio::OutputStream::try_default().into_diagnostic()?;

    let player = stream_handle.play_once(cursor).into_diagnostic()?;
    player.sleep_until_end();
    Ok(())
}

#[instrument]
async fn text_to_speech(elevenlabs_key: &str, voice: &str, message: &str) -> Result<Vec<u8>> {
    let client = reqwest::Client::new();

    let mut url = Url::parse("https://api.elevenlabs.io/v1/text-to-speech").into_diagnostic()?;
    url.path_segments_mut()
        .expect("Infallible")
        .extend(&[voice]);

    let stream = client
        .post(url)
        .header("accept", "audio/mpeg")
        .header("xi-api-key", elevenlabs_key)
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
    Ok(stream.to_vec())
}
