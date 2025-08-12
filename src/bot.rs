use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use crate::commands;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        commands::ping::run(&ctx, &msg).await;
        commands::sys::run(&ctx, &msg).await;
        commands::onping::run(&ctx, &msg).await;
        commands::rude::run(&ctx, &msg).await;
        commands::links::run(&ctx, &msg).await;
        if msg.content == "!rules" {
            let _ = commands::rules::run(&ctx, &msg).await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name);
    }
}
