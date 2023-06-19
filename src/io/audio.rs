use async_trait::async_trait;
use miette::{IntoDiagnostic, Result};
use std::fmt::Debug;
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
    async fn save<P: AsRef<Path> + Debug + Sync + Send>(&self, path: P) -> Result<()>;
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
    async fn save<P: AsRef<Path> + Debug + Sync + Send>(&self, path: P) -> Result<()> {
        let mut file = tokio::fs::File::create(path.as_ref())
            .await
            .into_diagnostic()?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn save_will_write_contents() {
        let tempdir = tempdir().expect("Failed to create tempdir");
        let path = tempdir.path().join("test.mp3");

        let stream = VecU8A::from(vec![1, 2, 3]);
        stream.save(&path).await.expect("Failed to save file");

        let contents = tokio::fs::read(path).await.expect("Failed to read file");
        assert_eq!(contents, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn save_will_overwrite_an_existing_file() {
        let tempdir = tempdir().expect("Failed to create tempdir");
        let path = tempdir.path().join("test.mp3");
        tokio::fs::write(&path, &[4, 5, 6])
            .await
            .expect("Failed to create file");

        let stream = VecU8A::from(vec![1, 2, 3]);
        stream.save(&path).await.expect("Failed to save file");

        let contents = tokio::fs::read(path).await.expect("Failed to read file");
        assert_eq!(contents, vec![1, 2, 3]);
    }

    #[ignore = "This test requires an audio device, which most CI environments do not have"]
    #[tokio::test]
    async fn play_will_play_contents() {
        // Thank you https://github.com/mathiasbynens/small for contributing to the public domain
        let smallest_syntactically_valid_mp3: Vec<u8> = vec![
            255, 227, 24, 196, 0, 0, 0, 3, 72, 0, 0, 0, 0, 76, 65, 77, 69, 51, 46, 57, 56, 46, 50,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let stream = VecU8A::from(smallest_syntactically_valid_mp3);
        stream.play().expect("Failed to play file");
    }
}
