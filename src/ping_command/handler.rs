//! Handler for the `/ping` slash command.
//!
//! This command is used to check the bot’s responsiveness and latency.
//! It replies with a “Pong!” message and the measured latency in milliseconds.
//!
//! # Example
//! ```text
//! /ping
//! → Pong! 123ms
//! ```
//!
//! # Module Overview
//! - Defines the [`ping`] command handler.
//! - Uses [`PingCommandData`](crate::ping_command::ping_data::PingCommandData)
//!   to compute the latency since command invocation.
use crate::ping_command::ping_data;
use crate::discord::poise_structs::*;

/// Responds with the bot's latency to confirm it is online and responsive.
///
/// This slash command measures the elapsed time between the interaction creation
/// and the bot’s response, providing a simple latency check.
///
/// # Arguments
/// * `ctx` - The command context provided by the Poise framework.
///
/// # Example
/// ```ignore
/// /ping
/// → Pong! 87ms
/// ```
///
/// # Errors
/// Returns an error if the bot fails to send a reply message to Discord.
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