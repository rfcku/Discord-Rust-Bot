use serenity::model::channel::Message;
use serenity::client::Context;
use crate::services::openai::OpenAi;
use std::env;
use Option;

pub fn is_not_bot_message(msg: &Message) -> bool {
    let bot_id = env::var("BOT_USER_ID").expect("BOT_USER_ID must be set.");
    msg.author.id.to_string() != bot_id
}

pub fn is_bot_mention(msg: &Message) -> bool {
    let bot_id = env::var("BOT_USER_ID").expect("BOT_USER_ID must be set.");
    msg.content.contains(bot_id.as_str())
}

pub fn is_private(msg: &Message) -> bool {
    msg.is_private()
}

pub fn is_command(msg: &Message) -> bool {
    msg.content.starts_with("!")
}

pub fn is_ping(msg: &Message) -> bool {
    msg.content == "!ping"
}

pub fn should_respond(msg: &Message) -> bool {
    (is_private(msg) || is_bot_mention(msg)) && is_not_bot_message(msg)
}

pub async fn channel_history(ctx: &Context, msg: &Message) -> Vec<Message> {
    let messages = ctx.http.get_messages(
        msg.channel_id,
        Option::None,
        Option::Some(10),
    ).await;
    let messages = match messages {
        Ok(messages) => messages,  
        Err(e) => {
            println!("Error getting messages: {:?}", e);
            return vec![];
        }
    };
    messages
}

pub fn message_role(msg: &Message) -> String {
    let bot_id = env::var("BOT_USER_ID").expect("BOT_USER_ID must be set.");
    if msg.author.id.to_string() == bot_id {
        "assistant".to_string()
    } else {
        "user".to_string()
    }
}

pub fn to_history(messages: Vec<Message>) -> Vec<serde_json::Value> {
    let mut history = vec![];
    for message in messages.iter() {
        let role = message_role(message);
        history.push(
            serde_json::json!({
                "role": role,
                "content": message.content,
            })
        );
    }
    history
}

pub async fn handle(ctx: &Context, msg: &Message) -> () {

    if should_respond(msg) {
        msg.channel_id.broadcast_typing(&ctx.http).await.unwrap();
        if is_ping(msg) || is_command(msg) {
            let _ = msg.channel_id.say(&ctx.http, "Pong!").await;
            return;
        }

        let messages = channel_history(ctx, msg).await;
        let history = to_history(messages);

        let response: String = match OpenAi::new().create_completion( &msg.content, history ).await {
            Ok(response) => response,
            Err(e) => {
                println!("Error creating run: {:?}", e);
                return;
            }
        };
        
        msg.channel_id.say(&ctx.http, format!("{}", response)).await.unwrap();
        return;
    }
}
