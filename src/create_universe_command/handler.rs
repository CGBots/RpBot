//! Provides the `/create_universe` slash command for Discord guild administrators.
//!
//! This module defines the command logic to create a new [`Universe`] in the database,
//! associating it with the guild and user that invoked the command. It performs
//! validation checks before creating the universe, and returns localized responses
//! to the user.
//!
//! The command is designed to be used by administrators only and must be executed
//! within a guild context (not in DMs).

use crate::add_server_to_universe_command::handler::add_server;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::database::universe::{Universe};
use crate::discord::poise_structs::*;
use crate::database::server::{Server};
use crate::database::stats::{Stat, SPEED_STAT};
use crate::database::stats::StatValue::Int;
use crate::setup_command::handler::{setup, SetupType, _setup};
use crate::utility::reply::reply;

/// Creates a new universe and binds it to the current Discord guild.
///
/// This slash command allows an administrator to create a new [`Universe`]
/// associated with the guild where the command is executed.  
/// The command checks whether the user and guild meet the required
/// creation conditions via [`check_universe_limit`].
///
/// If all checks pass, a new [`Universe`] instance is built, populated with
/// guild and user data, and inserted into the database.
///
/// # Command Behavior
///
/// 1. The command defers its response (acknowledges the interaction).
/// 2. It constructs a preliminary [`Universe`] object, initializing
///    core attributes such as the creator ID, guild ID, and creation timestamp.
/// 3. It validates whether the universe can be created using
///    [`check_universe_limit`].
/// 4. If validation fails, a localized error message is sent to the user.
/// 5. Otherwise, the universe is saved to the database, and a localized
///    `"universe_created"` confirmation message is returned.
///
/// # Arguments
///
/// * `ctx` - The Poise command context, giving access to Discord interaction data,
///   localization, and database resources.
/// * `universe_name` - The desired name for the new universe.
///
/// # Returns
///
/// Returns `Ok(())` when the command executes successfully,
/// or an [`Error`] if an unexpected failure occurs during message sending
/// or database insertion.
///
/// # Errors
///
/// * If `check_universe_conditions_for_creation` fails, the command
///   responds with an error message and exits gracefully.  
/// * Internal `.unwrap()` calls may cause a panic if:
///   - The interaction defer or send operation fails.
///   - The guild information is unavailable.
///   - Database locking or insertion fails.
///
/// # Permissions
///
/// This command requires the `ADMINISTRATOR` permission and is only
/// executable within guilds.
///
/// # Example
///
/// ```ignore
/// /create_universe MyFirstUniverse
/// ```
///
/// Responds with a confirmation message:
///
/// > ✅ Universe **MyFirstUniverse** has been successfully created!

#[poise::command(slash_command, subcommands("create_universe", "add_server", "setup"), subcommand_required)]
pub async fn universe(ctx: Context<'_>) -> Result<(), Error>{
    Ok(())
}

#[poise::command(slash_command, required_permissions= "ADMINISTRATOR", guild_only)]
pub async fn create_universe(
    ctx: Context<'_>,
    universe_name: String,
    setup_type: SetupType
) -> Result<(), Error> {
    ctx.defer().await?;
    let result = _create_universe(&ctx, universe_name, setup_type).await;
    reply(ctx.clone(), result).await?;
    Ok(())
}


pub async fn _create_universe(
    ctx: &Context<'_>,
    universe_name: String,
    setup_type: SetupType
) -> Result<&'static str, Error> {
    let Ok(result) = Universe::check_universe_limit(ctx.author().id.into()).await
        else {return Err("create_universe__check_universe_limit_failed".into())};

    if !result { return Err("create_universe__universe_limit_reached".into()); }

    let Ok(server) = Server::get_server_by_id(ctx.guild_id().unwrap().get().to_string()).await
        else {return Err("create_universe__get_server_failed".into())};

    if server.is_some(){ return Err("create_universe__already_exist_for_this_server".into()) }

    let universe = Universe {
        universe_id: Default::default(),
        name: universe_name.clone(),
        creator_id: ctx.author().id.get(),
        global_time_modifier: 100,
        creation_timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    };

    match universe.insert_universe().await{
        Ok(_) => {
            if universe.setup_constraints().await.is_err() { return Err("create_universe__setup_constraints_failed".into()); }

            let server = Server::default()
                .universe_id(universe.universe_id)
                .server_id(ctx.guild_id().unwrap().get()).clone();

            if server.insert_server().await.is_err(){ return Err("create_universe__server_insert_failed".into())}
        }
        Err(_) => { return Err("create_universe__universe_insert_failed".into()) }
    };

    let speed_stat = Stat{
        _id: Default::default(),
        universe_id: universe.universe_id,
        name: SPEED_STAT.to_string(),
        base_value: Int(3),
        formula: None,
        min: Some(Int(0)),
        max: Some(Int(999)),
        modifiers: vec![],
    };

    if let Err(_) = speed_stat.insert_stat().await{
        universe.delete().await?;
        return Err("create_universe__speed_stat_insert_failed".into());
    }

    _setup(ctx, setup_type).await?;

    Ok("create_universe__universe_successfully_created")
}