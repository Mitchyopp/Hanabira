use serenity::model::channel::Message;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, msg: &Message) {
    if msg.content == "!links" {

let content = "\
# links

## Useful
https://mitchopp.dev // My website
https://outeruniverse.org // Dashboard
https://youtube.com // Youtube
https://keybr.com // Typing
https://monkeytype.com // Speed typing
https://archlinux.org // Arch wiki
https://tofugu.com // Japanese / 日本人

## Coding
https://github.com/Mitchyopp // Github

## Reading
https://manhuaplus.top
https://asuracomic.net
https://mangadex.com

## Goated apps
https://mihon.app // A great app for reading
https://dantotsu.org
https://github.com/recloudstream/cloudstream // A great app for watching shows/movies
https://github.com/LagradOst/QuickNovel // A great app for reading novels
https://revanced.app // Better youtube
";

        let _ = msg.channel_id.say(&ctx.http, content).await;
    }
}
