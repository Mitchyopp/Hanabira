mod bot;
mod commands;

use serenity::all::GatewayIntents;
use serenity::Client;
use dotenv::dotenv;
use std::env;
use tracing::error;

use bot::Handler;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("No token in .env!");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client.");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
