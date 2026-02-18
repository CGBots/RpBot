//! Complementary setup module for Discord server configuration.
//!
//! This module handles the creation and configuration of Discord categories and channels
//! required for the bot's roleplay functionality. It manages the setup of administrative,
//! non-roleplay (NRP), and roleplay (RP) categories along with their associated channels.
//!
//! The setup process is idempotent and includes automatic rollback on failure to maintain
//! server consistency.

use log::{log, Level};
use serenity::all::{ChannelType, GuildChannel};
use crate::database::server::{Id, IdType, Server};
use crate::discord::channels::{create_channel, get_admin_category_permission_set, get_rp_character_permission_set};
use crate::discord::poise_structs::{Context, Error};
use crate::tr;

/// Performs complementary setup for a Discord server by creating required categories and channels.
///
/// This function creates or verifies the existence of the following Discord server structure:
///
/// # Categories Created
/// - **Admin Category**: Contains administrative channels with restricted permissions
/// - **NRP Category**: Contains non-roleplay channels for general discussion
/// - **RP Category**: Contains roleplay-specific channels
///
/// # Channels Created
/// - **Log Channel**: For bot logging (in Admin category)
/// - **Commands Channel**: For bot commands (in Admin category)
/// - **Moderation Channel**: For moderation activities (in Admin category)
/// - **NRP General Channel**: General discussion (in NRP category)
/// - **RP Character Channel**: Character sheets with special permissions (in RP category)
/// - **Wiki Channel**: Forum-type channel for RP documentation (in RP category)
///
/// # Arguments
///
/// * `ctx` - The Discord context containing HTTP client and guild information
/// * `server` - Mutable reference to the server database object that will be updated with created channel/category IDs
///
/// # Returns
///
/// * `Ok((&'a str, Vec<Role>, Vec<GuildChannel>))` - Success message key and empty vectors on successful setup
/// * `Err(Vec<&'a str>)` - Vector of error message keys if setup fails
///
/// # Behavior
///
/// The function follows this process:
/// 1. Creates or verifies categories (admin, nrp, rp)
/// 2. Creates or verifies channels within those categories
/// 3. Updates the server database object with new IDs
/// 4. Reorders channels to maintain consistent structure
/// 5. Persists changes to the database
///
/// If any step fails, the function performs automatic rollback by deleting all created resources.
///
/// # Errors
///
/// Returns error keys for:
/// - `setup__admin_category_not_created`: Failed to create admin category
/// - `setup__nrp_category_not_created`: Failed to create NRP category
/// - `setup__rp_category_not_created`: Failed to create RP category
/// - `setup__log_channel_not_created`: Failed to create log channel
/// - `setup__commands_channel_not_created`: Failed to create commands channel
/// - `setup__moderation_channel_not_created`: Failed to create moderation channel
/// - `setup__nrp_general_channel_not_created`: Failed to create NRP general channel
/// - `setup__rp_character_channel_not_created`: Failed to create character channel
/// - `setup__wiki_channel_not_created`: Failed to create wiki channel
/// - `setup__server_update_failed`: Failed to update database after successful creation
/// - `setup__rollback_failed`: Failed to rollback changes (critical error)
///
/// # Example
///
/// ```ignore
/// let result = complementary_setup(ctx, &mut server).await;
/// match result {
///     Ok((message_key, _, _)) => println!("Setup successful: {}", message_key),
///     Err(errors) => eprintln!("Setup failed with errors: {:?}", errors),
/// }
/// ```
///
/// # Notes
///
/// - The function is idempotent: existing channels are reused if they exist
/// - All created resources are tracked for potential rollback
/// - Channel reordering may fail silently without affecting overall success
/// - Requires appropriate bot permissions (Administrator recommended)
pub async fn complementary_setup<'a>(ctx: &Context<'_>, server : &'a mut Server, snapshot: Server) -> Result<&'a str, Error> {
    let mut created_categories: Vec<GuildChannel> = vec![];
    let mut errors: Vec<&str> = vec![];


    //Créer les catégories: admin, nrp, rp
    // Category creation follows an idempotent pattern:
    // 1. Check if category ID exists in database
    // 2. If exists, try to fetch from Discord (may have been manually deleted)
    // 3. If fetch fails or ID is None, create new category
    // 4. Track newly created categories for potential rollback
    let admin_category_permissions = get_admin_category_permission_set(
        server.everyone_role_id.clone().unwrap().id.into(),
        server.spectator_role_id.clone().unwrap().id.into(),
        server.player_role_id.clone().unwrap().id.into(),
        server.moderator_role_id.clone().unwrap().id.into());

    let admin_category_result = match server.admin_category_id{
        None => {
            match create_channel(ctx, tr!(*ctx, "admin_category_name"), ChannelType::Category, 0, admin_category_permissions, None).await {
                Ok(category) => { created_categories.push(category.clone()); Ok(category)}
                Err(e) => {errors.push("setup__admin_category_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.id.into()).await{
                Ok(channel) => {Ok(channel.guild().unwrap())}
                Err(_) => {
                    match create_channel(ctx, tr!(*ctx, "admin_category_name"), ChannelType::Category, 0, admin_category_permissions, None).await {
                        Ok(category) => {
                            server.admin_category_id((category.id.get(), IdType::Category));
                            created_categories.push(category.clone());
                            Ok(category)}
                        Err(e) => {errors.push("setup__admin_category_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    let nrp_category_result = match server.nrp_category_id{
        None => {
            match create_channel(ctx, tr!(*ctx, "nrp_category_name"), ChannelType::Category, 1, vec![], None).await {
                Ok(category) => {created_categories.push(category.clone()); Ok(category)}
                Err(e) => {errors.push("setup__nrp_category_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.id.into()).await{
                Ok(channel) => {Ok(channel.guild().unwrap())}
                Err(_) => {
                    match create_channel(ctx, tr!(*ctx, "nrp_category_name"), ChannelType::Category, 1, vec![], None).await {
                        Ok(category) => {
                            server.nrp_category_id((category.id.get(), IdType::Category));
                            created_categories.push(category.clone());
                            Ok(category)}
                        Err(e) => {errors.push("setup__nrp_category_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    let rp_category_result = match server.rp_category_id{
        None => {
            match create_channel(ctx, tr!(*ctx, "rp_category_name"), ChannelType::Category, 1, vec![], None).await {
                Ok(category) => {created_categories.push(category.clone()); Ok(category)}
                Err(e) => {errors.push("setup__rp_category_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.id.into()).await{
                Ok(channel) => {Ok(channel.guild().unwrap())}
                Err(_) => {
                    match create_channel(ctx, tr!(*ctx, "rp_category_name"), ChannelType::Category, 1, vec![], None).await {
                        Ok(category) => {
                            server.rp_category_id((category.id.get(), IdType::Category));
                            created_categories.push(category.clone());
                            Ok(category)}
                        Err(e) => {errors.push("setup__rp_category_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    // If any category creation failed, rollback all newly created categories
    // to maintain server consistency. Only categories added to created_categories
    // vector (i.e., newly created, not pre-existing) are deleted.
    if !errors.is_empty() {

        return Err("setup__roles_setup_failed".into())
    }

    let admin_category = admin_category_result.unwrap();
    let nrp_category = nrp_category_result.unwrap();
    let rp_category = rp_category_result.unwrap();

    let mut created_channels = vec![];
    let mut errors = vec![];

    // Channel creation follows the same idempotent pattern as categories:
    // - Check database for existing ID
    // - Verify channel still exists on Discord
    // - Create if missing, passing parent category ID to nest the channel
    // - Track all channels (new and existing) for potential rollback
    let log_channel_result = match server.log_channel_id{
        None => {
            let result = create_channel(ctx, tr!(*ctx, "log_channel_name"), ChannelType::Text, 0, vec![], Some(admin_category.clone().id.get())).await;
            match result {
                Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                Err(e) => { errors.push("setup__log_channel_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.id.into()).await{
                Ok(channel) => {created_channels.push(channel.clone().guild().unwrap().clone()); Ok(channel.guild().unwrap())}
                Err(_) => {
                    let result = create_channel(ctx, tr!(*ctx, "log_channel_name"), ChannelType::Text, 0, vec![], Some(admin_category.clone().id.get())).await;
                    match result{
                        Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                        Err(e) => { errors.push("setup__log_channel_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    let commands_channel_result = match server.commands_channel_id{
        None => {
            let result = create_channel(ctx, tr!(*ctx, "commands_channel_name"), ChannelType::Text, 0, vec![], Some(admin_category.clone().id.get())).await;
            match result {
                Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                Err(e) => {errors.push("setup__commands_channel_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.id.into()).await{
                Ok(channel) => {created_channels.push(channel.clone().guild().unwrap().clone()); Ok(channel.guild().unwrap())}
                Err(_) => {
                    let result = create_channel(ctx, tr!(*ctx, "commands_channel_name"), ChannelType::Text, 0, vec![], Some(admin_category.clone().id.get())).await;
                    match result{
                        Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                        Err(e) => {errors.push("setup__commands_channel_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    let moderation_channel_result = match server.moderation_channel_id{
        None => {
            let result = create_channel(ctx, tr!(*ctx, "moderation_channel_name"), ChannelType::Text, 0, vec![], Some(admin_category.clone().id.get())).await;
            match result {
                Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                Err(e) => {errors.push("setup__moderation_channel_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.id.into()).await{
                Ok(channel) => {created_channels.push(channel.clone().guild().unwrap().clone()); Ok(channel.guild().unwrap())}
                Err(_) => {
                    let result = create_channel(ctx, tr!(*ctx, "moderation_channel_name"), ChannelType::Text, 0, vec![], Some(admin_category.clone().id.get())).await;
                    match result{
                        Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                        Err(e) => {errors.push("setup__moderation_channel_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    let nrp_general_channel_result = match server.nrp_general_channel_id{
        None => {
            let result = create_channel(ctx, tr!(*ctx, "nrp_general_channel_name"), ChannelType::Text, 0, vec![], Some(nrp_category.clone().id.get())).await;
            match result {
                Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                Err(e) => {errors.push("setup__nrp_general_channel_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.id.into()).await{
                Ok(channel) => {created_channels.push(channel.clone().guild().unwrap().clone()); Ok(channel.guild().unwrap())}
                Err(_) => {
                    let result = create_channel(ctx, tr!(*ctx, "nrp_general_channel_name"), ChannelType::Text, 0, vec![], Some(nrp_category.clone().id.get())).await;
                    match result{
                        Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                        Err(e) => {errors.push("setup__nrp_general_channel_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    let character_channel_permissions = get_rp_character_permission_set(server.player_role_id.clone().unwrap().id.into());

    let rp_character_channel = match server.rp_character_channel_id{
        None => {
            let result = create_channel(ctx, tr!(*ctx, "rp_character_channel_name"), ChannelType::Text, 0, character_channel_permissions, Some(rp_category.clone().id.get())).await;
            match result {
                Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                Err(e) => {errors.push("setup__rp_character_channel_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.id.into()).await{
                Ok(channel) => {created_channels.push(channel.clone().guild().unwrap().clone()); Ok(channel.guild().unwrap())}
                Err(_) => {
                    let result = create_channel(ctx, tr!(*ctx, "rp_character_channel_name"), ChannelType::Text, 0, character_channel_permissions, Some(rp_category.clone().id.get())).await;
                    match result{
                        Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                        Err(e) => {errors.push("setup__rp_character_channel_not_created"); Err(e)}
                    }
                }
            }
        }
    };


    let wiki_channel_result = match server.rp_wiki_channel_id{
        None => {
            let result = create_channel(ctx, tr!(*ctx, "rp_wiki_channel_name"), ChannelType::Forum, 0, vec![], Some(rp_category.clone().id.get())).await;
            match result {
                Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                Err(e) => {errors.push("setup__wiki_channel_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.id.into()).await{
                Ok(channel) => {created_channels.push(channel.clone().guild().unwrap().clone()); Ok(channel.guild().unwrap())}
                Err(_) => {
                    let result = create_channel(ctx, tr!(*ctx, "rp_wiki_channel_name"), ChannelType::Forum, 0, vec![], Some(rp_category.clone().id.get())).await;
                    match result{
                        Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                        Err(e) => {errors.push("setup__wiki_channel_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    // If any channel creation failed, rollback all tracked channels.
    // Note: This includes both newly created and pre-existing channels from the
    // created_channels vector, ensuring complete cleanup on failure.
    created_channels.append(&mut created_categories);
    if !errors.is_empty()  {
        for channel in created_channels {
            match channel.clone().delete(ctx).await {
                Ok(_) => {}
                Err(_) => {
                    server.rollback(ctx, snapshot).await;
                    return Err("setup__rollback_failed".into())
                }
            }
        }
        return Err("setup__channel_setup_failed".into())
    }

    let log_channel = log_channel_result.unwrap();
    let commands_channel = commands_channel_result.unwrap();
    let moderation_channel = moderation_channel_result.unwrap();
    let nrp_general_channel = nrp_general_channel_result.unwrap();
    let rp_character_channel = rp_character_channel.unwrap();
    let wiki_channel = wiki_channel_result.unwrap();



    server.nrp_category_id(Id{ id: nrp_category.id.get(), id_type: IdType::Category });
    server.rp_category_id(Id{ id: rp_category.id.get(), id_type: IdType::Category });
    server.admin_category_id(Id{ id: admin_category.id.get(), id_type: IdType::Category });
    server.log_channel_id(Id{id: log_channel.id.get(), id_type: IdType::Channel });
    server.commands_channel_id(Id{id: commands_channel.id.get(), id_type: IdType::Channel });
    server.moderation_channel_id(Id{id: moderation_channel.id.get(), id_type: IdType::Channel });
    server.nrp_general_channel_id(Id{id: nrp_general_channel.id.get(), id_type: IdType::Channel });
    server.rp_character_channel_id(Id{id: rp_character_channel.id.get(), id_type: IdType::Channel });
    server.rp_wiki_channel_id(Id{id: wiki_channel.id.get(), id_type: IdType::Channel });

    // Reorder categories to maintain consistent structure:
    // Position 0-3: Our managed categories (admin, nrp, rp, road)
    // Position 4+: All other existing channels/categories in the guild
    // This ensures our categories appear at the top while preserving
    // any user-created channels below them.
    let mut channel_order = vec![(admin_category.id, 0), (nrp_category.id, 1), (rp_category.id, 2), (server.road_category_id.unwrap().id.into(), 3)];
    let channels = ctx.guild_id().unwrap().channels(ctx).await.unwrap();

    let allowed = [
        admin_category.id.get(),
        nrp_category.id.get(),
        rp_category.id.get(),
        server.road_category_id.unwrap().id.into()
    ];

    for (channel_id, _) in channels{
        if !allowed.contains(&channel_id.get()) {
            channel_order.push((channel_id, 4))
        }
    }


    let reorder_result = ctx.guild_id().unwrap().reorder_channels(ctx, channel_order).await;

    match reorder_result {
        Ok(_) => {}
        Err(_) => {}
    }

    // Persist all channel/category IDs to database. On failure, perform complete
    // rollback of both channels and categories to prevent orphaned Discord resources.
    // This ensures atomicity: either everything succeeds and is saved, or everything
    // is rolled back and the server state remains unchanged.
    match server.update().await {
        Ok(_) => {}
        Err(_) => {
            for channel in created_channels{
                match channel.clone().delete(ctx).await {
                    Ok(_) => {}
                    Err(_) => {
                        log!(Level::Error, "Error while setuping and rollbacking.\
                     universe_id: {}\
                     server_id: {}\
                     channel_id: {}", server.universe_id, server.server_id, channel.id);
                        return Err("setup__rollback_failed".into())
                    }
                }
            }
            return Err("setup__server_update_failed".into())}
    };

    Ok("setup__setup_success_message")
}