use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;

#[command]
#[only_in(guilds)]
async fn summon(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();
    let guild_id = msg.guild_id.unwrap();

    let voice_channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let voice_channel = match voice_channel_id {
        Some(channel) => {
            msg.reply(&ctx, "Connecting to voice channel").await?;
            channel
        },
        None => {
            msg.reply(&ctx, "You're not in a voice channel!").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.join(guild_id, voice_channel).await;

    Ok(())
}