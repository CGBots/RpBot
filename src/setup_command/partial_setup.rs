use log::{log, Level};
use serenity::all::{ChannelType, GuildChannel, Role, RoleId};
use crate::database::server::{Id, Server};
use crate::database::server::IdType::Category;
use crate::discord::channels::{create_channel, get_road_category_permission_set};
use crate::discord::poise_structs::Context;
use crate::discord::roles::{create_role, edit_role_positions, AdminRolePermissions, ModeratorRolePermissions, PlayerRolePermissions, SpectatorRolePermissions};
use crate::tr;

pub async fn partial_setup<'a>(ctx : Context<'_>, server: &mut Server) -> Result<(&'a str, Vec<Role>, Vec<GuildChannel>), Vec<&'a str>> {

    //everyone role
    let everyone_role = ctx.guild_id().unwrap().everyone_role();

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