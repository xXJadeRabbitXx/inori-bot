use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;

#[command]
#[only_in(guilds)]
async fn dismiss(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            msg.reply(&ctx, format!("Failed: {:?}", e)).await?;
        }

        msg.reply(&ctx, "Left voice channel").await?;
    } else {
        msg.reply(&ctx, "Not in a voice channel").await?;
    }

    Ok(())
}