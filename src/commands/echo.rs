use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;

#[command]
async fn echo(ctx: &Context, msg: &Message) -> CommandResult {
    let payload = msg.content.as_str().strip_prefix("!echo");

    msg.reply(&ctx.http, payload.unwrap_or_default()).await?;

    Ok(())
}