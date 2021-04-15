use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::group;

#[command]
pub(super) async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "A simple test bot").await?;
    Ok(())
}

#[command]
pub(super) async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "pong!").await?;
    Ok(())
}

const HELP_MESSAGE: &str = "
          Hello there, Human!

          You have summoned me. Let's see about getting you what you need.

          ? Need technical help?
          => Post in the <#CHANNEL_ID> channel and other humans will assist you.

          ? Looking for the Code of Conduct?
          => Here it is: <https://opensource.facebook.com/code-of-conduct>

          ? Something wrong?
          => You can flag an admin with @admin

          I hope that resolves your issue!
          -- Helpbot

          ";

pub(super) async fn help(ctx: &Context, message: &Message) -> Option<Message> {
    return match message.channel_id.say(&ctx.http, HELP_MESSAGE).await {
        Ok(reply) => Some(reply),
        Err(why) => {
            println!("Error sending message: {:?}", why);
            None
        },
    }
}

pub(super) async fn echo(ctx: &Context, message: &Message) -> Option<Message> {
    let payload = message.content.as_str().strip_prefix("!echo");

    return match message.channel_id.say(&ctx.http, payload.unwrap_or_default()).await {
        Ok(reply) => Some(reply),
        Err(why) => {
            println!("Error sending message: {:?}", why);
            None
        },
    }
}

pub(super) async fn summon(ctx: &Context, message: &Message) -> Option<Message> {
    //ctx.
    None
}

pub(super) async fn unknown_command(ctx: &Context, message: &Message) -> Option<Message> {
    return match message.channel_id.say(&ctx.http, "Unknown Command").await {
        Ok(reply) => Some(reply),
        Err(why) => {
            println!("Error sending message: {:?}", why);
            None
        },
    }
}
