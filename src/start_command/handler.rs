use poise::CreateReply;
use crate::discord::poise_structs::{Context, Error};
use crate::translation::tr;

#[poise::command(slash_command, required_permissions = "ADMINISTRATOR", guild_only)]
pub async fn start(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(CreateReply::default().content(tr!(ctx, "start_message"))).await.unwrap();
    Ok(())
}
