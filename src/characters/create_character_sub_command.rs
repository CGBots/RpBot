use futures::{StreamExt, TryStreamExt};
use std::time::Duration;
use serenity::all::{ButtonStyle, Color, ComponentInteraction, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateInputText, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateModal, EditMessage, Embed, EmbedField, InputTextStyle, Mentionable, Message, ModalInteraction, Permissions, QuickModalResponse};
use crate::discord::poise_structs::{Context, Error};
use crate::utility::reply::reply;
use serenity::client::Context as SerenityContext;
use serenity::http::CacheHttp;
use serenity::utils::CreateQuickModal;
use crate::database::server::{get_server_by_id};
use crate::{tr, tr_locale};
use crate::database::characters::Character;
use crate::database::stats::{Stat, StatValue};
use crate::database::universe::get_universe_by_id;

pub static CHARACTER_MODAL_TITLE: &str = "character_modal_title";
pub static MODIFY_CHARACTER_BUTTON_CUSTOM_ID: &str = "create_character__modify_character";
pub static DELETE_CHARACTER_BUTTON_CUSTOM_ID: &str = "create_character__delete_character";
pub static SUBMIT_CHARACTER_BUTTON_CUSTOM_ID: &str = "create_character__submit_character";
pub static ACCEPT_CHARACTER_BUTTON_CUSTOM_ID: &str = "create_character__accept_character";
pub static REJECT_CHARACTER_BUTTON_CUSTOM_ID: &str = "create_character__refuse_character";
pub static CREATE_CHARACTER_SUBMIT_NOTIFICATION: &str = "create_character__submit_notification";

pub static CHARACTER_NAME: &str = "character_name";
pub static CHARACTER_DESCRIPTION: &str = "character_description";
pub static CHARACTER_STORY: &str = "character_story";
pub static CHARACTER_SPECIAL_REQUEST: &str = "character_special_request";
pub static CHARACTER_INSTRUCTION: &str = "character_instruction";
pub static CHARACTER_REJECT_REASON: &str = "character_reject_reason";

#[poise::command(slash_command, guild_only)]
pub async fn create_character(
    ctx: Context<'_>
) -> Result<(), Error> {
    let result = _create_character(ctx).await;
    if result.is_err(){ reply(ctx, result).await?;}
    Ok(())
}

pub async fn _create_character(ctx: Context<'_>) -> Result<&'static str, Error>{
    // 2 process qui échangent
    //  validation par les modos/admins
    //  modification par l'utilisateur DONE
    // la validation ouvre un modal pour demander les stats du joueur.
    // les infos sont enregistrés
    // le role joueur est attribué

    let guild_id = ctx.guild_id().unwrap();

    let server = get_server_by_id(guild_id.get()).await;
    let server = match server {
        Ok(result) => {
            match result{
                None => {return Err("create_character__no_universe_found".into())}
                Some(server) => {server}
            }
        }
        Err(_) => {return Err("create_character__database_error".into())}
    };

    if server.rp_character_channel_id.unwrap().id != ctx.channel_id().get(){
        return Err("create_character__wrong_channel".into())
    }

    let player_result = server.has_character(ctx.author().id.get()).await?;
    match player_result {
        None => {}
        Some(_) => {
            ctx.send(poise::CreateReply::default()
                .embed(CreateEmbed::new()
                    .color(Color::from_rgb(255, 0, 0))
                    .title(crate::translation::get(ctx, "create_character__character_already_existing", Some("title"), None))
                    .description(crate::translation::get(ctx, "create_character__character_already_existing", Some("message"), None))
                )
                .ephemeral(true)
            ).await?;
            return Ok("create_character__character_already_existing")
        }
    }

    let app_ctx = match ctx.clone() {
        Context::Application(app_ctx) => app_ctx,
        _ => return Err("create_character__guild_only".into()),
    };

    let modal = CreateQuickModal::new(tr!(ctx, CHARACTER_MODAL_TITLE))
        .field(
            CreateInputText::new(InputTextStyle::Short, tr!(ctx, CHARACTER_NAME), CHARACTER_NAME)
                .required(true).min_length(3).max_length(64))
        .field(
            CreateInputText::new(InputTextStyle::Paragraph, tr!(ctx, CHARACTER_DESCRIPTION), CHARACTER_DESCRIPTION)
                .required(true).max_length(1024)
                .value(tr!(ctx, CHARACTER_INSTRUCTION))
        )
        .field(
            CreateInputText::new(InputTextStyle::Paragraph, tr!(ctx, CHARACTER_STORY), CHARACTER_STORY)
                .required(true).max_length(1024)
        )
        .field(
            CreateInputText::new(InputTextStyle::Paragraph, tr!(ctx, CHARACTER_SPECIAL_REQUEST), CHARACTER_SPECIAL_REQUEST)
                .required(false).max_length(1024)
        )
        .timeout(Duration::from_secs(30));

    let interaction = app_ctx.interaction.quick_modal(ctx.serenity_context(), modal).await?;
    let modal_response = match interaction {
        Some(interaction) => {
            interaction.interaction.create_response(ctx, CreateInteractionResponse::Acknowledge).await?;
            interaction }
        None => {
            return Err("create_character__timed_out".into()) }
    };

    let inputs = modal_response.inputs;

    let buttons = vec![
        CreateActionRow::Buttons(
            vec![
                CreateButton::new(SUBMIT_CHARACTER_BUTTON_CUSTOM_ID).label(tr!(ctx, SUBMIT_CHARACTER_BUTTON_CUSTOM_ID)).style(ButtonStyle::Success),
                CreateButton::new(MODIFY_CHARACTER_BUTTON_CUSTOM_ID).label(tr!(ctx, MODIFY_CHARACTER_BUTTON_CUSTOM_ID)).style(ButtonStyle::Primary),
                CreateButton::new(DELETE_CHARACTER_BUTTON_CUSTOM_ID).label(tr!(ctx, DELETE_CHARACTER_BUTTON_CUSTOM_ID)).style(ButtonStyle::Danger),
            ]
        )
    ];

    let result_message = app_ctx.channel_id().send_message(ctx, CreateMessage::new().embed(
        CreateEmbed::new()
            .footer(CreateEmbedFooter::new(ctx.author().id.get().to_string()))
            .title(inputs[0].clone())
            .field(tr!(ctx, CHARACTER_DESCRIPTION), inputs[1].clone(), true)
            .field(tr!(ctx, CHARACTER_STORY), inputs[2].clone(), true)
            .field(tr!(ctx, CHARACTER_SPECIAL_REQUEST), inputs[3].clone(), false)
            .author(CreateEmbedAuthor::new(ctx.author().name.as_str()))
            .color(Color::from_rgb(112, 190, 255))
    )
        .components(buttons)
    ).await;

    match result_message {
        Ok(_) => {
            Ok("create_character__submited")
        }
        Err(_) => { Err("create_place__character_too_long".into()) }
    }
}

pub async fn delete_character(ctx: SerenityContext, component_interaction: ComponentInteraction) -> Result<&'static str, Error>{
    let interaction = component_interaction.clone();
    let user_id_string = interaction.user.id.get().to_string();
    let user = user_id_string.as_str();
    let character = interaction.message.embeds[0].clone().footer.unwrap();
    let character_user = character.text.as_str();

    if user != character_user{
        let _ = component_interaction.create_response(ctx, CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(tr_locale!(component_interaction.locale.as_str(), "create_character__not_owner")).ephemeral(true)
        )).await?;
        return Err("create_character__not_owner".into())
    }

    component_interaction.message.delete(ctx).await?;
    Ok("delete_character")
}
pub async fn submit_character(ctx: SerenityContext, component_interaction: ComponentInteraction) -> Result<&'static str, Error> {
    let interaction = component_interaction.clone();
    let user_id_string = interaction.user.id.get().to_string();
    let user = user_id_string.as_str();
    let character = interaction.message.embeds[0].clone().footer.unwrap();
    let character_user = character.text.as_str();

    if user != character_user{
        let _ = component_interaction.create_response(ctx.clone(), CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(tr_locale!(component_interaction.locale.as_str(), "create_character__not_owner")).ephemeral(true)
        )).await?;
    }

    let buttons = vec![
        CreateActionRow::Buttons(
            vec![
                CreateButton::new(ACCEPT_CHARACTER_BUTTON_CUSTOM_ID).label(tr_locale!(component_interaction.locale.as_str(), ACCEPT_CHARACTER_BUTTON_CUSTOM_ID)).style(ButtonStyle::Success),
                CreateButton::new(REJECT_CHARACTER_BUTTON_CUSTOM_ID).label(tr_locale!(component_interaction.locale.as_str(), REJECT_CHARACTER_BUTTON_CUSTOM_ID )).style(ButtonStyle::Danger),
            ]
        )
    ];

    let message = component_interaction.message.clone();
    let embed: CreateEmbed = message.embeds[0].clone().into();

    let result_message =
        component_interaction.channel_id.edit_message(ctx.clone(), message.id, EditMessage::new().embed(
            embed.color(Color::from_rgb(0, 255, 0))
        )
            .components(buttons)
        ).await;

    let _ = component_interaction.create_response(ctx.clone(), CreateInteractionResponse::Acknowledge).await;

    let message = tr_locale!(component_interaction.locale.as_str(), CREATE_CHARACTER_SUBMIT_NOTIFICATION) + " " + component_interaction.message.link().as_str();

    if let Some(server) = get_server_by_id(component_interaction.guild_id.unwrap().get()).await? {
        if let Some(log_channel) = server.log_channel_id {
            let _ = ctx.http().send_message(
                log_channel.id.into(),
                vec![],
                &CreateMessage::new().content(message),
            ).await;
        }
    }


    match result_message {
        Ok(_) => {
            Ok("create_character__submited")
        }
        Err(_) => { Err("create_place__character_too_long".into()) }
    }
}

pub async fn modify_character(ctx: SerenityContext, component_interaction: ComponentInteraction) -> Result<&'static str, Error> {
     let interaction = component_interaction.clone();
     let user_id_string = interaction.user.id.get().to_string();
     let user = user_id_string.as_str();
     let character = interaction.message.embeds[0].clone().footer.unwrap();
     let character_user = character.text.as_str();

     if user != character_user{
         let _ = component_interaction.create_response(ctx.clone(), CreateInteractionResponse::Message(
             CreateInteractionResponseMessage::new().content(tr_locale!(component_interaction.locale.as_str(), "create_character__not_owner")).ephemeral(true)
         )).await?;
         return Err("create_character__not_owner".into())
     }

    let embed_fields = component_interaction.message.embeds[0].clone().fields;

    let modal = CreateQuickModal::new(tr_locale!(component_interaction.locale.as_str(), CHARACTER_MODAL_TITLE))
        .field(
            CreateInputText::new(InputTextStyle::Short, tr_locale!(component_interaction.locale.as_str(), CHARACTER_NAME), CHARACTER_NAME)
                .required(true).min_length(3).max_length(64)
                .value(component_interaction.message.embeds[0].title.clone().unwrap().as_str())
        )
        .field(
            CreateInputText::new(InputTextStyle::Paragraph, tr_locale!(component_interaction.locale.as_str(), CHARACTER_DESCRIPTION), CHARACTER_DESCRIPTION)
                .required(true).max_length(1024)
                .value(embed_fields[0].value.clone())
        )
        .field(
            CreateInputText::new(InputTextStyle::Paragraph, tr_locale!(component_interaction.locale.as_str(), CHARACTER_STORY), CHARACTER_STORY)
                .required(true).max_length(1024)
                .value(embed_fields[1].value.clone())
        )
        .field(
            CreateInputText::new(InputTextStyle::Paragraph, tr_locale!(component_interaction.locale.as_str(), CHARACTER_SPECIAL_REQUEST), CHARACTER_SPECIAL_REQUEST)
                .required(false).max_length(1024)
                .value(embed_fields[2].value.clone())
        )
        .timeout(Duration::from_secs(30));

    let interaction = component_interaction.quick_modal(&ctx, modal).await?;
    let modal_response = match interaction {
        Some(interaction) => {
            interaction.interaction.create_response(ctx.clone(), CreateInteractionResponse::Acknowledge).await?;
            interaction }
        None => {
            return Err("create_character__timed_out".into()) }
    };

    let inputs = modal_response.inputs.clone();

    let buttons = vec![
        CreateActionRow::Buttons(
            vec![
                CreateButton::new(SUBMIT_CHARACTER_BUTTON_CUSTOM_ID).label(tr_locale!(modal_response.interaction.locale.as_str(), SUBMIT_CHARACTER_BUTTON_CUSTOM_ID)).style(ButtonStyle::Success),
                CreateButton::new(MODIFY_CHARACTER_BUTTON_CUSTOM_ID).label(tr_locale!(modal_response.interaction.locale.as_str(), MODIFY_CHARACTER_BUTTON_CUSTOM_ID)).style(ButtonStyle::Primary),
                CreateButton::new(DELETE_CHARACTER_BUTTON_CUSTOM_ID).label(tr_locale!(modal_response.interaction.locale.as_str(), DELETE_CHARACTER_BUTTON_CUSTOM_ID)).style(ButtonStyle::Danger),
            ]
        )
    ];

    let interaction = modal_response.interaction.clone();

    let result_message = match modal_response.interaction.message {
        None => {
            modal_response.interaction.channel_id.send_message(ctx, CreateMessage::new().embed(
                CreateEmbed::new()
                    .footer(CreateEmbedFooter::new(modal_response.interaction.user.id.get().to_string()))
                    .title(inputs[0].clone())
                    .field(tr_locale!(interaction.locale.as_str(), CHARACTER_DESCRIPTION), inputs[1].clone(), true)
                    .field(tr_locale!(interaction.locale.as_str(), CHARACTER_STORY), inputs[2].clone(), true)
                    .field(tr_locale!(interaction.locale.as_str(), CHARACTER_SPECIAL_REQUEST), inputs[3].clone(), false)
                    .author(CreateEmbedAuthor::new(component_interaction.user.name.as_str()))
                    .color(Color::from_rgb(112, 190, 255))
            )
                .components(buttons)
            ).await}
        Some(message) => {
            let embed_fields = vec![
                (
                    tr_locale!(interaction.locale.as_str(), CHARACTER_DESCRIPTION),
                    inputs[1].clone(),
                    true
                ),
                (
                    tr_locale!(interaction.locale.as_str(), CHARACTER_STORY),
                    inputs[2].clone(),
                    true
                ),
                (
                    tr_locale!(interaction.locale.as_str(), CHARACTER_SPECIAL_REQUEST),
                    inputs[3].clone(),
                    false
                )
            ];
            modal_response.interaction.channel_id.edit_message(ctx, message.id , EditMessage::new().embed(
                CreateEmbed::new()
                    .footer(CreateEmbedFooter::new(message.embeds.get(0).unwrap().footer.clone().unwrap().text.as_str()))
                    .title(inputs[0].clone())
                    .fields(embed_fields)
                    .author(CreateEmbedAuthor::new( component_interaction.user.name.as_str()))
                    .color(Color::from_rgb(112, 190, 255))
            )
                .components(buttons)
            ).await
        }
    };

    match result_message {
        Ok(_) => {
            Ok("create_character__submited")
        }
        Err(_) => { Err("create_place__character_too_long".into()) }
    }
 }
pub async fn refuse_character(ctx: SerenityContext, component_interaction: ComponentInteraction) -> Result<&'static str, Error> {
    let member = component_interaction.member.as_ref().unwrap();

    let guild_id = component_interaction.guild_id.unwrap();
    let server = get_server_by_id(guild_id.get()).await;
    let server = match server {
        Ok(result) => {
            match result {
                None => { return Err("create_character__no_universe_found".into()) }
                Some(server) => { server }
            }
        }
        Err(_) => { return Err("create_character__database_error".into()) }
    };

    let has_admin_permission = member.permissions.map_or(false, |p| p.contains(Permissions::ADMINISTRATOR));
    let has_moderator_role = server.moderator_role_id.map_or(false, |role| member.roles.contains(&role.id.into()));
    let has_admin_role = server.admin_role_id.map_or(false, |role| member.roles.contains(&role.id.into()));

    if !has_admin_permission && !has_moderator_role && !has_admin_role {
        let _ = component_interaction.create_response(ctx.clone(), CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(tr_locale!(component_interaction.locale.as_str(), "create_character__no_permission")).ephemeral(true)
        )).await?;
        return Err("create_character__no_permission".into());
    }

    let modal = CreateQuickModal::new(tr_locale!(component_interaction.locale.as_str(), CHARACTER_MODAL_TITLE))
        .field(
            CreateInputText::new(InputTextStyle::Paragraph, tr_locale!(component_interaction.locale.as_str(), CHARACTER_REJECT_REASON), CHARACTER_REJECT_REASON)
                .required(false).max_length(864)
        )
        .timeout(Duration::from_secs(30));

    let interaction = component_interaction.quick_modal(&ctx, modal).await?;
    let modal_response = match interaction {
        Some(interaction) => {
            interaction.interaction.create_response(ctx.clone(), CreateInteractionResponse::Acknowledge).await?;
            interaction }
        None => {
            return Err("create_character__timed_out".into()) }
    };

    let inputs = modal_response.inputs.clone();

    let buttons = vec![
        CreateActionRow::Buttons(
            vec![
                CreateButton::new(SUBMIT_CHARACTER_BUTTON_CUSTOM_ID).label(tr_locale!(modal_response.interaction.locale.as_str(), SUBMIT_CHARACTER_BUTTON_CUSTOM_ID)).style(ButtonStyle::Success),
                CreateButton::new(MODIFY_CHARACTER_BUTTON_CUSTOM_ID).label(tr_locale!(modal_response.interaction.locale.as_str(), MODIFY_CHARACTER_BUTTON_CUSTOM_ID)).style(ButtonStyle::Primary),
                CreateButton::new(DELETE_CHARACTER_BUTTON_CUSTOM_ID).label(tr_locale!(modal_response.interaction.locale.as_str(), DELETE_CHARACTER_BUTTON_CUSTOM_ID)).style(ButtonStyle::Danger),
            ]
        )
    ];

    let interaction = modal_response.interaction.clone();

    let result_message = if let Some(message) = modal_response.interaction.message {
        let mut embed_fields = component_interaction.message.embeds[0].clone().fields;
        embed_fields.push(EmbedField::new(tr_locale!(component_interaction.locale.as_str(), CHARACTER_REJECT_REASON), inputs[0].clone(), false));

        let embed_fields: Vec<(String, String, bool)> = embed_fields
            .iter()
            .map(|field| (field.name.clone(), field.value.clone(), field.inline))
            .collect();

        let mess = modal_response.interaction.channel_id.edit_message(ctx, message.id, EditMessage::new().embed(
            CreateEmbed::new()
                .footer(CreateEmbedFooter::new(message.embeds.get(0).unwrap().footer.clone().unwrap().text.as_str()))
                .title(inputs[0].clone())
                .fields(embed_fields)
                .author(CreateEmbedAuthor::new(component_interaction.user.name.as_str()))
                .color(Color::from_rgb(255, 0, 0))
        )
            .components(buttons)
        ).await;
        mess
    }
        else{return Err("create_character__message_not_found".into())};

    match result_message {
        Ok(_) => {
            Ok("create_character__submited")
        }
        Err(_) => { Err("create_place__character_too_long".into()) }
    }
}
pub async fn accept_character(ctx: SerenityContext, component_interaction: ComponentInteraction) -> Result<&'static str, Error> {
    let member = component_interaction.member.as_ref().unwrap();

    let guild_id = component_interaction.guild_id.unwrap();
    let server = get_server_by_id(guild_id.get()).await;
    let server = match server {
        Ok(result) => {
            match result {
                None => { return Err("create_character__no_universe_found".into()) }
                Some(server) => { server }
            }
        }
        Err(_) => { return Err("create_character__database_error".into()) }
    };

    let has_admin_permission = member.permissions.map_or(false, |p| p.contains(Permissions::ADMINISTRATOR));
    let has_moderator_role = server.moderator_role_id.map_or(false, |role| member.roles.contains(&role.id.into()));
    let has_admin_role = server.admin_role_id.map_or(false, |role| member.roles.contains(&role.id.into()));

    if !has_admin_permission && !has_moderator_role && !has_admin_role {
        let _ = component_interaction.create_response(ctx.clone(), CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(tr_locale!(component_interaction.locale.as_str(), "create_character__no_permission")).ephemeral(true)
        )).await?;
        return Err("create_character__no_permission".into());
    }

    let universe = get_universe_by_id(server.universe_id.to_string()).await?;
    let stats_cursor = universe.unwrap().get_stats().await?;
    let stats: Vec<Stat> = stats_cursor.try_collect().await.unwrap();

    let mut quick_modal = CreateQuickModal::new(tr_locale!(component_interaction.locale.as_str(), CHARACTER_MODAL_TITLE));

    let mut text = "".to_string();
    for stat in stats.clone() {
        text += (stat.name.to_string() + ": [".into() + format!("{:?}", stat.base_value).as_str() + "]\n".into()).as_str();
    };
    quick_modal = quick_modal.field(CreateInputText::new(InputTextStyle::Paragraph, "test", "test").value(text).required(true));

    let interaction = component_interaction.quick_modal(&ctx, quick_modal).await?;

    let mut extracted_stats: Vec<Stat> = Vec::new();

    match interaction {
        Some(interaction) => {
            for stat in stats.iter() {
                let input = interaction.inputs[0].clone();

                let mut line_matched = std::collections::HashSet::new();
                for line in input.lines() {
                    for stat in stats.iter() {
                        if line.contains(&stat.name) {
                            line_matched.insert(stat.name.clone());
                            if let Some(colon_pos) = line.find(':') {
                                let value_str = line[colon_pos + 1..].trim();
                                let parsed_value = match &stat.base_value {
                                    StatValue::I64(_) => {
                                        value_str.parse::<i64>().ok().map(StatValue::I64)
                                    }
                                    StatValue::F64(_) => {
                                        value_str.parse::<f64>().ok().map(StatValue::F64)
                                    }
                                    StatValue::String(_) => {
                                        Some(StatValue::String(value_str.to_string()))
                                    }
                                    StatValue::Bool(_) => {
                                        value_str.parse::<bool>().ok().map(StatValue::Bool)
                                    }
                                };
                                if let Some(value) = parsed_value {
                                    let stat_with_value = Stat {
                                        _id: Default::default(),
                                        universe_id: Default::default(),
                                        name: stat.name.clone(),
                                        base_value: value,
                                        formula: stat.formula.clone(),
                                        min: stat.min.clone(),
                                        max: stat.max.clone(),
                                        modifiers: vec![],
                                    };
                                    extracted_stats.push(stat_with_value);
                                } else {
                                    interaction.interaction.create_response(ctx.clone(), CreateInteractionResponse::Message(
                                        CreateInteractionResponseMessage::new()
                                            .embed(CreateEmbed::new()
                                                .color(Color::from_rgb(255, 0, 0))
                                                .title(tr_locale!(component_interaction.locale.as_str(), "create_character__validation_error"))
                                                .description(format!("{}: {}", stat.name, tr_locale!(component_interaction.locale.as_str(), "create_character__type_mismatch")))
                                            ).ephemeral(true)
                                    )).await?;
                                    return Err("create_character__type_mismatch".into());
                                }
                            } else {
                                let parsed_value = match &stat.base_value {
                                    StatValue::I64(_) => {
                                        line.trim().parse::<i64>().ok().map(StatValue::I64)
                                    }
                                    StatValue::F64(_) => {
                                        line.trim().parse::<f64>().ok().map(StatValue::F64)
                                    }
                                    StatValue::String(_) => {
                                        Some(StatValue::String(line.trim().to_string()))
                                    }
                                    StatValue::Bool(_) => {
                                        line.trim().parse::<bool>().ok().map(StatValue::Bool)
                                    }
                                };
                                if let Some(value) = parsed_value {
                                    let stat_with_value = Stat {
                                        _id: Default::default(),
                                        universe_id: Default::default(),
                                        name: stat.name.clone(),
                                        base_value: value,
                                        formula: stat.formula.clone(),
                                        min: stat.min.clone(),
                                        max: stat.max.clone(),
                                        modifiers: vec![],
                                    };
                                    extracted_stats.push(stat_with_value);
                                } else {
                                    interaction.interaction.create_response(ctx.clone(), CreateInteractionResponse::Message(
                                        CreateInteractionResponseMessage::new()
                                            .embed(CreateEmbed::new()
                                                .color(Color::from_rgb(255, 0, 0))
                                                .title(tr_locale!(component_interaction.locale.as_str(), "create_character__validation_error"))
                                                .description(format!("{}: {}", stat.name, tr_locale!(component_interaction.locale.as_str(), "create_character__type_mismatch")))
                                            ).ephemeral(true)
                                    )).await?;
                                    return Err("create_character__type_mismatch".into());
                                }
                            }
                            break;
                        }
                    }
                }
                for stat in stats.iter() {
                    if !line_matched.contains(&stat.name) {
                        extracted_stats.push(stat.clone());
                    }
                }
            }
            interaction.interaction.create_response(ctx.clone(), CreateInteractionResponse::Acknowledge).await?;
        }
        None => {
            return Err("create_character__timed_out".into());
        }
    }


    let character_user_id = component_interaction.message.embeds[0]
        .footer.as_ref()
        .unwrap()
        .text.parse::<u64>()
        .unwrap();

    let character_name = component_interaction.message.embeds[0]
        .title.as_ref()
        .unwrap()
        .clone();

    let embed_fields = &component_interaction.message.embeds[0].fields;
    let description = embed_fields.iter()
        .find(|f| f.name == tr_locale!(component_interaction.locale.as_str(), CHARACTER_DESCRIPTION))
        .map(|f| f.value.clone())
        .unwrap_or_default();

    let story = embed_fields.iter()
        .find(|f| f.name == tr_locale!(component_interaction.locale.as_str(), CHARACTER_STORY))
        .map(|f| f.value.clone())
        .unwrap_or_default();

    let special_request = embed_fields.iter()
        .find(|f| f.name == tr_locale!(component_interaction.locale.as_str(), CHARACTER_SPECIAL_REQUEST))
        .map(|f| f.value.clone())
        .unwrap_or_default();



    let character = Character {
        _id: Default::default(), // Assuming `_id` is an `Option<T>` or an equivalent type
        user_id: character_user_id,
        universe_id: server.universe_id,
        name: character_name,
        stats: extracted_stats, // Assuming `extracted_stats` matches the `stats` field's type
    };

    character.update().await?;

    if let Some(player_role_id) = server.player_role_id {
        if let Ok(mut member) = ctx.http().get_member(guild_id, character_user_id.into()).await {
            let _ = member.add_role(&ctx.http(), player_role_id.id).await;
        }
    }

    let message = component_interaction.message.clone();
    let original_embed: CreateEmbed = message.embeds[0].clone().into();
    let _ = component_interaction.channel_id.edit_message(
        ctx,
        message.id,
        EditMessage::new().components(vec![]).embed(
            original_embed.color(Color::from_rgb(0, 0, 255))
        ),
    ).await;

    Ok("accept_character")
}
