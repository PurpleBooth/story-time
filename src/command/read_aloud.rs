use std::path::PathBuf;

use super::super::remote::chatgpt;
use super::super::remote::elevenlabs;
use crate::io::audio::Audio;
use crate::remote::chatgpt::Repository as ChatGPTRepository;
use crate::remote::elevenlabs::Repository as ElevenlabsRepository;
use miette::Result;

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

    pub async fn run(
        self,
        chatgpt_direction: String,
        chatgpt_prompt: String,
        elevenlabs_voice: String,
        output: Option<PathBuf>,
    ) -> Result<()> {
        let message: String = self
            .chatgpt_client
            .generate_text(chatgpt_direction, chatgpt_prompt)
            .await?;
        let audio = self
            .elevenlabs_client
            .text_to_speech(elevenlabs_voice, message)
            .await?;

        if let Some(ref path) = output {
            audio.save(path).await?;
        } else {
            audio.play()?;
        }

        Ok(())
    }
}
