use serenity::all::{GuildId, UserId};
use crate::database::universe::{Universe, FREE_LIMIT_UNIVERSE};

pub async fn check_universe(guild_id: GuildId, creator_id: UserId) -> Result<(), &'static str> {
    let universes = Universe::get_creator_universes(creator_id.get()).await;
    if universes.len() >= FREE_LIMIT_UNIVERSE {
        return Err("exceed_limit_number_of_universes")
    }

    for universe in universes {
        if universe.server_ids.contains(&guild_id.get()){
            return Err("create_universe__already_exist_for_this_server")
        }
    }
    Ok(())
}