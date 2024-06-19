use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;

pub fn register() -> CreateCommand {
    CreateCommand::new("welcome")
        .description("Welcome a user")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "The user to welcome")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "message", "The message to send")
                .required(true)
                .add_string_choice(
                    "Welcome to our cool server! Ask me if you need help",
                    "pizza",
                )
                .add_string_choice("Hey, do you want a coffee?", "coffee")
                .add_string_choice(
                    "Welcome to the club, you're now a good person. Well, I hope.",
                    "club"
                )
                .add_string_choice(
                    "I hope that you brought a controller to play together!",
                    "game"
                ),
        )
}

pub fn run(options: &[serenity::model::application::ResolvedOption]) -> String {

    println!("THIOS OPTIONS -0>>>>> {:?}", options);

    "Welcome to the server!".to_string()
}
