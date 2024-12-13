# Capysay

A Rust-based CLI tool for customizable Capybara ASCII art with colorful messages.

## Features

- **Custom Messages**: Specify any message.
- **Dead Mode**: Add `X` eyes with a flag.
- **Colorful Output**: Styled text with `colored` crate.
- **Identity Guard**: Capybara won't "meow."

## Installation

Requires [Rust](https://www.rust-lang.org/):

```bash
cargo install capysay
```

## Usage

```bash
# Default
capysay

# Custom message
capysay "Hello, world!"

# Dead mode
capysay --dead
```

### Examples

#### Default
```text
        Libenter homines id quod volunt credunt!
                       \
                        \
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣄⢘⣒⣀⣀⣀⣀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣽⣿⣛⠛⢛⣿⣿⡿⠟⠂⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣀⡀⠀⣤⣾⣿⣿⣿⣿⣿⣿⣿⣷⣿⡆⠀
⠀⠀⠀⠀⠀⠀⣀⣤⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠁⠀
⠀⠀⠀⢀⣴⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀
⠀⠀⣠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀
⠀⠀⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠜⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⢿⣿⣿⣿⣿⠿⠿⣿⣿⡿⢿⣿⣿⠈⣿⣿⣿⡏⣠⡴⠀⠀⠀⠀⠀⠀⠀
⠀⠀⣠⣿⣿⣿⡿⢁⣴⣶⣄⠀⠀⠉⠉⠉⠀⢻⣿⡿⢰⣿⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⢿⣿⠟⠋⠀⠈⠛⣿⣿⠀⠀⠀⠀⠀⠀⠸⣿⡇⢸⣿⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⢸⣿⠀⠀⠀⠀⠀⠘⠿⠆⠀⠀⠀⠀⠀⠀⣿⡇⠀⠿⠇⠀⠀⠀⠀⠀⠀⠀
█─▄▄▄─██▀▄─██▄─▄▄─█▄─█─▄█─▄▄▄▄██▀▄─██▄─█─▄█
█─███▀██─▀─███─▄▄▄██▄─▄██▄▄▄▄─██─▀─███▄─▄██
▀▄▄▄▄▄▀▄▄▀▄▄▀▄▄▄▀▀▀▀▄▄▄▀▀▄▄▄▄▄▀▄▄▀▄▄▀▀▄▄▄▀▀
```

## Libraries Used

- **[clap](https://docs.rs/clap/)**: CLI argument parsing.
- **[colored](https://docs.rs/colored/)**: Styled output.

## ASCII Art Source

ASCII art from [Emoji Combos](https://emojicombos.com/capybara-ascii-art).

## License

MIT License. See `LICENSE` for details.