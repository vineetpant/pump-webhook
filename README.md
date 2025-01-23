# Pump.fun telegram bot

This is the webhook to receive notification from stream whenever some pump fun token graduates and migrated to Raydium, Please read the medium post to understand the whole flow:-

[Link to Medium Post](https://medium.com/@viny.pant88/telegram-bot-to-track-token-migration-from-pump-fun-2d788b115bb7)

## Telegram bot handle 

Please join following telegram bot handle to receive all token migration events 

`@pumpfun_migration_bot`

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Installation

1. Clone the repository.
2. Install Rust and Cargo if you haven't already.
3. Set environment variables and or create .env file to store them. (Please refer to .env_sample for variables)
3. Run `cargo build` to build the project.
4. Run `cargo run` to run the project.

## Usage

This Rust project implements webhook to listen for messages regarding token migration from pump.fun to Raydium, it calls APIs to get price and metadata of tokens then forwards the message to telegram bot.

## License

This project is licensed under the [Apache License 2.0](LICENSE).
