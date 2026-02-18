//! Partial setup functionality for Discord server configuration.
//!
//! This module provides functionality to perform a partial setup of a Discord server,
//! creating essential roles and channels needed for the bot to function properly.
//! It handles role creation, permission ordering, and category setup while maintaining
//! rollback capabilities in case of errors.

use serenity::all::{ChannelType, Role, RoleId};
use crate::database::server::{Id, IdType, Server};
use crate::database::server::IdType::Category;
use crate::discord::channels::{create_channel, get_road_category_permission_set};
use crate::discord::poise_structs::{Context, Error};
use crate::discord::roles::{create_role, edit_role_positions, AdminRolePermissions, ModeratorRolePermissions, PlayerRolePermissions, SpectatorRolePermissions};
use crate::tr;

/// Performs a partial setup of the Discord server by creating essential roles and channels.
///
/// This function creates or verifies the existence of four key roles (Admin, Moderator, Spectator, Player)
/// and a roads category channel. It also properly orders the roles in the guild hierarchy.
/// If any step fails, the function attempts to rollback all changes made during the setup process.
///
/// # Arguments
///
/// * `ctx` - The Discord context containing guild and HTTP information
/// * `server` - A mutable reference to the server configuration that will be updated with new role and channel IDs
///
/// # Returns
///
/// * `Ok((message_key, created_roles, created_channels))` - On success, returns:
///   - A translation key for the success message
///   - A vector of newly created roles
///   - A vector of newly created channels (currently only the roads category)
/// * `Err(error_keys)` - On failure, returns a vector of translation keys describing the errors that occurred
///
/// # Behavior
///
/// 1. Checks if roles already exist in the database and on Discord, creates them if needed
/// 2. Reorders roles in the guild hierarchy: Bot > Admin > Moderator > Spectator > Player
/// 3. Creates or verifies the roads category channel with appropriate permissions
/// 4. Updates the server database record with new role and channel IDs
/// 5. Attempts to rollback (delete) all created resources if any step fails
///
/// # Errors
///
/// Returns error translation keys for various failure scenarios:
/// - `setup__admin_role_not_created` - Failed to create admin role
/// - `setup__moderator_role_not_created` - Failed to create moderator role
/// - `setup__spectator_role_not_created` - Failed to create spectator role
/// - `setup__player_role_not_created` - Failed to create player role
/// - `setup__rollback_failed` - Failed to rollback changes after an error
/// - `setup__reorder_went_wrong` - Failed to reorder roles in hierarchy
/// - `setup__road_category_not_created` - Failed to create roads category
/// - `setup__server_update_failed` - Failed to save server configuration to database
///
/// # Example
///
/// ```no_run
/// # use crate::setup_command::partial_setup::partial_setup;
/// # async fn example(ctx: Context<'_>, mut server: Server) {
/// match partial_setup(ctx, &mut server).await {
///     Ok((msg, roles, channels)) => {
///         println!("Setup successful: {}", msg);
///         println!("Created {} roles and {} channels", roles.len(), channels.len());
///     }
///     Err(errors) => {
///         eprintln!("Setup failed with errors: {:?}", errors);
///     }
/// }
/// # }
/// ```
pub async fn partial_setup<'a>(ctx: &Context<'_>, server: &mut Server, snapshot: Server) -> Result<&'a str, Error> {
    //everyone role
    let guild_id = ctx.guild_id().ok_or("guild_only")?;
    let everyone_role = guild_id.everyone_role();
    
    let Ok(existing_roles) = ctx.http().get_guild_roles(ctx.guild_id().unwrap()).await else {return Err("partial_setup__get_guild_roles_error".into())};

    let mut roles_created: Vec<Role> = vec![];
    let mut errors: Vec<&str> = vec![];

    // Role creation pattern: First check if role ID exists in database.
    // If it exists, verify it still exists on Discord by fetching it.
    // If database has no ID or Discord fetch fails, create a new role.
    // Track newly created roles in roles_created vector for potential rollback.
    // This ensures idempotency - we don't recreate resources that already exist.

    let admin_role = async {
        if let Some(role_id) = server.clone().admin_role_id {
            if let Ok(role) = guild_id.role(ctx, role_id.id.into()).await {
                return Ok(role);
            }
        }

        match create_role(ctx, tr!(*ctx, "admin_role_name"), *AdminRolePermissions).await {
            Ok(role) => {
                server.admin_role_id((role.id.get(), IdType::Role));
                roles_created.push(role.clone());
                Ok(role)
            }
            Err(e) => {
                errors.push("setup__admin_role_not_created");
                Err(e)
            }
        }
    }.await;


    let moderator_role = async {
        if let Some(role_id) = server.clone().moderator_role_id {
            if let Ok(role) = guild_id.role(ctx, role_id.id.into()).await {
                return Ok(role);
            }
        }

        match create_role(ctx, tr!(*ctx, "moderator_role_name"), *ModeratorRolePermissions).await {
            Ok(role) => {
                server.moderator_role_id((role.id.get(), IdType::Role));
                roles_created.push(role.clone());
                Ok(role)
            }
            Err(e) => {
                errors.push("setup__moderator_role_not_created");
                Err(e)
            }
        }
    }.await;


    let spectator_role = async {
        if let Some(role_id) = server.clone().spectator_role_id {
            if let Ok(role) = guild_id.role(ctx, role_id.id.into()).await {
                return Ok(role);
            }
        }

        match create_role(ctx, tr!(*ctx, "spectator_role_name"), *SpectatorRolePermissions).await {
            Ok(role) => {
                server.spectator_role_id((role.id.get(), IdType::Role));
                roles_created.push(role.clone());
                Ok(role)
            }
            Err(e) => {
                errors.push("setup__spectator_role_not_created");
                Err(e)
            }
        }
    }.await;


    let player_role = async {
        if let Some(role_id) = server.clone().player_role_id {
            if let Ok(role) = guild_id.role(ctx, role_id.id.into()).await {
                return Ok(role);
            }
        }

        match create_role(ctx, tr!(*ctx, "player_role_name"), *PlayerRolePermissions).await {
            Ok(role) => {
                server.player_role_id((role.id.get(), IdType::Role));
                roles_created.push(role.clone());
                Ok(role)
            }
            Err(e) => {
                errors.push("setup__player_role_not_created");
                Err(e)
            }
        }
    }.await;


    //verification que les rôles ont bien été créés
    if !errors.is_empty() {
        server.rollback(ctx, snapshot).await;
        return Err("setup__error_during_role_creation".into())
    }

    //Unwrapping
    let admin_role = admin_role?;
    let moderator_role = moderator_role?;
    let spectator_role = spectator_role?;
    let player_role = player_role?;
    let everyone_role = everyone_role;

    //Reordering roles
    let guild_id = ctx.guild_id().unwrap();
    let bot_id = ctx.cache().current_user().id;
    let bot_member = guild_id
        .member(ctx.http(), bot_id)
        .await.unwrap();
    let bot_role = bot_member.roles.clone()[0];

    let mut roles_pos: Vec<(RoleId, Option<u64>)> = vec![(admin_role.id, Some(4)), (moderator_role.id, Some(3)), (spectator_role.id, Some(2)), (player_role.id, Some(1)), (bot_role, Some(5))];
    
    // Add existing roles with higher positions to roles_pos without changing their positions
    for role in &existing_roles {
        if role.id != everyone_role {
            roles_pos.push((role.id, Some((role.position + existing_roles.len() as u16).into())));
        }
    }
    
    let res = edit_role_positions(ctx, ctx.guild_id().unwrap(), roles_pos).await;

    match res {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e);
            // Rollback on role reordering failure: delete all newly created roles.
            // Best-effort cleanup - log errors but don't propagate deletion failures.
            server.rollback(ctx, snapshot).await;
            return Err("setup__reorder_went_wrong".into())}
    }

    let permissions = get_road_category_permission_set(everyone_role, player_role.id, spectator_role.id, moderator_role.id);

    // Tricky Result wrapper inversion: Existing channels are wrapped in Ok(), new channels in Err().
    // This allows us to distinguish between "found existing channel" vs "need to create new channel".
    // When we find an existing channel, we wrap it in Ok(channel).
    // When we need to create a channel, we wrap the create_channel result in Err().
    // The outer type is Result<Channel, Result<Channel, Error>>, enabling tracking via road_created flag.
    let result_road_category = match server.clone().road_category_id {
        None => { Err(create_channel(ctx, tr!(*ctx, "road_channel_name"), ChannelType::Category, 0, permissions, None).await) }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.id.into()).await {
                Ok(channel) => { Ok(channel) }
                Err(_) => {
                    Err(create_channel(ctx, tr!(*ctx, "road_channel_name"), ChannelType::Category, 0, permissions, None).await)}
            }
        }
    };

    // Track whether the road category was newly created (vs already existing).
    // This flag is critical for rollback: we only delete newly created resources on failure.
    // Existing resources that were found should never be deleted during rollback.

    let road_category = match result_road_category {
        Ok(channel) => {channel.guild().unwrap()}
        Err(new_channel_result) => {
            match new_channel_result {
                Ok(channel) => {channel}
                Err(_) => {
                    server.rollback(ctx, snapshot).await;
                    return Err("setup__road_category_not_created".into()); }
            }
        }
    };
    
    server.admin_role_id(Id{ id: admin_role.id.get(), id_type: IdType::Role })
        .moderator_role_id(Id{ id: moderator_role.id.get(), id_type: IdType::Role })
        .spectator_role_id(Id{ id: spectator_role.id.get(), id_type: IdType::Role })
        .player_role_id(Id{ id: player_role.id.get(), id_type: IdType::Role })
        .everyone_role_id(Id{ id: everyone_role.get(), id_type: IdType::Role })
        .road_category_id(Id{ id: road_category.id.get(), id_type: Category });
    
    let update_result = server.update().await;

    if update_result.is_err() {
        // Final rollback point: If database update fails, cleanup all Discord resources created in this run.
        // Delete all newly created roles (tracked in roles_created vector).
        // Delete road category only if road_created flag is true (meaning we created it, not found existing).
        // This prevents orphaned Discord resources when database is out of sync.
        // Best-effort cleanup - we log individual deletion failures but don't fail the rollback.
        server.rollback(ctx, snapshot).await;
        return Err("setup__server_update_failed".into());
    };

    Ok("setup__setup_success_message")
}
