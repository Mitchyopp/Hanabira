use serenity::model::channel::Message;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, msg: &Message) {
    let _ = reply_if(ctx, msg, "fuck", "No profanity here please").await;
    let _ = reply_if(ctx, msg, "Response", "Goated auto response").await;
}

pub async fn reply_if(ctx: &Context, msg: &Message, word: &str, response: &str) -> serenity::Result<()> {
    if msg.content.eq_ignore_ascii_case(word) {
        msg.channel_id.say(&ctx.http, response).await;
    }
    Ok(())
}
