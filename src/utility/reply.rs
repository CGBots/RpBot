use log::log;
use poise::{CreateReply};
use serenity::all::{Color, CreateEmbed, CreateEmbedFooter};
use crate::discord::poise_structs::{Context, Error};

pub async fn reply<'a>(
    ctx: Context<'a>,
    result: Result<&'a str, Error>,
) -> Result<&'a str, Error> {
    let (color, string) = match result {
        Ok(string) => (Color::from_rgb(0, 255, 0), string.to_string()),
        Err(error) => (Color::from_rgb(255, 0, 0), error.to_string()),
    };

    match ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title(crate::translation::get(ctx, &string, Some("title"), None))
                .description(crate::translation::get(ctx, &string, Some("message"), None))
                .footer(
                    CreateEmbedFooter::new(string.clone())
                )
                .color(color),
        ),
    )
        .await {
        Ok(_) => {Ok("reply__reply_success")}
        Err(_) => {
            log!(log::Level::Error, "failed to reply:\nserver: {:?}\nerror_string: {}", ctx.guild_id(), string);
            Err("reply__reply_failed".into())}
    }
}