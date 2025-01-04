# Project Name

A brief description of the project.

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

## Contributing

Contributions are welcome! Please follow the guidelines in [CONTRIBUTING.md](CONTRIBUTING.md).

## License

This project is licensed under the [Apache License 2.0](LICENSE).
