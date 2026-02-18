//! Full setup functionality for Discord server initialization.
//!
//! This module provides the complete setup process for a Discord server, combining both
//! partial setup (basic roles and channels) and complementary setup (additional categories
//! and specialized channels). It includes automatic rollback capabilities in case of failures.

use crate::database::server::Server;
use crate::discord::poise_structs::{Context, Error};
use crate::setup_command::complementary_setup::complementary_setup;
use crate::setup_command::partial_setup::partial_setup;

/// Performs a complete setup of the Discord server including roles, channels, and categories.
///
/// This function orchestrates the full server setup process by executing partial setup first
/// (creating basic roles and channels) followed by complementary setup (creating categories
/// and additional channels). If the complementary setup fails, all resources created during
/// partial setup are automatically rolled back to maintain consistency.
///
/// # Arguments
///
/// * `ctx` - The Discord command context providing access to the HTTP API and cache
/// * `server` - A mutable reference to the Server database entity that will be updated with
///              the IDs of created Discord resources
///
/// # Returns
///
/// * `Ok((&str, Vec<Role>, Vec<GuildChannel>))` - Success tuple containing:
///   - A translation key for the success message ("setup__setup_success_message")
///   - Vector of all created roles (from both partial and complementary setup)
///   - Vector of all created channels (from both partial and complementary setup)
///
/// * `Err(Vec<&str>)` - Vector of translation keys for error messages encountered during setup
///
/// # Error Handling
///
/// If partial_setup fails, the error is immediately returned.
/// If complementary_setup fails after partial_setup succeeds, all roles and channels created
/// during partial_setup are deleted before returning the error.
///
/// # Examples
///
/// ```no_run
/// use crate::setup_command::full_setup::full_setup;
///
/// async fn setup_server(ctx: Context<'_>, server: &mut Server) {
///     match full_setup(ctx, server).await {
///         Ok((msg_key, roles, channels)) => {
///             println!("Setup successful: {} roles, {} channels", roles.len(), channels.len());
///         }
///         Err(errors) => {
///             eprintln!("Setup failed with errors: {:?}", errors);
///         }
///     }
/// }
/// ```
pub async fn full_setup<'a>(ctx: &Context<'_>, server: &'a mut Server, snapshot: Server) -> Result<&'static str, Error> {
    partial_setup(ctx, server, snapshot).await?;
    complementary_setup(ctx, server, snapshot).await?;
    Ok("setup__full_setup_success")
}