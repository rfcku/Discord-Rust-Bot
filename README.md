[![Rust](https://github.com/rfcku/rust-bot-starter-kit/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rfcku/rust-bot-starter-kit/actions/workflows/rust.yml)

# Rust Discord Bot Starter Kit

This Discord Bot is a starter kit built using Rust, designed to get you up and running with your own Discord bot that utilizes slash commands and integrates with OpenAI's powerful AI models. It's perfect for developers looking to explore Discord bot development with Rust's performance and safety features.

## Features

- **Slash Commands**: Easy to use slash commands for interaction within Discord.
- **OpenAI Integration**: Leverage OpenAI's API to integrate AI-powered responses and functionalities.
- **Customizable**: A clean codebase that's easy to extend with more commands and features.
- **Asynchronous**: Built with async/await for efficient performance.

## Getting Started

To get your Discord bot up and running, follow these steps:

### Prerequisites

- Rust and Cargo installed on your machine.
- A Discord Bot Token ([Discord Developers](https://discord.com/developers/docs/intro)).
- An OpenAI API Key ([OpenAI API](https://openai.com/api/)).

### Installation

1. **Clone the repository**

```
git clone https://github.com/rfcku/rust-bot-starter-kit/tree/main

cd rust-discord-bot-starter
```

2. **Configure Environment Variable**

Rename .env.example to .env and fill in your Discord Bot Token and OpenAI API key.

```bash
    DISCORD_TOKEN=your_discord_bot_token_here
    OPENAI_API_KEY=your_openai_api_key_here
```

3. **Build and run**

Compile and run the bot with Cargo.

```bash
cargo build
cargo run
```

## Adding Commands

Usage
After starting the bot, it will be online in your Discord server. You can interact with it using slash commands that you define.

Adding New Slash Commands
To add a new slash command:

Define the command in `src/commands.rs`.
Update the command handler in `src/main.rs` to include your new command.
Integrating with OpenAI
The starter kit includes a basic OpenAI integration example. To utilize 

## OpenAI's capabilities:

Use the provided OpenAI client in `src/services/openai.rs` to make requests.
Process the OpenAI API responses according to your application's needs.

```rust
    use serenity::framework::standard::{
        macros::{command, group},
        CommandResult,
    };
    use serenity::model::prelude::*;
    use serenity::prelude::*;

    #[command]
    async fn your_command(ctx: &Context, msg: &Message) -> CommandResult {
        msg.channel_id.say(&ctx.http, "Hello, World!").await?;
        Ok(())
    }
```

### Contributing

Contributions are welcome! Please feel free to submit pull requests, report bugs, or suggest features.
