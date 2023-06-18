use async_trait::async_trait;
use miette::{IntoDiagnostic, Result};
use std::io::Cursor;
use std::path::Path;
use tokio::io::AsyncWriteExt;
use tracing::instrument;

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

#[derive(Debug)]
pub struct Impl {
    stream: Vec<u8>,
}

#[async_trait]
pub trait Audio {
    fn play(&self) -> Result<()>;
    async fn save(&self, path: &Path) -> Result<()>;
}

#[async_trait]
impl Audio for Impl {
    // Instrument panic is false positive
    #[allow(clippy::panic_in_result_fn)]
    #[instrument]
    fn play(&self) -> Result<()> {
        play_audio(self.stream.clone())
    }

    #[instrument]
    async fn save(&self, path: &Path) -> Result<()> {
        let mut file = tokio::fs::File::create(path).await.into_diagnostic()?;
        file.write_all(&self.stream).await.into_diagnostic()?;
        Ok(())
    }
}

impl From<Vec<u8>> for Impl {
    #[instrument]
    fn from(stream: Vec<u8>) -> Self {
        Self { stream }
    }
}
