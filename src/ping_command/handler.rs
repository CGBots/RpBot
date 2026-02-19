use crate::ping_command::ping_data;
use crate::discord::poise_structs::*;

/// Responds to a "ping" command with "Pong!" and the latency in milliseconds.
///
/// # Arguments
/// * `ctx` - The command context, which provides access to information about the command execution, such as the message and invoking user.
///
/// # Returns
/// * `Result<(), Error>` - Returns `Ok(())` if the response message is successfully sent. Returns an `Error` if there is an issue during execution.
///
/// # Behavior
/// * Calculates the latency between the command's creation time and the current time in milliseconds.
/// * Sends a message in the channel where the command was invoked, containing "Pong!" followed by the computed latency.
/// * Logs an error to the console if there is a failure in sending the message.
///
/// # Example
/// User sends a `/ping` command:
/// ```
/// User: /ping
/// Bot: Pong! 123ms
/// ```
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