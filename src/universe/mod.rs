pub mod create_universe_sub_command;
pub mod add_server_sub_command;
pub mod setup;

use crate::universe::setup::setup_sub_command::setup;
use crate::universe::add_server_sub_command::add_server;
use crate::discord::poise_structs::{Context, Error};
use crate::universe::create_universe_sub_command::create_universe;

/// Handles the `/universe` slash command with multiple subcommands.
///
/// This command acts as a namespace for related subcommands that perform
/// various operations. By design, a subcommand is required when invoking this
/// command.
///
/// ### Subcommands:
/// - **create_universe**: Command to create a new universe.
/// - **add_server**: Command to add a server to an existing universe.
/// - **setup**: Command to configure or set up the universe.
///
/// ### Parameters:
/// - `ctx`: The command context, which provides access to Discord interaction data
///   and utilities for responding to the user.
///
/// ### Returns:
/// - Ok(()) if the command executes successfully.
///
/// ### Errors:
/// - This function itself does not produce any errors, but the subcommands it delegates
///   to may return errors as appropriate.
///
/// ### Notes:
/// - This command requires specifying one of the listed subcommands as it does not
///   have a default action.
#[poise::command(slash_command, subcommands("create_universe", "add_server", "setup"), subcommand_required)]
pub async fn universe(ctx: Context<'_>) -> Result<(), Error>{
    Ok(())
}