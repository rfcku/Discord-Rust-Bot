
use crate::commands;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use std::env;
use serenity::client::Context;
use serenity::model::application::Command;

pub async fn register(ctx: Context, ready: Ready){
    println!("{} is connected!", ready.user.name);
        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let _commands = guild_id
            .set_commands(&ctx.http, vec![
                commands::ping::register(),
                commands::id::register(),
                commands::welcome::register(),
                commands::numberinput::register(),
                commands::attachmentinput::register(),
                commands::modal::register(),
            ])
            .await;
        let _guild_command =
            Command::create_global_command(&ctx.http, commands::wonderful_command::register())
                .await;}
