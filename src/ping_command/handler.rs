//! Handler for the ping_command command.
use crate::ping_command::ping_data;
use crate::discord::poise_structs::*;

#[poise::command(slash_command)]
pub async fn ping(
    ctx: Context<'_>
) -> Result<(), Error> {
    let ping = ping_data::PingCommandData::new(ctx.created_at().timestamp_millis() as u128).ping;

    if let Err(why) = ctx.say(format!("Pong! {}ms", ping)).await {
        println!("Error sending message: {why:?}");
    }
    Ok(())
}