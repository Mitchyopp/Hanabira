use serenity::all::ReactionType;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::model::channel::Message;
use serenity::prelude::*;

pub async fn run(ctx: &Context, msg: &Message) -> serenity::Result<()> {
    let embed = CreateEmbed::new()
        .title("🌸 Server Rules 🌸")
        .description("**Please read**")
        .field("#1 - Be respectful to each other.", "As the title says, We want this to be a friendly place so please be respectful of others!", false)
        .field("#2 - Do NOT ping the emergency role without a valid reason.", "This role (<@&1129107168478638132>) is used to inform admins of something important like scammers/raids to the server. Not for everyday things.", false)
        .field("#3 - Swearing is allowed but racial/hate speech is not.", "Using words like the N-word or racial slurs are not tolerated in this server. There is a strike system in place that will eventually lead to a ban.", false)
        .field("#4 - Moderation", "You may be punished for the following:\n\n>Mass pinging\n>Mass emojis\n>Hate speech/slurs\n>Linkspam\n>Self-promotion", false)
        .field("#5 - Follow discords TOS", "https://discord.com/terms", false)
        .field("#6 - No NFSW", "We don't accept any NSFW in any channel here please.", false)
        .field("#7 - Don't argue with Admins/Staff please.", "We don't have the patience or time to deal with you if your trying to start an argument. If you feel a staff member is abusing their powers feel free to ping <@431023897882656769>.\n The staff roles are:\n\n<@&1132016166999687168>\n<@&1151538257155338382>\n<@&1128022534441414696>\n<@&1156595385838600202>", false)
        .field("By reacting to this message you agree to the rules!", "", false)
        .color(0xe805a4);

    let builder = CreateMessage::new().embed(embed);
    msg.channel_id.send_message(&ctx.http, builder).await?;
    msg.react(&ctx.http, ReactionType::Unicode("🌸".to_string())).await?;
    Ok(())
}

