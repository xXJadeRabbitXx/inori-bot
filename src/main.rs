mod commands;

//used to store environment variables, in this case the discord auth token
use std::env;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::channel::Message;
use tokio::task;
use tokio::time::{Duration, sleep};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with('!') {
            let command = msg.content.as_str().split_whitespace().next().unwrap_or("");

            let result = match command{
                "!help" => commands::help(&ctx,&msg).await,
                "!echo" => commands::echo(&ctx, &msg).await,
                _ => commands::unknown_command(&ctx, &msg).await,
            };

            task::spawn(async move {
                sleep(Duration::from_millis(5000)).await;
                if let Err(why) = msg.delete(&ctx).await{
                    println!("Error sending message: {:?}", why);
                };

                match result{
                    Some(message) => {
                        if let Err(why) = message.delete(&ctx).await {
                            println!("Error sending message: {:?}", why);
                        };
                    },
                    None => ()
                };
            });
        };
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main(worker_threads = 2)]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    println!("{}", token);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {:?}", why);
    }
}