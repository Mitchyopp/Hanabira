use serenity::model::channel::Message;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, msg: &Message) {
    if msg.content == "<@1376689833795256421>" {
        let _ = msg.channel_id.say(&ctx.http, "Hi").await;
    }
}
