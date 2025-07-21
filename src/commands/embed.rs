use serenity::builder::CreateEmbed;
use serenity::builder::CreateMessage
use serenity::model::channel::Message;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, msg: &Message) {
  if msg.content == "!embed" {
      let embed = CreateEmbed::new()
          .title("Learning embeds")
          .description("...")
      
      let builder = CreateMessage::new()
            .cont
  }
}
