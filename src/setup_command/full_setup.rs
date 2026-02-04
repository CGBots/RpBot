//! Full setup functionality for Discord server initialization.
//!
//! This module provides the complete setup process for a Discord server, combining both
//! partial setup (basic roles and channels) and complementary setup (additional categories
//! and specialized channels). It includes automatic rollback capabilities in case of failures.

use crate::database::server::Server;
use crate::discord::poise_structs::Context;
use crate::setup_command::complementary_setup::complementary_setup;
use crate::setup_command::partial_setup::partial_setup;
use serenity::all::{GuildChannel, Role};
use futures::future::BoxFuture;
use poise::futures_util::future::join_all;

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
pub async fn full_setup<'a>(ctx: Context<'_>, server: &'a mut Server) -> Result<(&'a str, Vec<Role>, Vec<GuildChannel>), Vec<&'a str>> {
    let partial_setup_result = partial_setup(ctx, server).await;

    if partial_setup_result.is_err() {
        return Err(partial_setup_result.unwrap_err());
    }

    let complementary_setup_result = complementary_setup(ctx, server).await;

    if complementary_setup_result.is_err() {
        // Rollback mechanism: If complementary setup fails, we need to clean up all resources
        // created during partial setup to avoid leaving the server in an inconsistent state
        let (_, roles, channels) = partial_setup_result.unwrap();

        // Create async delete tasks for all roles created during partial setup
        let role_futures = roles.into_iter().map(|mut role| {
            Box::pin(async move {
                role.delete(ctx).await;
            }) as BoxFuture<'_, ()>
        });

        // Create async delete tasks for all channels created during partial setup
        let channel_futures = channels.into_iter().map(|channel| {
            Box::pin(async move {
                channel.delete(ctx).await;
            }) as BoxFuture<'_, ()>
        });

        // Chain all futures together (roles + channels) for concurrent execution
        let all_futures = role_futures.chain(channel_futures);

        // Execute all delete operations concurrently and wait for completion
        join_all(all_futures).await;

        return Err(complementary_setup_result.unwrap_err());
    }

    // Aggregate results from both setup phases into combined vectors
    // Clone is necessary here because we need to extract values from Result types
    // while maintaining ownership for the return value
    let mut roles = partial_setup_result.clone()?.1;
    roles.extend(complementary_setup_result.clone().unwrap().1);
    let mut channels = partial_setup_result.clone()?.2;
    channels.extend(complementary_setup_result.clone().unwrap().2);

    Ok(("setup__setup_success_message", roles, channels))
}