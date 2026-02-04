use log::{log, Level};
use serenity::all::{ChannelType, GuildChannel, Role};
use crate::database::server::{Id, IdType, Server};
use crate::discord::channels::{create_channel, get_admin_category_permission_set, get_rp_character_permission_set};
use crate::discord::poise_structs::Context;
use crate::tr;

pub async fn complementary_setup<'a>(ctx: Context<'_>, server : &'a mut Server) -> Result<(&'a str, Vec<Role>, Vec<GuildChannel>), Vec<&'a str>> {
    let mut created_categories: Vec<GuildChannel> = vec![];
    let mut errors: Vec<&str> = vec![];


    //Créer les catégories: admin, nrp, rp
    let admin_category_permissions = get_admin_category_permission_set(server.everyone_role_id.id.unwrap().into(), server.spectator_role_id.id.unwrap().into(), server.player_role_id.id.unwrap().into(), server.moderator_role_id.id.unwrap().into());
    let admin_category_result = match server.admin_category_id.id{
        None => {
            match create_channel(ctx, tr!(ctx, "admin_category_name"), ChannelType::Category, 0, admin_category_permissions, None).await {
                Ok(category) => { created_categories.push(category.clone()); Ok(category)}
                Err(e) => {errors.push("setup__admin_category_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.into()).await{
                Ok(channel) => {Ok(channel.guild().unwrap())}
                Err(_) => {
                    match create_channel(ctx, tr!(ctx, "admin_category_name"), ChannelType::Category, 0, admin_category_permissions, None).await {
                        Ok(category) => { created_categories.push(category.clone()); Ok(category)}
                        Err(e) => {errors.push("setup__admin_category_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    let nrp_category_result = match server.nrp_category_id.id{
        None => {
            match create_channel(ctx, tr!(ctx, "nrp_category_name"), ChannelType::Category, 1, vec![], None).await {
                Ok(category) => {created_categories.push(category.clone()); Ok(category)}
                Err(e) => {errors.push("setup__nrp_category_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.into()).await{
                Ok(channel) => {Ok(channel.guild().unwrap())}
                Err(_) => {
                    match create_channel(ctx, tr!(ctx, "nrp_category_name"), ChannelType::Category, 1, vec![], None).await {
                        Ok(category) => {created_categories.push(category.clone()); Ok(category)}
                        Err(e) => {errors.push("setup__nrp_category_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    let rp_category_result = match server.rp_category_id.id{
        None => {
            match create_channel(ctx, tr!(ctx, "rp_category_name"), ChannelType::Category, 1, vec![], None).await {
                Ok(category) => {created_categories.push(category.clone()); Ok(category)}
                Err(e) => {errors.push("setup__rp_category_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.into()).await{
                Ok(channel) => {Ok(channel.guild().unwrap())}
                Err(_) => {
                    match create_channel(ctx, tr!(ctx, "rp_category_name"), ChannelType::Category, 1, vec![], None).await {
                        Ok(category) => {created_categories.push(category.clone()); Ok(category)}
                        Err(e) => {errors.push("setup__rp_category_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    if !errors.is_empty() {
        for channel in created_categories{
            match channel.clone().delete(ctx).await {
                Ok(_) => {}
                Err(_) => {
                    log!(Level::Error, "Error while setuping and rollbacking.\
                     universe_id: {}\
                     server_id: {}\
                     category_id: {}", server.universe_id, server.server_id, channel.id);
                    return Err(vec!["setup__rollback_failed"])
                }
            }
        }
        return Err(errors)
    }

    let admin_category = admin_category_result.unwrap();
    let nrp_category = nrp_category_result.unwrap();
    let rp_category = rp_category_result.unwrap();

    let mut created_channels = vec![];
    let mut errors = vec![];

    let log_channel_result = match server.log_channel_id.id{
        None => {
            let result = create_channel(ctx, tr!(ctx, "log_channel_name"), ChannelType::Text, 0, vec![], Some(admin_category.clone().id.get())).await;
            match result {
                Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                Err(e) => { errors.push("setup__log_channel_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.into()).await{
                Ok(channel) => {created_channels.push(channel.clone().guild().unwrap().clone()); Ok(channel.guild().unwrap())}
                Err(_) => {
                    let result = create_channel(ctx, tr!(ctx, "log_channel_name"), ChannelType::Text, 0, vec![], Some(admin_category.clone().id.get())).await;
                    match result{
                        Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                        Err(e) => { errors.push("setup__log_channel_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    let commands_channel_result = match server.commands_channel_id.id{
        None => {
            let result = create_channel(ctx, tr!(ctx, "commands_channel_name"), ChannelType::Text, 0, vec![], Some(admin_category.clone().id.get())).await;
            match result {
                Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                Err(e) => {errors.push("setup__commands_channel_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.into()).await{
                Ok(channel) => {created_channels.push(channel.clone().guild().unwrap().clone()); Ok(channel.guild().unwrap())}
                Err(_) => {
                    let result = create_channel(ctx, tr!(ctx, "commands_channel_name"), ChannelType::Text, 0, vec![], Some(admin_category.clone().id.get())).await;
                    match result{
                        Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                        Err(e) => {errors.push("setup__commands_channel_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    let moderation_channel_result = match server.moderation_channel_id.id{
        None => {
            let result = create_channel(ctx, tr!(ctx, "moderation_channel_name"), ChannelType::Text, 0, vec![], Some(admin_category.clone().id.get())).await;
            match result {
                Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                Err(e) => {errors.push("setup__moderation_channel_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.into()).await{
                Ok(channel) => {created_channels.push(channel.clone().guild().unwrap().clone()); Ok(channel.guild().unwrap())}
                Err(_) => {
                    let result = create_channel(ctx, tr!(ctx, "moderation_channel_name"), ChannelType::Text, 0, vec![], Some(admin_category.clone().id.get())).await;
                    match result{
                        Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                        Err(e) => {errors.push("setup__moderation_channel_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    let nrp_general_channel_result = match server.nrp_general_channel_id.id{
        None => {
            let result = create_channel(ctx, tr!(ctx, "nrp_general_channel_name"), ChannelType::Text, 0, vec![], Some(nrp_category.clone().id.get())).await;
            match result {
                Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                Err(e) => {errors.push("setup__nrp_general_channel_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.into()).await{
                Ok(channel) => {created_channels.push(channel.clone().guild().unwrap().clone()); Ok(channel.guild().unwrap())}
                Err(_) => {
                    let result = create_channel(ctx, tr!(ctx, "nrp_general_channel_name"), ChannelType::Text, 0, vec![], Some(nrp_category.clone().id.get())).await;
                    match result{
                        Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                        Err(e) => {errors.push("setup__nrp_general_channel_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    let character_channel_permissions = get_rp_character_permission_set(server.player_role_id.id.unwrap().into());

    let rp_character_channel = match server.rp_character_channel_id.id{
        None => {
            let result = create_channel(ctx, tr!(ctx, "rp_character_channel_name"), ChannelType::Text, 0, character_channel_permissions, Some(rp_category.clone().id.get())).await;
            match result {
                Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                Err(e) => {errors.push("setup__rp_character_channel_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.into()).await{
                Ok(channel) => {created_channels.push(channel.clone().guild().unwrap().clone()); Ok(channel.guild().unwrap())}
                Err(_) => {
                    let result = create_channel(ctx, tr!(ctx, "rp_character_channel_name"), ChannelType::Text, 0, character_channel_permissions, Some(rp_category.clone().id.get())).await;
                    match result{
                        Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                        Err(e) => {errors.push("setup__rp_character_channel_not_created"); Err(e)}
                    }
                }
            }
        }
    };


    let wiki_channel_result = match server.rp_wiki_channel_id.id{
        None => {
            let result = create_channel(ctx, tr!(ctx, "rp_wiki_channel_name"), ChannelType::Forum, 0, vec![], Some(rp_category.clone().id.get())).await;
            match result {
                Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                Err(e) => {errors.push("setup__wiki_channel_not_created"); Err(e)}
            }
        }
        Some(channel_id) => {
            match ctx.http().get_channel(channel_id.into()).await{
                Ok(channel) => {created_channels.push(channel.clone().guild().unwrap().clone()); Ok(channel.guild().unwrap())}
                Err(_) => {
                    let result = create_channel(ctx, tr!(ctx, "rp_wiki_channel_name"), ChannelType::Forum, 0, vec![], Some(rp_category.clone().id.get())).await;
                    match result{
                        Ok(channel) => {created_channels.push(channel.clone()); Ok(channel)}
                        Err(e) => {errors.push("setup__wiki_channel_not_created"); Err(e)}
                    }
                }
            }
        }
    };

    if !errors.is_empty() {
        for channel in created_channels{
            match channel.clone().delete(ctx).await {
                Ok(_) => {}
                Err(_) => {
                    log!(Level::Error, "Error while setuping and rollbacking.\
                     universe_id: {}\
                     server_id: {}\
                     category_id: {}", server.universe_id, server.server_id, channel.id);
                    return Err(vec!["setup__rollback_failed"])
                }
            }
        }
        return Err(errors)
    }

    let log_channel = log_channel_result.unwrap();
    let commands_channel = commands_channel_result.unwrap();
    let moderation_channel = moderation_channel_result.unwrap();
    let nrp_general_channel = nrp_general_channel_result.unwrap();
    let rp_character_channel = rp_character_channel.unwrap();
    let wiki_channel = wiki_channel_result.unwrap();



    server.nrp_category_id = Id{ id: Some(nrp_category.id.get()), id_type: Some(IdType::Category) };
    server.rp_category_id = Id{ id: Some(rp_category.id.get()), id_type: Some(IdType::Category) };
    server.admin_category_id = Id{ id: Some(admin_category.id.get()), id_type: Some(IdType::Category) };
    server.log_channel_id = Id{id: Some(log_channel.id.get()), id_type: Some(IdType::Channel)};
    server.commands_channel_id = Id{id: Some(commands_channel.id.get()), id_type: Some(IdType::Channel)};
    server.moderation_channel_id = Id{id: Some(moderation_channel.id.get()), id_type: Some(IdType::Channel)};
    server.nrp_general_channel_id = Id{id: Some(nrp_general_channel.id.get()), id_type: Some(IdType::Channel)};
    server.rp_character_channel_id = Id{id: Some(rp_character_channel.id.get()), id_type: Some(IdType::Channel)};
    server.rp_wiki_channel_id = Id{id: Some(wiki_channel.id.get()), id_type: Some(IdType::Channel)};

    let mut channel_order = vec![(admin_category.id, 0), (nrp_category.id, 1), (rp_category.id, 2), (server.road_category_id.id.unwrap().into(), 3)];
    let channels = ctx.guild_id().unwrap().channels(ctx).await.unwrap();

    let allowed = [
        admin_category.id.get(),
        nrp_category.id.get(),
        rp_category.id.get(),
        server.road_category_id.id.unwrap().into()
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
            for channel in created_channels{
                match channel.clone().delete(ctx).await {
                    Ok(_) => {}
                    Err(_) => {
                        log!(Level::Error, "Error while setuping and rollbacking.\
                     universe_id: {}\
                     server_id: {}\
                     channel_id: {}", server.universe_id, server.server_id, channel.id);
                        return Err(vec!["setup__rollback_failed"])
                    }
                }
            }
            for category in created_categories{
                match category.clone().delete(ctx).await {
                    Ok(_) => {}
                    Err(_) => {
                        log!(Level::Error, "Error while setuping and rollbacking.\
                     universe_id: {}\
                     server_id: {}\
                     category_id: {}", server.universe_id, server.server_id, category.id);
                        return Err(vec!["setup__rollback_failed"])
                    }
                }
            }
            return Err(vec!["setup__server_update_failed"])}
    };

    Ok(("setup__setup_success_message", vec![], vec![]))
    //Err((vec!["placeholder"], created_categories))
}