//used to store environment variables, in this case the discord auth token
use std::env;

use serenity::async_trait;
use serenity::model::gateway::Ready;

use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "pong!").await?;

    Ok(())
}

#[command]
async fn echo(ctx: &Context, msg: &Message) -> CommandResult {
    let payload = msg.content.as_str().strip_prefix("!echo");

    msg.reply(&ctx.http, payload.unwrap_or_default()).await?;

    Ok(())
}

/*
#[command]
async fn clear(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "deleting messages").await?;

    let mut messages = msg.channel_id.messages_iter(&ctx).boxed();
    while let Some(message_result) = messages.next().await {
        match message_result {
            Ok(message) => {
                if message.is_own(&ctx.cache).await {
                    message.delete(&ctx).await?;
                }
            },
            Err(error) => eprintln!("Uh oh! Error: {}", error),
        }
    }

    Ok(())
}
*/

#[group]
#[commands(ping, echo)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    println!("Token: {}", token);

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {:?}", why);
    }
}