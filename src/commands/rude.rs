use serenity::model::channel::Message;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, msg: &Message) {
    if msg.content == "test" {
        let _ = msg.reply(&ctx.http, "Yeah").await;
    }

    if msg.content == "avatar" {
        let avatar = msg.author.avatar_url().unwrap();
        //let _ = msg.reply(&ctx.http, &format!("{avatar}")).await;
        let _ = msg.reply(&ctx.http, &avatar).await;
    }
}
