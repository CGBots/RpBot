use log::{log, Level};
use serenity::all::{ChannelType, GuildChannel};
use crate::database::server::{Id, IdType, Server};
use crate::discord::channels::{create_channel, get_admin_category_permission_set, get_rp_character_permission_set};
use crate::discord::poise_structs::{Context, Error};
use crate::tr;

/// Asynchronous function that sets up the necessary categories and channels for a server.
/// This function creates the required channel categories and text channels for the server setup
/// while resolving existing ones if already created. It follows a specific hierarchy and permissions setup.
///
/// # Parameters
/// - `ctx`: A reference to the async `Context` object for executing Discord operations such as API calls.
/// - `server`: A mutable reference to the server configuration that gets modified during the setup.
/// - `snapshot`: A snapshot of the original server configuration to serve as a fallback during the process.
///
/// # Returns
/// A `Result` containing:
/// - `Ok(&'a str)`: A string slice indicating successful setup.
/// - `Err(Error)`: An error if any portion of the setup fails.
///
/// # Errors
/// The function returns an error in the following cases:
/// - If permissions for categories cannot be obtained or set up.
/// - If any required category (admin, RP, NRP) or text channel
///   (e.g., log, commands, moderation, general) cannot be created.
/// - Discord API errors during channel/category creation or retrieval.
///
/// # Behavior
/// - The function attempts to create the following categories:
///   - Admin category
///   - Non-roleplay (NRP) category
///   - Roleplay (RP) category
/// - If a category already exists, it retrieves the data.
///   If the category ID is invalid, it creates the category anew.
/// - Within the Admin category, it attempts to create or retrieve channels:
///   - Log channel
///   - Commands channel
///   - Moderation channel
/// - An additional general channel for each of the NRP and RP categories is created.
/// - Tracks errors during the creation process and halts further processing in case of failure.
///
/// # Notes
/// - The function leverages pre-defined localized strings for category and channel names.
///
/// # Examples
/// ```rust
/// let mut server_config = configure_server();
/// let setup_result = complementary_setup(ctx, &mut server_config, snapshot).await;
///
/// match setup_result {
///     Ok(success_msg) => println!("Setup completed successfully: {}", success_msg),
///     Err(e) => eprintln!("Setup failed: {}", e),
/// }
/// ```
///
/// # Related Functions
/// - `create_channel`: Handles the actual API call for creating Discord channels.
/// - `get_admin_category_permission_set`: Configures the permission overrides for the Admin category.
///
/// # Context
/// Used in bots or services needing automated Discord server provisioning and setup. This function
/// follows best practices like resolving existing entities and minimal privilege access for channels.
///
/// # Dependencies
/// Relies on asynchronous Discord API interactions through the `Context` object, as well as utilities
/// for managing permissions, channel types, and localized translations.
pub async fn complementary_setup<'a>(ctx: &Context<'_>, server : &'a mut Server, snapshot: Server) -> Result<&'a str, Error> {
    let mut errors: Vec<&str> = vec![];

    let admin_category_permissions = get_admin_category_permission_set(
        server.everyone_role_id.clone().unwrap().id.into(),
        server.spectator_role_id.clone().unwrap().id.into(),
        server.player_role_id.clone().unwrap().id.into(),
        server.moderator_role_id.clone().unwrap().id.into());

    let admin_category_result = match server.admin_category_id{
        None => {
            match create_channel(ctx, tr!(*ctx, "admin_category_name"), ChannelType::Category, 0, admin_category_permissions, None).await {
                Ok(category) => { Ok(category)}
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
                Ok(category) => { Ok(category)}
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
                Ok(category) => { Ok(category)}
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
                            Ok(category)}
                        Err(e) => {errors.push("setup__rp_category_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    if !errors.is_empty() {

        return Err("setup__roles_setup_failed".into())
    }

    let admin_category = admin_category_result.unwrap();
    let nrp_category = nrp_category_result.unwrap();
    let rp_category = rp_category_result.unwrap();

    let mut errors = vec![];

    let log_channel_result = match server.log_channel_id{
        None => {
            let result = create_channel(ctx, tr!(*ctx, "log_channel_name"), ChannelType::Text, 0, vec![], Some(admin_category.clone().id.get())).await;
            match result {
                Ok(channel) => { Ok(channel)}
                Err(e) => { errors.push("setup__log_channel_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.id.into()).await{
                Ok(channel) => { Ok(channel.guild().unwrap())}
                Err(_) => {
                    let result = create_channel(ctx, tr!(*ctx, "log_channel_name"), ChannelType::Text, 0, vec![], Some(admin_category.clone().id.get())).await;
                    match result{
                        Ok(channel) => { Ok(channel)}
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
                Ok(channel) => { Ok(channel)}
                Err(e) => {errors.push("setup__commands_channel_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.id.into()).await{
                Ok(channel) => { Ok(channel.guild().unwrap())}
                Err(_) => {
                    let result = create_channel(ctx, tr!(*ctx, "commands_channel_name"), ChannelType::Text, 0, vec![], Some(admin_category.clone().id.get())).await;
                    match result{
                        Ok(channel) => { Ok(channel)}
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
                Ok(channel) => { Ok(channel)}
                Err(e) => {errors.push("setup__moderation_channel_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.id.into()).await{
                Ok(channel) => { Ok(channel.guild().unwrap())}
                Err(_) => {
                    let result = create_channel(ctx, tr!(*ctx, "moderation_channel_name"), ChannelType::Text, 0, vec![], Some(admin_category.clone().id.get())).await;
                    match result{
                        Ok(channel) => { Ok(channel)}
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
                Ok(channel) => { Ok(channel)}
                Err(e) => {errors.push("setup__nrp_general_channel_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.id.into()).await{
                Ok(channel) => { Ok(channel.guild().unwrap())}
                Err(_) => {
                    let result = create_channel(ctx, tr!(*ctx, "nrp_general_channel_name"), ChannelType::Text, 0, vec![], Some(nrp_category.clone().id.get())).await;
                    match result{
                        Ok(channel) => { Ok(channel)}
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
                Ok(channel) => { Ok(channel)}
                Err(e) => {errors.push("setup__rp_character_channel_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.id.into()).await{
                Ok(channel) => { Ok(channel.guild().unwrap())}
                Err(_) => {
                    let result = create_channel(ctx, tr!(*ctx, "rp_character_channel_name"), ChannelType::Text, 0, character_channel_permissions, Some(rp_category.clone().id.get())).await;
                    match result{
                        Ok(channel) => { Ok(channel)}
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
                Ok(channel) => { Ok(channel)}
                Err(e) => {errors.push("setup__wiki_channel_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.id.into()).await{
                Ok(channel) => { Ok(channel.guild().unwrap())}
                Err(_) => {
                    let result = create_channel(ctx, tr!(*ctx, "rp_wiki_channel_name"), ChannelType::Forum, 0, vec![], Some(rp_category.clone().id.get())).await;
                    match result{
                        Ok(channel) => { Ok(channel)}
                        Err(e) => {errors.push("setup__wiki_channel_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    if !errors.is_empty()  {
        server.rollback(ctx, snapshot).await;
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

    match server.update().await {
        Ok(_) => {}
        Err(_) => {
            server.rollback(ctx, snapshot).await;
            return Err("setup__server_update_failed".into())}
    };

    Ok("setup__setup_success_message")
}