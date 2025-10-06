//! Provides the `/create_universe` slash command for Discord guild administrators.
//!
//! This module defines the command logic to create a new [`Universe`] in the database,
//! associating it with the guild and user that invoked the command. It performs
//! validation checks before creating the universe, and returns localized responses
//! to the user.
//!
//! The command is designed to be used by administrators only and must be executed
//! within a guild context (not in DMs).
use std::time::{SystemTime, UNIX_EPOCH};
use poise::CreateReply;
use crate::create_universe_command::logic::check_universe_conditions_for_creation;
use crate::database::universe::{Universe, FREE_LIMIT_UNIVERSE};
use crate::database::db_client::DB_CLIENT;
use crate::discord::poise_structs::*;
use crate::translation::tr;

/// Creates a new universe and binds it to the current Discord guild.
///
/// This slash command allows an administrator to create a new [`Universe`]
/// associated with the guild where the command is executed.  
/// The command checks whether the user and guild meet the required
/// creation conditions via [`check_universe_conditions_for_creation`].
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
///    [`check_universe_conditions_for_creation`].
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
/// # TODO
///
/// - Add an optional deployment step to configure roles and other
///   Discord elements before inserting the universe into the database.
/// - Replace `.unwrap()` calls with proper error handling.
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
#[poise::command(slash_command, required_permissions= "ADMINISTRATOR", guild_only)]
pub async fn create_universe(
    ctx: Context<'_>,
    universe_name: String,
) -> Result<(), Error> {
    ctx.defer().await.unwrap();
    
    let mut universe = Universe{
        universe_id: Default::default(),
        server_ids: vec![ctx.guild_id().unwrap().get()],
        name: universe_name.clone(),
        creator_id: ctx.author().id.get(),
        global_time_modifier: 100,
        creation_timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
        default_locale: ctx.partial_guild().await.unwrap().preferred_locale,
    };
    
    let check_result = check_universe_conditions_for_creation(ctx.guild_id().unwrap(), ctx.author().id).await;

    match check_result {
        Ok(_) => {}
        Err(fluent_token) => {
            ctx.send(
                CreateReply::default()
                    .content(tr!(ctx, fluent_token))
                    .ephemeral(true)
            ).await.unwrap();
            return Ok(());
        }
    }

    //TODO dans un second temps
    // proposer un déploiement partiel ou complet
    // créer les roles et autres éléments avant d'insérer dans la base de données

    let db_client = DB_CLIENT.lock().unwrap().clone();
    universe.universe_id = Default::default();
    match universe.insert_universe().await{
        Ok(result) => {
            ctx.send(
                CreateReply::default()
                    .content(tr!(ctx, "universe_created", universe_name: universe.name))
            ).await.unwrap();

            Ok(())
        }
        Err(_) => {Ok(())}
    }
}