
use crate::commands;
use serenity::model::application::Interaction;
use serenity::client::Context;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};

pub async fn handle(ctx: &Context, interaction: &Interaction) {
   if let Interaction::Command(command) = interaction {
        let content = match command.data.name.as_str() {
            "welcome" => Some(commands::welcome::run(&command.data.options())),
            "ping" => Some(commands::ping::run(&command.data.options())),
            "id" => Some(commands::id::run(&command.data.options())),
            "numberinput" => Some(commands::numberinput::run(&command.data.options())),
            "attachmentinput" => Some(commands::attachmentinput::run(&command.data.options())),
            "modal" => {
                commands::modal::run(&ctx, &command).await.unwrap();
                None
            },
            _ => Some("not implemented :(".to_string()),
        };

        println!("Command: {:?}", command.data.name);

        if let Some(content) = content {
            let data = CreateInteractionResponseMessage::new().content(content);
            let builder = CreateInteractionResponse::Message(data);
            if let Err(why) = command.create_response(&ctx.http, builder).await {
                println!("Cannot respond to slash command: {why}");
            }
        }
    }
}
