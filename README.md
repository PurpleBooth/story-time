# Story time

Read a story out loud

    Tools for generating audio stories

    Usage: story-time [OPTIONS] <COMMAND>

    Commands:
      read-aloud  Read a prompt from ChatGPT aloud
      help        Print this message or the help of the given subcommand(s)

    Options:
      -r, --rust-log <RUST_LOG>
              Log level
              
              Can be trace, debug, info, warn, error, or off. You can also put a module name after a comma to set a specific log level for that module "error,hello=warn" turn on global error logging and also warn for hello
              
              [env: RUST_LOG=]
              [default: info]

      -h, --help
              Print help (see a summary with '-h')

      -V, --version
              Print version

The `read-aloud` command

    Read a prompt from ChatGPT aloud

    Usage: story-time read-aloud [OPTIONS] --chatgpt-key <CHATGPT_KEY> --elevenlabs-key <ELEVENLABS_KEY> --chatgpt-prompt <CHATGPT_PROMPT>

    Options:
      -c, --chatgpt-key <CHATGPT_KEY>
              Key for ChatGPT [env: CHATGPT_KEY=]
      -e, --elevenlabs-key <ELEVENLABS_KEY>
              Key for ElevenLabs [env: ELEVENLABS_KEY=]
      -c, --chatgpt-prompt <CHATGPT_PROMPT>
              Prompt to give to ChatGPT [env: CHATGPT_PROMPT=]
      -c, --chatgpt-direction <CHATGPT_DIRECTION>
              A style to read in [env: CHATGPT_DIRECTION=] [default: "You are reading aloud"]
      -e, --elevenlabs-voice <ELEVENLABS_VOICE>
              ID of the voice to use [env: ELEVENLABS_VOICE=] [default: MF3mGyEYCl7XYWbV9V6O]
      -o, --output <OUTPUT>
              Save to a file rather than reading aloud [env: OUTPUT=]
      -h, --help
              Print help
      -V, --version
              Print version
