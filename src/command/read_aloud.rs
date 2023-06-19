use std::{fmt::Debug, path::Path};

use miette::Result;
use tracing::instrument;

use super::super::remote::{chatgpt, elevenlabs};
use crate::{
    chatgpt::Direction,
    io::audio::Audio,
    remote::{
        chatgpt::{Prompt, Repository as ChatGPTRepository},
        elevenlabs::{Repository as ElevenlabsRepository, Voice},
    },
};

#[derive(Debug)]
pub struct Command {
    chatgpt_client: chatgpt::ChatGPT,
    elevenlabs_client: elevenlabs::Reqwest,
}

impl Command {
    pub const fn new(
        chatgpt_client: chatgpt::ChatGPT,
        elevenlabs_client: elevenlabs::Reqwest,
    ) -> Self {
        Self {
            chatgpt_client,
            elevenlabs_client,
        }
    }

    #[instrument]
    pub async fn run<
        D: Into<Direction> + Sync + Send + Debug,
        P: Into<Prompt> + Sync + Send + Debug,
        V: Into<Voice> + Sync + Send + Debug,
        O: AsRef<Path> + Sync + Send + Debug,
    >(
        self,
        chatgpt_direction: D,
        chatgpt_prompt: P,
        elevenlabs_voice: V,
        output: Option<O>,
    ) -> Result<()> {
        let message = self
            .chatgpt_client
            .generate_text(chatgpt_direction.into(), chatgpt_prompt.into())
            .await?;
        let audio = self
            .elevenlabs_client
            .text_to_speech(elevenlabs_voice.into(), message)
            .await?;

        if let Some(path) = output {
            audio.save(path.as_ref()).await?;
        } else {
            audio.play()?;
        }

        Ok(())
    }
}
