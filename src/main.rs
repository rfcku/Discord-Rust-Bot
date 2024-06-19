mod services;
mod models;
mod messages;
mod commands;

use std::env;
use dotenv::dotenv;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::application::Interaction;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        commands::handler::handle(&ctx, &interaction).await;
    }
    
    async fn ready(&self, ctx: Context, ready: Ready) {
        commands::register::register(ctx, ready).await;        
    }
   
    async fn message(&self, ctx: Context, msg: Message) {
        messages::main::handle(&ctx, &msg).await;
    }

}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
