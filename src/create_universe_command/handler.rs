use std::time::{SystemTime, UNIX_EPOCH};
use poise::CreateReply;
use crate::create_universe_command::logic::check_universe;
use crate::database::universe::{Universe, FREE_LIMIT_UNIVERSE};
use crate::database::db_client::DB_CLIENT;
use crate::discord::poise_structs::*;
use crate::translation::tr;

#[poise::command(slash_command, required_permissions= "ADMINISTRATOR", guild_only)]
pub async fn create_universe(
    ctx: Context<'_>,
    universe_name: String,
) -> Result<(), Error> {
    ctx.defer().await.unwrap();
    
    let mut universe = Universe{
        universe_id: Default::default(),
        server_ids: vec![ctx.guild_id().unwrap().get()],
        name: universe_name.clone(),
        creator_id: ctx.author().id.get(),
        global_time_modifier: 100,
        creation_timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
        default_locale: ctx.partial_guild().await.unwrap().preferred_locale,
    };
    
    let check_result = check_universe(ctx.guild_id().unwrap(), ctx.author().id).await;

    match check_result {
        Ok(_) => {}
        Err(fluent_token) => {
            ctx.send(
                CreateReply::default()
                    .content(tr!(ctx, fluent_token))
                    .ephemeral(true)
            ).await.unwrap();
            return Ok(());
        }
    }

    //TODO dans un second temps
    // proposer un déploiement partiel ou complet
    // créer les roles et autres éléments avant d'insérer dans la base de données

    let db_client = DB_CLIENT.lock().unwrap().clone();
    universe.universe_id = Default::default();
    match universe.insert_universe().await{
        Ok(result) => {
            ctx.send(
                CreateReply::default()
                    .content(tr!(ctx, "universe_created", universe_name: universe.name))
            ).await.unwrap();

            Ok(())
        }
        Err(_) => {Ok(())}
    }
}