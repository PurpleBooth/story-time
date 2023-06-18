use async_trait::async_trait;
use miette::{IntoDiagnostic, Result};
use std::io::Cursor;
use std::path::Path;
use tokio::io::AsyncWriteExt;
use tracing::instrument;

#[derive(Debug)]
pub struct VecU8A {
    stream: Vec<u8>,
}

#[async_trait]
pub trait Audio {
    fn play(&self) -> Result<()>;
    async fn save(&self, path: &Path) -> Result<()>;
}

#[async_trait]
impl Audio for VecU8A {
    // Instrument panic is false positive
    #[allow(clippy::panic_in_result_fn)]
    #[instrument]
    fn play(&self) -> Result<()> {
        let stream = self.stream.clone();
        let cursor = Cursor::new(stream);

        let (_stream, stream_handle) = rodio::OutputStream::try_default().into_diagnostic()?;

        let player = stream_handle.play_once(cursor).into_diagnostic()?;
        player.sleep_until_end();
        Ok(())
    }

    #[instrument]
    async fn save(&self, path: &Path) -> Result<()> {
        let mut file = tokio::fs::File::create(path).await.into_diagnostic()?;
        file.write_all(&self.stream).await.into_diagnostic()?;
        Ok(())
    }
}

impl From<Vec<u8>> for VecU8A {
    #[instrument]
    fn from(stream: Vec<u8>) -> Self {
        Self { stream }
    }
}
