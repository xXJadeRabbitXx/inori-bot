mod commands;

//used to store environment variables, in this case the discord auth token
use std::env;
use songbird::SerenityInit;

use serenity::{
    async_trait,
    model::gateway::Ready,
    client::{
        Client,
        Context,
        EventHandler
    },
    framework::standard::{
        StandardFramework,
        macros::group
    }
};

use commands::{
    clear::*,
    echo::*,
    ping::*,
    summon::*,
    dismiss::*,
    play::*
};

#[group]
#[commands(ping, echo, summon, dismiss, play)]
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
        .register_songbird()
        .await
        .expect("Err creating client");

    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {:?}", why);
    }
}