use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::builder::EditMessage;
use std::time::Instant;

pub async fn run(ctx: &Context, msg: &Message) {
    if msg.content == "!ping" {
        let start = Instant::now();

        if let Ok(mut sent_msg) = msg.channel_id.say(&ctx.http, "Pinging...").await {
            let elapsed = start.elapsed().as_millis();

            let content = format!("Pong. `{}` MS.", elapsed);
            let _ = sent_msg.edit(&ctx.http, EditMessage::new().content(content)).await;
        }
    }
}
