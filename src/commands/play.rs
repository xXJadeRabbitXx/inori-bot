use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, Args};
use serenity::framework::standard::macros::command;

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            msg.reply(&ctx.http, "Must provide a URL to a video or audio").await?;

            return Ok(());
        },
    };

    if !url.starts_with("http") {
        msg.reply(&ctx.http, "Must provide a valid URL").await?;

        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        println!("{}", url);

        let source = match songbird::ytdl(&url).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                msg.reply(&ctx.http, "Error sourcing ffmpeg").await?;

                return Ok(());
            },
        };

        handler.play_source(source);

        msg.reply(&ctx.http, "Playing song").await?;
    } else {
        msg.reply(&ctx.http, "Not in a voice channel to play in").await?;
    }

    Ok(())
}