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
use log::{log, Level};
use poise::{CreateReply};
use serenity::all::{ButtonStyle, ComponentInteractionCollector, CreateActionRow, CreateButton, CreateEmbed};
use serenity::model::Colour;
use crate::database::server::Server;
use crate::database::universe::Universe;
use crate::discord::poise_structs::{Context, Error};
use crate::tr;
use crate::setup_command::full_setup::full_setup;
use crate::setup_command::partial_setup::partial_setup;

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
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR", guild_only)]
pub async fn setup(ctx: Context<'_>, setup_type: SetupType) -> Result<(), Error> {
    _setup(ctx, setup_type).await.map_err(|e| e.into())
}


pub async fn _setup(ctx: Context<'_>, setup_type: SetupType) -> Result<(), Error> {
    ctx.defer().await.unwrap();


    let guild_id = ctx.guild_id().unwrap();
    let universe_result = Universe::get_universe_by_server_id(guild_id.get()).await;

    // Retrieve the universe ID associated with this Discord server.
    // This uses nested match statements to handle both database errors and the case where
    // no universe is found. Both scenarios result in the same error message to the user,
    // but they represent different failure modes: database connectivity issues vs. missing data.
    let universe_id = match universe_result {
        Ok(cursor) => {
            match cursor{
                None => {
                    let message = ctx.reply_builder(
                        CreateReply::default()
                            .embed(CreateEmbed::new()
                                .color(Colour::from_rgb(0, 255, 0))
                                .description(tr!(ctx, "setup__universe_not_found"))
                                .title(tr!(ctx, "setup__setup_error_title")))
                    ).reply(true);
                    ctx.send(message).await?; return Ok(())}
                Some(universe) => {universe.universe_id.to_string()}
            }
        }
        Err(_) => {
            let message = ctx.reply_builder(
            CreateReply::default()
                .embed(CreateEmbed::new()
                    .color(Colour::from_rgb(0, 255, 0))
                    .description(tr!(ctx, "setup__universe_not_found"))
                    .title(tr!(ctx, "setup__setup_error_title")))
        ).reply(true);
            ctx.send(message).await?; return Ok(())}
    };

    let server = Server::get_server_by_id(universe_id, guild_id.get().to_string()).await;

    // Retrieve the server configuration from the database.
    // Uses chained error checking: first checks if the query failed, then checks if the
    // result is None. The double unwrap() is safe here because we've verified both conditions.
    // This pattern ensures the server exists before proceeding with setup operations.
    let mut server = if server.is_err() || server.clone().unwrap().is_none(){
        let message = ctx.reply_builder(
            CreateReply::default()
                .embed(CreateEmbed::new()
                    .color(Colour::from_rgb(0, 255, 0))
                    .description(tr!(ctx, "setup__server_not_found"))
                    .title(tr!(ctx, "setup__setup_error_title")))
        ).reply(true);
        ctx.send(message).await?; return Ok(())
    }
    else {server.unwrap().unwrap()};

    // Check if any server configuration already exists by testing all possible fields.
    // This comprehensive check detects partial setups where only some roles/channels were created.
    // If any field is populated, we need user confirmation before proceeding, as the setup
    // process will attempt to create missing elements and may overwrite existing configuration.
    if server.admin_role_id.id.is_some()
        || server.moderator_role_id.id.is_some()
        || server.spectator_role_id.id.is_some()
        || server.player_role_id.id.is_some()
        || server.road_category_id.id.is_some()
        || server.rp_wiki_channel_id.id.is_some()
        || server.admin_category_id.id.is_some()
        || server.character_channel_id.id.is_some()
        || server.nrp_category_id.id.is_some()
        || server.rp_category_id.id.is_some(){

        // Present an interactive confirmation dialog with Cancel/Continue buttons.
        // The ComponentInteractionCollector waits up to 60 seconds for the user to respond.
        // This prevents accidental overwrites by requiring explicit user confirmation.
        // Three outcomes: timeout (cancel), explicit cancel, or continue (any other button).
        let reply = {
            let components = vec![CreateActionRow::Buttons(vec![
                CreateButton::new("cancel")
                    .style(ButtonStyle::Primary)
                    .label(tr!(ctx, "cancel_setup")),
                CreateButton::new("continue")
                    .style(ButtonStyle::Danger)
                    .label(tr!(ctx, "continue_setup")),
            ])];

            CreateReply::default()
                .content(tr!(ctx, "continue_setup_message"))
                .components(components)
        };

        let message = ctx.send(reply).await.unwrap();

        let interaction = ComponentInteractionCollector::new(ctx)
            .author_id(ctx.author().id)
            .channel_id(ctx.channel_id())
            .timeout(std::time::Duration::from_secs(60))
            .await;
        match interaction {
            None => {
                message.delete(ctx).await.unwrap();
                let message = ctx.reply_builder(
                    CreateReply::default()
                        .embed(CreateEmbed::new()
                            .color(Colour::from_rgb(0, 255, 0))
                            .description(tr!(ctx, "setup__server_already_setup_timeout"))
                            .title(tr!(ctx, "setup__setup_error_title")))
                ).reply(true);
                ctx.send(message).await?; return Ok(())
            }
            Some(mci) => {
                message.delete(ctx).await.unwrap();
                let interaction_button_id = mci.data.custom_id.as_str();
                match interaction_button_id {
                    "cancel" => {
                        let message = ctx.reply_builder(
                            CreateReply::default()
                                .embed(CreateEmbed::new()
                                    .color(Colour::from_rgb(0, 255, 0))
                                    .description(tr!(ctx, "setup__canceled"))
                                    .title(tr!(ctx, "setup__setup_success_title")))
                        ).reply(true);
                        ctx.send(message).await?; return Ok(())
                    }
                    _ => {}
                }
            }
        }
    }
    
    let result = match setup_type {
        SetupType::FullSetup => { full_setup(ctx, &mut server).await }
        SetupType::PartialSetup => { partial_setup(ctx, &mut server).await }
    };

    match result{
        Ok(result) => {
            let message = ctx.reply_builder(
                CreateReply::default()
                    .embed(CreateEmbed::new()
                        .color(Colour::from_rgb(0, 255, 0))
                        .description(tr!(ctx, result.0))
                        .title(tr!(ctx, "setup__setup_success_title")))
            ).reply(true);
            ctx.send(message).await?;
        }
        Err(err) => {
            // Format error messages for display by prepending arrow characters and translating.
            // Each error key is translated to the user's language, then prefixed with "► " for
            // visual hierarchy. The resulting strings are joined with newlines to create a
            // bulleted list that's embedded in the Discord message.
            let errs: Vec<String> = err.iter()
                .map(|error|"► ".to_string() + tr!(ctx, error).as_str())
                .collect();

            let errors = errs.join("\n");

            let message = ctx.reply_builder(
                CreateReply::default()
                    .embed(CreateEmbed::new()
                        .color(Colour::from_rgb(255, 0, 0))
                        .description(tr!(ctx, "setup__setup_error_message", errors: errors.as_str()))
                        .title(tr!(ctx, "setup__setup_error_title"))
                    )
            ).reply(true);
            let message_result = ctx.send(message).await;
            match message_result {
                Ok(_) => {}
                Err(_) => {
                    log!(Level::Error, "Error while sending error message:\
                     {}", tr!(ctx, "setup__setup_error_message", errors: errors.as_str()));
                }
            }
        }
    }

    Ok(())
}