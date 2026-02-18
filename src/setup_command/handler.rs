//! Setup command handler for Discord server configuration.
//!
//! This module provides the `/setup` command that allows server administrators to configure
//! their Discord server with roles, channels, and categories required for roleplay functionality.
//!
//! # Features
//!
//! - **Full Setup**: Creates all roles (Admin, Moderator, Spectator, Player), categories
//!   (Administration, Non-RP, RolePlay, Roads), and channels (character sheets, wiki, logs,
//!   commands, moderation, general).
//! - **Partial Setup**: Creates only the minimum necessary roles and channels.
//! - **Validation**: Checks if the server is registered in a universe before proceeding.
//! - **Interactive Confirmation**: Prompts users if the server is already configured.
//!
//! # Permissions
//!
//! This command requires `ADMINISTRATOR` permissions and can only be used in guilds.
use poise::{CreateReply};
use serenity::all::{ButtonStyle, Color, ComponentInteractionCollector, CreateActionRow, CreateButton, CreateEmbed};
use crate::database::server::Server;
use crate::discord::poise_structs::{Context, Error};
use crate::tr;
use crate::setup_command::full_setup::full_setup;
use crate::setup_command::partial_setup::partial_setup;
use crate::utility::reply::reply;

/// The type of setup to perform on the Discord server.
///
/// This enum defines two configuration modes for server setup.
#[derive(Debug, poise::ChoiceParameter, Clone, Copy)]
pub enum SetupType {
    /// Creates all roles, categories, and channels including admin tools,
    /// character sheets, wiki, and moderation channels.
    FullSetup,
    /// Creates only the minimum necessary roles and channels for basic functionality.
    PartialSetup
}

/// Configures the Discord server with roles, channels, and categories for roleplay.
///
/// This command initializes or updates the server configuration based on the selected setup type.
/// It validates that the server is registered in a universe and prompts for confirmation if
/// existing configuration is detected.
///
/// # Arguments
///
/// * `ctx` - The command context containing guild and user information.
/// * `setup_type` - The type of setup to perform (Full or Partial).
///
/// # Returns
///
/// * `Ok(())` - If the setup completes successfully or is cancelled by the user.
/// * `Err(Error)` - If there are issues with permissions, database access, or Discord API calls.
///
/// # Errors
///
/// This function will return an error if:
/// - The guild is not registered in any universe
/// - The server configuration cannot be retrieved from the database
/// - Role or channel creation fails due to insufficient permissions
/// - The database update fails
///
/// # Examples
///
/// ```text
/// /setup setup_type:Full
/// /setup setup_type:Partial
/// ```
///
/// # Permissions
///
/// Requires `ADMINISTRATOR` permission and must be used in a guild context.
///
///
// ... existing code ...
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR", guild_only)]
pub async fn setup(ctx: Context<'_>, setup_type: SetupType) -> Result<(), Error> {
    ctx.defer().await?;
    let result = _setup(&ctx, setup_type).await;
    reply(ctx, result).await?;
    Ok(())
}

pub async fn _setup(ctx: &Context<'_>, setup_type: SetupType) -> Result<&'static str, Error> {

    let guild_id = ctx.guild_id().unwrap();

    let Some(mut server) = Server::get_server_by_id(guild_id.get().to_string()).await? else {return Err("setup__server_not_found".into())};
    let server_snapshot = server.clone().snaphot(ctx).await;

    // Check if any server configuration already exists by testing all possible fields.
    // This comprehensive check detects partial setups where only some roles/channels were created.
    // If any field is populated, we need user confirmation before proceeding, as the setup
    // process will attempt to create missing elements and may overwrite existing configuration.
    if server.admin_role_id.is_some()
        || server.moderator_role_id.is_some()
        || server.spectator_role_id.is_some()
        || server.player_role_id.is_some()
        || server.road_category_id.is_some()
        || server.rp_wiki_channel_id.is_some()
        || server.admin_category_id.is_some()
        || server.nrp_category_id.is_some()
        || server.rp_category_id.is_some()
        || server.rp_character_channel_id.is_some() {

        // Present an interactive confirmation dialog with Cancel/Continue buttons.
        // The ComponentInteractionCollector waits up to 60 seconds for the user to respond.
        // This prevents accidental overwrites by requiring explicit user confirmation.
        // Three outcomes: timeout (cancel), explicit cancel, or continue (any other button).
        let reply = {
            let components = vec![CreateActionRow::Buttons(vec![
                CreateButton::new("cancel")
                    .style(ButtonStyle::Primary)
                    .label(tr!(*ctx, "cancel_setup")),
                CreateButton::new("continue")
                    .style(ButtonStyle::Danger)
                    .label(tr!(*ctx, "continue_setup")),
            ])];

            CreateReply::default()
                .embed(
                    CreateEmbed::new()
                        .color(Color::from_rgb(0xff, 0x98, 0))
                        .title(crate::translation::get(*ctx, &"setup__continue_setup_message", Some("title"), None))
                        .description(crate::translation::get(*ctx, &"setup__continue_setup_message", Some("message"), None))
                )
                .components(components)
        };

        let message = ctx.send(reply.clone()).await.unwrap();

        let serenity_context = ctx.serenity_context();

        let interaction = ComponentInteractionCollector::new(&serenity_context)
            .author_id(ctx.author().id)
            .channel_id(ctx.channel_id())
            .timeout(std::time::Duration::from_secs(60))
            .await;
        match interaction {
            None => {
                message.delete(*ctx).await?;
                return Err("setup__server_already_setup_timeout".into()); }
            Some(mci) => {
                mci.defer(ctx).await?;
                message.edit(*ctx, reply.components(vec!())).await?;
                match mci.data.custom_id.as_str() {
                    "cancel" => {
                        message.delete(*ctx).await?;
                        return Ok("setup_server__cancelled");
                    }
                    _ => {}
                };
            }
        };
    }

    let result = match setup_type {
        SetupType::FullSetup => { full_setup(ctx, &mut server, server_snapshot).await }
        SetupType::PartialSetup => { partial_setup(ctx, &mut server, server_snapshot).await }
    };

    server.update().await?;

    match result {
        Ok(_) => {Ok("setup_server__success")}
        Err(_) => {Err("setup_server__failed".into())}
    }
}