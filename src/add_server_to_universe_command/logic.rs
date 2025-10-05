use futures::TryStreamExt;
use crate::database::universe::Universe;

pub async fn check_server_in_universe(guild_id: u64) -> Result<Universe, String>{
    if let Ok(mut cursor) = Universe::get_universe_by_server_id(guild_id).await {
        if let Some(universe) = cursor.try_next().await.unwrap() {
            return Ok(universe);
        }
    }
    Err(format!("Guild {} not bind to any existing universe", guild_id))
}

pub async fn add_server_to_universe(universe: String, guild_id: u64) -> Result<Universe, String> {
    let universe = Universe::get_universe_by_id(universe)
        .await
        .unwrap()
        .unwrap();

    let result = universe.add_server_to_universe(guild_id)
        .await
        .unwrap();
    
    Ok(universe)
}