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