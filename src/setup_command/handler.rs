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

#[derive(Debug, poise::ChoiceParameter)]
pub enum SetupType {
    FullSetup,
    PartialSetup
}

#[poise::command(slash_command, required_permissions = "ADMINISTRATOR", guild_only)]
pub async fn setup(ctx: Context<'_>, setup_type: SetupType) -> Result<(), Error> {
    ctx.defer().await.unwrap();


    let guild_id = ctx.guild_id().unwrap();
    let universe_result = Universe::get_universe_by_server_id(guild_id.get()).await;

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
            let errs: Vec<String> = err.iter()
                .map(|error|"► ".to_owned() + &tr!(ctx, error))
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