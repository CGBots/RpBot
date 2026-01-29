//! Logic for validating the preconditions required to create a new [`Universe`].
//!
//! This module ensures that a user or guild does not exceed the limits defined
//! for universe creation, such as the maximum number of universes per user or
//! servers per universe. It is used by the `/create_universe` command to prevent
//! invalid or duplicate universe creation requests.

use serenity::all::{GuildId, UserId};
use crate::database::universe::{Universe, FREE_LIMIT_SERVERS_PER_UNIERSE, FREE_LIMIT_UNIVERSE};

/// Validates whether a user and guild can create a new universe.
///
/// This function performs a series of checks to ensure that:
/// - The user has not already reached the maximum number of universes allowed.
/// - The guild is not already associated with one of the user’s existing universes.
/// - None of the user’s universes has exceeded the maximum number of servers allowed.
///
/// These limits are defined by the constants:
/// - [`FREE_LIMIT_UNIVERSE`]: maximum universes per creator.
/// - [`FREE_LIMIT_SERVERS_PER_UNIERSE`]: maximum servers per universe.
///
/// # Arguments
///
/// * `guild_id` - The Discord guild (server) attempting to create a new universe.
/// * `creator_id` - The user ID of the creator requesting universe creation.
///
/// # Returns
///
/// * `Ok(())` - If all conditions for creation are satisfied.
/// * `Err(&'static str)` - If any creation condition fails.  
///   The returned string corresponds to a *localization key* used for translation,
///   e.g., `"exceed_limit_number_of_universes"`.
///
/// # Errors
///
/// Possible error keys returned by this function:
///
/// * `"exceed_limit_number_of_universes"` – The user has reached the universe limit.  
/// * `"already_exist_for_this_server"` – The guild is already bound to one of the user’s universes.  
/// * `"exceed_limit_number_of_servers_per_universe"` – A universe owned by the user already has the maximum number of servers.
///
/// # Example
///
/// ```ignore
/// let guild_id = GuildId::new(123456789012345678);
/// let creator_id = UserId::new(987654321098765432);
///
/// match check_universe_conditions_for_creation(guild_id, creator_id).await {
///     Ok(_) => println!("All checks passed, universe can be created."),
///     Err(key) => println!("Creation denied. Reason: {}", key),
/// }
/// ```
pub async fn check_universe_conditions_for_creation(guild_id: GuildId, creator_id: UserId) -> Result<(), &'static str> {
    let universes = Universe::get_universe_creator(creator_id.get()).await;
    if universes.len() >= FREE_LIMIT_UNIVERSE {
        return Err("exceed_limit_number_of_universes")
    }

    for universe in universes {
        if universe.server_ids.contains(&guild_id.get()){
            return Err("already_exist_for_this_server")
        }
        
        if universe.server_ids.len() >= FREE_LIMIT_SERVERS_PER_UNIERSE {
            return Err("exceed_limit_number_of_servers_per_universe")
        }
    }
    Ok(())
}