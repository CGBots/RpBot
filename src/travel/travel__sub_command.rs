use std::sync::mpsc::channel;
use poise::serenity_prelude::Context as SerenityContext;
use serenity::all::{GuildChannel, ChannelId, CreateMessage, CreateActionRow, EditMessage, Color, CreateSelectMenuOption, ComponentInteraction, Interaction};
use crate::database::places::{get_place_by_category_id, Place};
use crate::database::server::{get_server_by_id, Server};
use crate::database::travel::{PlayerMove, SpaceType};
use crate::database::characters::Character;
use crate::discord::poise_structs::{Context, Error};
use crate::travel::logic::add_travel;
use crate::utility::reply::{reply, reply_with_args};
use fluent::FluentArgs;
use fluent::types::AnyEq;
use futures::{StreamExt, TryStreamExt};
use poise::{CreateReply, ReplyHandle};
use crate::characters::create_character_sub_command::ACCEPT_CHARACTER_CHOOSE_PLACE;
use crate::database::road::{get_road_by_source, Road};
use crate::tr_locale;

fn parse_channel_id(input: &str) -> Option<u64> {
    if let Ok(id) = input.parse::<u64>() {
        return Some(id);
    }
    if input.starts_with("<#") && input.ends_with('>') {
        if let Ok(id) = input[2..input.len() - 1].parse::<u64>() {
            return Some(id);
        }
    }
    None
}

#[poise::command(slash_command, guild_only)]
pub async fn travel(
    ctx: Context<'_>,
    destination: Option<String>,
) -> Result<(), Error> {
    let Ok(_) = ctx.defer_ephemeral().await else { return Err("reply__reply_failed".into()) };
    match destination {
        None => {travel_without_destination(ctx).await?}
        Some(dest) => {
            let error = match _travel(ctx, dest).await {
                Ok(_) => return Ok(()),
                Err(e) => reply(ctx, Err(e)).await,
            };}
    }


    Ok(())
}

pub async fn _travel(ctx: Context<'_>, destination_input: String) -> Result<(), Error>{
    let server = match get_server_by_id(ctx.guild_id().unwrap().get()).await {
        Ok(Some(s)) => s,
        Ok(None) => return Err("travel__server_not_found".into()),
        Err(e) => {
            log::error!("Database error in _travel when fetching server: {:?}", e);
            return Err("travel__database_error".into());
        }
    };

    let destination_category_id = parse_channel_id(&destination_input).ok_or_else(|| Error::from("travel__place_not_found"))?;

    println!("travel__place_category_id: {:?}", destination_category_id);

    let destination_place = match get_place_by_category_id(server.universe_id, destination_category_id).await {
        Ok(Some(p)) => p,
        _ => return Err("travel__place_not_found".into()),
    };


    let _character = match server.clone().get_character_by_user_id(ctx.author().id.get()).await {
        Ok(Some(c)) => c,
        _ => return Err("travel__character_not_found".into()),
    };

    let mut player_move = match server.clone().get_player_move(ctx.author().id.get()).await {
        Ok(Some(m)) => m,
        _ => {
            // Initialise un nouveau PlayerMove si inexistant
            PlayerMove {
                universe_id: server.universe_id,
                user_id: ctx.author().id.get(),
                server_id: server.server_id,
                actual_space_id: ctx.channel_id().get(),
                actual_space_type: SpaceType::Place, // Par défaut, on suppose qu'il est dans un lieu
                ..Default::default()
            }
        }
    };

    // On s'assure que le mouvement est bien dans l'univers actuel
    if player_move.universe_id != server.universe_id {
        // Le joueur change d'univers (rare mais possible via admin)
        // On supprime l'ancien mouvement dans l'autre univers
        let _ = player_move.remove().await;
        player_move.universe_id = server.universe_id;
    }

    match player_move.actual_space_type {
        SpaceType::Road => {
            move_from_road(ctx.serenity_context(), destination_place.category_id, server, player_move.clone()).await?;
        }
        SpaceType::Place => {
            move_from_place(ctx.serenity_context(), ctx.channel_id().get(), destination_place.category_id, server, player_move.clone()).await?;
        }
    }

    Ok(())

    //Ok(("travel__started", destination_mention))
}

async fn move_from_road(_ctx: &SerenityContext, destination_id: u64, server: Server, mut player_move: PlayerMove) -> Result<&'static str, Error>{
    let dest_id = destination_id;
    
    // Si on est sur une route, on ne peut aller que vers les extrémités (source ou destination originelle)
    if Some(dest_id) == player_move.destination_id {
        // Déjà en train d'y aller ? On ne fait rien ou on confirme
        return Ok("travel__already_moving_to_destination");
    } else if Some(dest_id) == player_move.source_id {
        // Demi-tour
        let old_dest = player_move.destination_id;
        let old_dest_role = player_move.destination_role_id;
        let old_dest_server = player_move.destination_server_id;
        
        player_move.destination_id = player_move.source_id;
        player_move.destination_role_id = player_move.source_role_id;
        player_move.destination_server_id = player_move.source_server_id;
        
        player_move.source_id = old_dest;
        player_move.source_role_id = old_dest_role;
        player_move.source_server_id = old_dest_server;
        
        // On recalcule la distance parcourue (on repart dans l'autre sens)
        // Pour simplifier, on inverse juste la progression
        if let Ok(Some(road)) = crate::database::road::get_road_by_channel_id(server.universe_id, player_move.road_id.unwrap()).await {
            player_move.distance_traveled = (road.distance as f64 - player_move.distance_traveled).max(0.0);
        }

        add_travel(_ctx.http.clone(), server.server_id.into(), player_move.clone()).await?;

    } else {
        return Err("travel__invalid_road_destination".into());
    }
    Ok("travel__started")
}


async fn travel_without_destination(ctx: Context<'_>) -> Result<(), Error>{
    let server = match get_server_by_id(ctx.guild_id().unwrap().get()).await {
        Ok(Some(s)) => s,
        Ok(None) => return Err("travel__server_not_found".into()),
        Err(e) => {
            log::error!("Database error in travel_without_destination when fetching server: {:?}", e);
            return Err("travel_without_destination__database_error".into());
        }
    };

    let available_roads: Vec<Road> = match get_road_by_source(
        server.universe_id,
        ctx.guild_channel().await.unwrap().parent_id.unwrap().get(),
    )
        .await
    {
        Ok(cursor) => {
            let res = cursor.try_collect().await;
            match res {
                Ok(road) => {road}
                Err(e) => {println!("{:?}", e); Vec::new()}
            }

        },
        Err(_) => Vec::new(),
    };

    let mut roads = vec![];
    for road in available_roads {
        let destination = if road.place_one_id == ctx.guild_channel().await.unwrap().parent_id.unwrap().get() {road.place_two_id}
            else {road.place_one_id};
        roads.push(CreateSelectMenuOption::new(road.road_name + " • " + road.distance.to_string().as_str() + "km", destination.to_string()));
    }

    let select_menu = serenity::all::CreateSelectMenu::new("select__menu__chose_destination",
        serenity::all::CreateSelectMenuKind::String {
            options: roads,
        }
    );

    let components = vec![CreateActionRow::SelectMenu(select_menu)];

    let result = ctx.send(CreateReply::default().components(components).reply(true)).await;
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("Reply failed in travel_without_destination: {:?}", e);
            Err("travel_without_destination__reply_failed".into())
        }
    }
}

pub async fn travel_from_handler(ctx: SerenityContext, interaction: ComponentInteraction) -> Result<&'static str, Error>{
    let destination_input = match &interaction.data.kind {
        serenity::all::ComponentInteractionDataKind::StringSelect { values } => {
            values.get(0).ok_or("create_character__invalid_interaction")?
        }
        _ => return Err("create_character__invalid_interaction".into()),
    };
    let server = match get_server_by_id(interaction.guild_id.unwrap().get()).await {
        Ok(Some(s)) => s,
        Ok(None) => return Err("travel__server_not_found".into()),
        Err(e) => {
            log::error!("Database error in _travel when fetching server: {:?}", e);
            return Err("travel__database_error".into());
        }
    };

    let destination_category_id = parse_channel_id(&destination_input).ok_or_else(|| Error::from("travel__place_not_found"))?;

    let destination_place = match get_place_by_category_id(server.universe_id, destination_category_id).await {
        Ok(Some(p)) => p,
        _ => return Err("travel__place_not_found".into()),
    };


    let _character = match server.clone().get_character_by_user_id(interaction.user.id.get()).await {
        Ok(Some(c)) => c,
        _ => return Err("travel__character_not_found".into()),
    };

    let mut player_move = match server.clone().get_player_move(interaction.user.id.get()).await {
        Ok(Some(m)) => m,
        _ => {
            // Initialise un nouveau PlayerMove si inexistant
            PlayerMove {
                universe_id: server.universe_id,
                user_id: interaction.user.id.get(),
                server_id: server.server_id,
                actual_space_id: interaction.channel_id.get(),
                actual_space_type: SpaceType::Place, // Par défaut, on suppose qu'il est dans un lieu
                ..Default::default()
            }
        }
    };

    // On s'assure que le mouvement est bien dans l'univers actuel
    if player_move.universe_id != server.universe_id {
        // Le joueur change d'univers (rare mais possible via admin)
        // On supprime l'ancien mouvement dans l'autre univers
        let _ = player_move.remove().await;
        player_move.universe_id = server.universe_id;
    }

    match player_move.actual_space_type {
        SpaceType::Road => {
            move_from_road(&ctx, interaction.channel_id.get(), server, player_move.clone()).await?;
        }
        SpaceType::Place => {
            move_from_place(&ctx, interaction.channel_id.get(), destination_input.parse().unwrap(), server, player_move.clone()).await?;
        }
    }

    Ok("")

    //Ok(("travel__started", destination_mention))
}


async fn move_from_place(ctx: &SerenityContext, source_id: u64, destination_id: u64, server: Server, mut player_move: PlayerMove) -> Result<&'static str, Error>{
    let source = ctx.http.get_channel(source_id.into()).await.unwrap();
    let source_id = source.clone().guild().unwrap().parent_id.unwrap().get();

    let dest_id = destination_id;

    let road = match server.clone().get_road(source_id, dest_id).await {
        Ok(Some(r)) => r,
        _ => return Err("move_from_place__road_not_found".into())
    };

    // Récupère les rôles des lieux source et destination
    let source_place = crate::database::places::get_place_by_category_id(server.universe_id, source_id).await
        .map_err(|_| Error::from("travel__database_error"))?
        .ok_or_else(|| Error::from("travel__source_place_not_found"))?;
    
    let dest_place = crate::database::places::get_place_by_category_id(server.universe_id, dest_id).await
        .map_err(|_| Error::from("travel__database_error"))?
        .ok_or_else(|| Error::from("travel__place_not_found"))?;

    player_move.actual_space_id = road.channel_id;
    player_move.actual_space_type = SpaceType::Road;
    player_move.road_id = Some(road.channel_id);
    player_move.road_role_id = Some(road.role_id);
    player_move.road_server_id = Some(road.server_id);
    player_move.source_id = Some(source_id);
    player_move.source_role_id = Some(source_place.role);
    player_move.source_server_id = Some(source_place.server_id);
    player_move.destination_id = Some(dest_id);
    player_move.destination_role_id = Some(dest_place.role);
    player_move.destination_server_id = Some(dest_place.server_id);
    
    add_travel(ctx.http.clone(), source.guild().unwrap().id.get(), player_move.clone()).await?;

    Ok("")
}
