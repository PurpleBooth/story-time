# Story time

Read a story out loud

```
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
```