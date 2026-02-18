//! Logic functions for linking Discord guilds (servers) to universes.
//!
//! This module provides helper functions used by the `/add_server` command
//! to check if a guild is already associated with a universe and to link
//! it to a new one. These functions interact with the database layer defined
//! in [`crate::database::universe::Universe`].

use crate::database::universe::Universe;

/// Checks whether a Discord guild (server) is already bound to an existing universe.
///
/// This function queries the database to determine if the provided `guild_id`
/// is linked to a [`Universe`]. If the guild is found within a universe,
/// the corresponding [`Universe`] object is returned.
///
/// # Arguments
///
/// * `guild_id` - The unique Discord ID of the guild to check.
///
/// # Returns
///
/// * `Ok(Universe)` if the guild is already bound to a universe.
/// * `Err(String)` if no associated universe is found, or if the query fails.
///
/// # Errors
///
/// Returns an `Err(String)` with a human-readable message when:
/// - The guild is not bound to any universe.
/// - The database query fails or produces no results.
///
/// # Example
///
/// ```ignore
/// let result = check_server_in_universe(123456789012345678).await;
/// match result {
///     Ok(universe) => println!("Guild is linked to universe: {}", universe.name),
///     Err(err) => println!("Error: {}", err),
/// }
/// ```
pub async fn check_server_in_universe(guild_id: u64) -> Result<Universe, String>{
    if let Ok(cursor) = Universe::get_universe_by_server_id(guild_id).await {
        if let Some(universe) = cursor{
            return Ok(universe);
        }
    }
    Err(format!("Guild {} not bind to any existing universe", guild_id))
}
