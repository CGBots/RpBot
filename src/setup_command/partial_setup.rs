//! Partial setup functionality for Discord server configuration.
//!
//! This module provides functionality to perform a partial setup of a Discord server,
//! creating essential roles and channels needed for the bot to function properly.
//! It handles role creation, permission ordering, and category setup while maintaining
//! rollback capabilities in case of errors.

use log::{log, Level};
use serenity::all::{ChannelType, GuildChannel, Role, RoleId};
use crate::database::server::{Id, Server};
use crate::database::server::IdType::Category;
use crate::discord::channels::{create_channel, get_road_category_permission_set};
use crate::discord::poise_structs::Context;
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
pub async fn partial_setup<'a>(ctx : Context<'_>, server: &mut Server) -> Result<(&'a str, Vec<Role>, Vec<GuildChannel>), Vec<&'a str>> {

    //everyone role
    let everyone_role = ctx.guild_id().unwrap().everyone_role();

    let mut roles_created: Vec<Role> = vec![];
    let mut errors: Vec<&str> = vec![];

    // Role creation pattern: First check if role ID exists in database.
    // If it exists, verify it still exists on Discord by fetching it.
    // If database has no ID or Discord fetch fails, create a new role.
    // Track newly created roles in roles_created vector for potential rollback.
    // This ensures idempotency - we don't recreate resources that already exist.
    let admin_role = match server.admin_role_id.id{
        None => {
            match create_role(ctx, tr!(ctx, "admin_role_name"), *AdminRolePermissions).await {
                Ok(role) => {roles_created.push(role.clone()); Ok(role)}
                Err(e) => {errors.push("setup__admin_role_not_created"); Err(e)}
            }
        }
        Some(role_id) => {
            match ctx.http().get_guild_role(server.server_id.into(), role_id.into()).await{
                Ok(role) => {Ok(role)}
                Err(_) => {
                    match create_role(ctx, tr!(ctx, "admin_role_name"), *AdminRolePermissions).await {
                        Ok(role) => {roles_created.push(role.clone()); Ok(role)}
                        Err(e) => {errors.push("setup__admin_role_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    let moderator_role = match server.moderator_role_id.id{
        None => {
            match create_role(ctx, tr!(ctx, "moderator_role_name"), *ModeratorRolePermissions).await {
                Ok(role) => {roles_created.push(role.clone()); Ok(role)}
                Err(e) => {errors.push("setup__moderator_role_not_created"); Err(e)}
            }
        }
        Some(role_id) => {
            match ctx.http().get_guild_role(server.server_id.into(), role_id.into()).await{
                Ok(role) => {Ok(role)}
                Err(_) => {
                    match create_role(ctx, tr!(ctx, "moderator_role_name"), *ModeratorRolePermissions).await {
                        Ok(role) => {roles_created.push(role.clone()); Ok(role)}
                        Err(e) => {errors.push("setup__moderator_role_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    let spectator_role = match server.spectator_role_id.id{
        None => {
            match create_role(ctx, tr!(ctx, "spectator_role_name"), *SpectatorRolePermissions).await {
                Ok(role) => {roles_created.push(role.clone()); Ok(role)}
                Err(e) => {errors.push("setup__spectator_role_not_created"); Err(e)}
            }
        }
        Some(role_id) => {
            match ctx.http().get_guild_role(server.server_id.into(), role_id.into()).await{
                Ok(role) => {Ok(role)}
                Err(_) => {
                    match create_role(ctx, tr!(ctx, "spectator_role_name"), *SpectatorRolePermissions).await {
                        Ok(role) => {roles_created.push(role.clone()); Ok(role)}
                        Err(e) => {errors.push("setup__spectator_role_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    let player_role = match server.player_role_id.id{
        None => {
            match create_role(ctx, tr!(ctx, "player_role_name"), *PlayerRolePermissions).await {
                Ok(role) => {roles_created.push(role.clone()); Ok(role)}
                Err(e) => {errors.push("setup__player_role_not_created"); Err(e)}
            }
        }
        Some(role_id) => {
            match ctx.http().get_guild_role(server.server_id.into(), role_id.into()).await{
                Ok(role) => {Ok(role)}
                Err(_) => {
                    match create_role(ctx, tr!(ctx, "player_role_name"), *PlayerRolePermissions).await {
                        Ok(role) => {roles_created.push(role.clone()); Ok(role)}
                        Err(e) => {errors.push("setup__player_role_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    //verification que les rôles ont bien été créés
    if !errors.is_empty() {
        for role in roles_created{
            match role.clone().delete(ctx).await {
                Ok(_) => {}
                Err(_) => {
                    log!(Level::Error, "Error while setuping and rollbacking.\
                     universe_id: {}\
                     server_id: {}\
                     role_id: {}", server.universe_id, server.server_id, role.id);
                    return Err(vec!["setup__rollback_failed"])
                }
            }
        }
        return Err(errors)
    }

    //Unwrapping
    let admin_role = admin_role.unwrap();
    let moderator_role = moderator_role.unwrap();
    let spectator_role = spectator_role.unwrap();
    let player_role = player_role.unwrap();
    let everyone_role = everyone_role;

    //Reordering roles
    let guild_id = ctx.guild_id().unwrap();
    let bot_id = ctx.cache().current_user().id;
    let bot_member = guild_id
        .member(ctx.http(), bot_id)
        .await.unwrap();
    let bot_role = bot_member.roles.clone()[0];

    let roles_pos: Vec<(RoleId, Option<u64>)> = vec![(admin_role.id, Some(4)), (moderator_role.id, Some(3)), (spectator_role.id, Some(2)), (player_role.id, Some(1)), (bot_role, Some(5))];
    let res = edit_role_positions(ctx, ctx.guild_id().unwrap(), roles_pos).await;

    match res {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e);
            // Rollback on role reordering failure: delete all newly created roles.
            // Best-effort cleanup - log errors but don't propagate deletion failures.
            for mut role in roles_created {
                match role.delete(ctx).await {
                    Ok(_) => {}
                    Err(_) => {
                        log!(Level::Error, "Error during setup and rollback.\
                            universe_id: {}\
                            server_id: {}\
                            role_id: {}", server.universe_id, server.server_id, role.id);
                    }
                };
            }
            return Err(vec!["setup__reorder_went_wrong"])}
    }

    let permissions = get_road_category_permission_set(everyone_role, player_role.id, spectator_role.id, moderator_role.id);

    // Tricky Result wrapper inversion: Existing channels are wrapped in Ok(), new channels in Err().
    // This allows us to distinguish between "found existing channel" vs "need to create new channel".
    // When we find an existing channel, we wrap it in Ok(channel).
    // When we need to create a channel, we wrap the create_channel result in Err().
    // The outer type is Result<Channel, Result<Channel, Error>>, enabling tracking via road_created flag.
    let result_road_category = match server.road_category_id.id {
        None => { Err(create_channel(ctx, tr!(ctx, "road_channel_name"), ChannelType::Category, 0, permissions, None).await) }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.into()).await {
                Ok(channel) => { Ok(channel) }
                Err(_) => {
                    Err(create_channel(ctx, tr!(ctx, "road_channel_name"), ChannelType::Category, 0, permissions, None).await)}
            }
        }
    };

    // Track whether the road category was newly created (vs already existing).
    // This flag is critical for rollback: we only delete newly created resources on failure.
    // Existing resources that were found should never be deleted during rollback.
    let mut road_created = false;

    let road_category = match result_road_category {
        Ok(channel) => {channel.guild().unwrap()}
        Err(new_channel_result) => {
            match new_channel_result {
                Ok(channel) => {road_created = true; channel}
                Err(_) => {
                    for mut role in roles_created {
                        match role.delete(ctx).await {
                            Ok(_) => {}
                            Err(_) => {
                                log!(Level::Error, "Error during setup and rollback.\
                                 universe_id: {}\
                                 server_id: {}\
                                 role_id: {}", server.universe_id, server.server_id, role.id);
                            }
                        };
                    }
                    return Err(vec!["setup__road_category_not_created"]); }
            }
        }
    };

    server.admin_role_id = Id{ id: Some(admin_role.id.get()), id_type: Some(crate::database::server::IdType::Role) };
    server.moderator_role_id = Id{ id: Some(moderator_role.id.get()), id_type: Some(crate::database::server::IdType::Role) };
    server.spectator_role_id = Id{ id: Some(spectator_role.id.get()), id_type: Some(crate::database::server::IdType::Role) };
    server.player_role_id = Id{ id: Some(player_role.id.get()), id_type: Some(crate::database::server::IdType::Role) };
    server.everyone_role_id = Id{ id: Some(everyone_role.get()), id_type: Some(crate::database::server::IdType::Role) };
    server.road_category_id = Id{ id: Some(road_category.id.get()), id_type: Some(Category) };

    match server.update().await {
        Ok(_) => {}
        Err(_) => {
            // Final rollback point: If database update fails, cleanup all Discord resources created in this run.
            // Delete all newly created roles (tracked in roles_created vector).
            // Delete road category only if road_created flag is true (meaning we created it, not found existing).
            // This prevents orphaned Discord resources when database is out of sync.
            // Best-effort cleanup - we log individual deletion failures but don't fail the rollback.
            for mut role in roles_created {
                match role.delete(ctx).await {
                    Ok(_) => {}
                    Err(_) => {
                        log!(Level::Error, "Error during setup and rollback.\
                            universe_id: {}\
                            server_id: {}\
                            role_id: {}", server.universe_id, server.server_id, role.id);
                    }
                };
            }
            if road_created{
                match road_category.delete(ctx).await {
                    Ok(_) => {}
                    Err(_) => {
                        log!(Level::Error, "Error during setup and rollback.\
                            universe_id: {}\
                            server_id: {}\
                            category_id: {}", server.universe_id, server.server_id, road_category.id.get());
                    }
                };
            }

            return Err(vec!["setup__server_update_failed"])}
    };

    Ok(("setup__setup_success_message", roles_created, vec![road_category]))
}