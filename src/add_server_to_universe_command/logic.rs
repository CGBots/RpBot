use crate::database::universe::Universe;

/// Asynchronously checks if a specific guild (server) is associated with a universe.
///
/// This function attempts to retrieve a `Universe` object that corresponds to the provided
/// `guild_id`. If the server is associated with a universe, the function returns the universe;
/// otherwise, an error message is returned indicating that no universe is bound to the server.
///
/// # Arguments
///
/// * `guild_id` - The unique identifier (`u64`) of the guild (server) to check.
///
/// # Returns
///
/// * `Ok(Universe)` - If the guild is successfully found in the universe.
/// * `Err(String)` - If no universe is associated with the given `guild_id`, includes an
///   error message detailing the guild's ID.
///
/// # Errors
///
/// This function will return an error string if:
/// - The guild is not bound to any existing universe.
/// - Retrieving the universe encounters a failure.
///
/// # Examples
///
/// ```
/// let guild_id = 123456789;
/// match check_server_in_universe(guild_id).await {
///     Ok(universe) => println!("Found universe: {:?}", universe),
///     Err(error) => println!("Error: {}", error),
/// }
/// ```
///
/// Note: This function relies on the `Universe::get_universe_by_server_id` method to fetch
/// universe details asynchronously.
pub async fn check_server_in_universe(guild_id: u64) -> Result<Universe, String>{
    if let Ok(cursor) = Universe::get_universe_by_server_id(guild_id).await {
        if let Some(universe) = cursor{
            return Ok(universe);
        }
    }
    Err(format!("Guild {} not bind to any existing universe", guild_id))
}
