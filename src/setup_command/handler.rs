//TODO
// Vérifier que l'utilisateur est le créateur de l'univers.
// Si l'utilisateur est bien le créateur effectuer le setup en fonction du mode choisis.
// Mode partiel:  DONE
//  . admin_role_id, DONE
//  . moderator_role_id, DONE
//  . spectator_role_id, DONE
//  . player_role_id, DONE
//  . get everyone_role_id, DONE
//  . road_category_id, DONE
// .
// Mode Complet:
//  + admin_category_id,
//    + Moderation
//    + Commandes
//    + Logs
//    + Discussions
//  + nrp_category_id,
//  + rp_category_id,
//    + character_channel_id
//    + wiki_forum_id,

use log::{log, Level};
use poise::{CreateReply};
use serenity::all::{ButtonStyle, Channel, ChannelType, ComponentInteractionCollector, CreateActionRow, CreateButton, CreateEmbed, Role, RoleId};
use serenity::model::Colour;
use tokio::join;
use crate::discord::poise_structs::{Context, Error};
use crate::discord::roles::{create_role, edit_role_positions, AdminRolePermissions, ModeratorRolePermissions};
use crate::tr;
use crate::database::server::{Id, Server};
use crate::database::server::IdType::{Category, Role as IdTypeRole};
use crate::database::universe::Universe;
use crate::discord::channels::{create_channel, get_road_category_permission_set};

#[derive(Debug, poise::ChoiceParameter)]
pub enum SetupType {
    FullSetup,
    PartialSetup
}

#[poise::command(slash_command, required_permissions = "ADMINISTRATOR", guild_only)]
pub async fn setup(ctx: Context<'_>, setup_type: SetupType) -> Result<(), Error> {
    ctx.defer().await.unwrap();
    
    let result = match setup_type {
        SetupType::FullSetup => { full_setup(ctx).await }
        SetupType::PartialSetup => { partial_setup(ctx).await }
    };

    match result{
        Ok(result) => {
            let message = ctx.reply_builder(
                CreateReply::default()
                    .embed(CreateEmbed::new()
                        .color(Colour::from_rgb(0, 255, 0))
                        .description(tr!(ctx, result))
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
                        .title(tr!(ctx, "setup__error_title"))
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

async fn partial_setup(ctx : Context<'_>) -> Result<&str, Vec<&str>> {
    
    //everyone role
    let guild_id = ctx.guild_id().unwrap();
    let everyone_role = ctx.guild_id().unwrap().everyone_role();

    //server
    let universe_result = Universe::get_universe_by_server_id(guild_id.get()).await;

    let universe_id = match universe_result {
        Ok(cursor) => {
            match cursor{
                None => {return Err(vec!["setup__universe_not_found"])}
                Some(universe) => {universe.universe_id.to_string()}
            }
        }
        Err(_) => {return Err(vec!["setup__universe_not_found"])}
    };

    let server = Server::get_server_by_id(universe_id, guild_id.get().to_string()).await;

    let mut server = if server.is_err() || server.clone().unwrap().is_none(){
        return Err(vec!["setup__server_not_found"]) }
        else {server.unwrap().unwrap()};

    if server.admin_role_id.id.is_some()
        || server.moderator_role_id.id.is_some()
        || server.spectator_role_id.id.is_some()
        || server.player_role_id.id.is_some()
        || server.road_category_id.id.is_some() {

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
                return Err(vec!["setup__server_already_setup_timeout"])}
            Some(mci) => {
                message.delete(ctx).await.unwrap();
                let interaction_button_id = mci.data.custom_id.as_str();
                match interaction_button_id {
                    "cancel" => {return Ok("setup__canceled")}
                    _ => {}
                }
            }
        }
    }

    let mut roles_created: Vec<Role> = vec![];
    let mut errors: Vec<&str> = vec![];

    //Ne récréé pas ce qui existe déjà
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
                    match create_role(ctx, tr!(ctx, "moderator_role_name"), *AdminRolePermissions).await {
                        Ok(role) => {roles_created.push(role.clone()); Ok(role)}
                        Err(e) => {errors.push("setup__moderator_role_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    let spectator_role = match server.spectator_role_id.id{
        None => {
            match create_role(ctx, tr!(ctx, "spectator_role_name"), *ModeratorRolePermissions).await {
                Ok(role) => {roles_created.push(role.clone()); Ok(role)}
                Err(e) => {errors.push("setup__spectator_role_not_created"); Err(e)}
            }
        }
        Some(role_id) => {
            match ctx.http().get_guild_role(server.server_id.into(), role_id.into()).await{
                Ok(role) => {Ok(role)}
                Err(_) => {
                    match create_role(ctx, tr!(ctx, "spectator_role_name"), *AdminRolePermissions).await {
                        Ok(role) => {roles_created.push(role.clone()); Ok(role)}
                        Err(e) => {errors.push("setup__spectator_role_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    let player_role = match server.player_role_id.id{
        None => {
            match create_role(ctx, tr!(ctx, "player_role_name"), *ModeratorRolePermissions).await {
                Ok(role) => {roles_created.push(role.clone()); Ok(role)}
                Err(e) => {errors.push("setup__player_role_not_created"); Err(e)}
            }
        }
        Some(role_id) => {
            match ctx.http().get_guild_role(server.server_id.into(), role_id.into()).await{
                Ok(role) => {Ok(role)}
                Err(_) => {
                    match create_role(ctx, tr!(ctx, "player_role_name"), *AdminRolePermissions).await {
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

    let result_road_category = match server.road_category_id.id{
        None => {Err(create_channel(ctx, tr!(ctx, "road_channel_name"), ChannelType::Category, 0, permissions, None).await)}
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.into()).await{
                Ok(channel) => {Ok(channel)}
                Err(_) => {
                    Err(create_channel(ctx, tr!(ctx, "road_channel_name"), ChannelType::Category, 0, permissions, None).await)}
            }
        }
    };

    let mut road_created = false;

    let road_category = match result_road_category {
        Ok(channel) => {channel}
        Err(new_channel_result) => {
            match new_channel_result {
                Ok(channel) => {road_created = true; Channel::Guild(channel)}
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
    
    server.admin_role_id = Id{ id: Some(admin_role.id.get()), id_type: Some(IdTypeRole) };
    server.moderator_role_id = Id{ id: Some(moderator_role.id.get()), id_type: Some(IdTypeRole) };
    server.spectator_role_id = Id{ id: Some(spectator_role.id.get()), id_type: Some(IdTypeRole) };
    server.player_role_id = Id{ id: Some(player_role.id.get()), id_type: Some(IdTypeRole) };
    server.everyone_role_id = Id{ id: Some(everyone_role.get()), id_type: Some(IdTypeRole) };
    server.road_category_id = Id{ id: Some(road_category.id().get()), id_type: Some(Category) };

    match server.update().await {
        Ok(_) => {}
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
            if road_created{
                match road_category.delete(ctx).await {
                    Ok(_) => {}
                    Err(_) => {
                        log!(Level::Error, "Error during setup and rollback.\
                            universe_id: {}\
                            server_id: {}\
                            category_id: {}", server.universe_id, server.server_id, road_category.id());
                    }
                };
            }

            return Err(vec!["setup__server_update_failed"])}
    };

    Ok("setup__setup_success_message")
}
async fn complementary_setup() {
    
}
async fn full_setup(ctx: Context<'_>) -> Result<&str, Vec<&str>> {
    let partial_setup = partial_setup(ctx);
    let complementary_setup = complementary_setup();
    let _res = join!(partial_setup, complementary_setup);
    Ok("")
}