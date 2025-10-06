//! Handler for the `/start` slash command.
//!
//! This command sends a welcome or startup message to the user who
//! invoked it. Typically used to introduce the bot or provide initial
//! instructions.
//!
//! # Example
//! ```text
//! /start
//! → Welcome! Here’s how to use the bot...
//! ```

use poise::CreateReply;
use crate::discord::poise_structs::{Context, Error};
use crate::translation::tr;

/// Sends a startup message to the user who invoked the `/start` command.
///
/// # Arguments
/// * `ctx` - The Poise command context, providing access to the Discord interaction
///   and localization.
///
/// # Permissions
/// Requires the `ADMINISTRATOR` permission and must be executed in a guild.
///
/// # Errors
/// Returns an [`Error`] if the bot fails to send the reply message.
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR", guild_only)]
pub async fn start(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(CreateReply::default().content(tr!(ctx, "start_message"))).await.unwrap();
    Ok(())
}
