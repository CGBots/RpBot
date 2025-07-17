use std::time::{SystemTime, UNIX_EPOCH};
use crate::universe::Universe;
use crate::database::db_client::DB_CLIENT;
use crate::discord::poise_structs::*;

#[poise::command(slash_command, required_permissions= "ADMINISTRATOR", guild_only)]
pub async fn create_universe(
    ctx: Context<'_>,
    universe_name: String,
) -> Result<(), Error> {
    //TODO 1 créer les objets
    // 2 appeler insert_universe
    // 3 relever si un univers existe déjà, et envoyer l'erreur sinon indiquer que l'univers à bien été créé
    
    //TODO dans un second temps
    // proposer un déploiement partiel ou complet
    // créer les roles et autres éléments avant d'insérer dans la base de données
    
    let mut universe = Universe{
        universe_id: Default::default(),
        server_ids: vec![ctx.guild_id().unwrap().get()],
        name: universe_name.clone(),
        creator_id: ctx.author().id.get(),
        global_time_modifier: 100,
        creation_timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
        default_locale: ctx.partial_guild().await.unwrap().preferred_locale,
    };

    let db_client = DB_CLIENT.lock().unwrap().clone();
    universe.universe_id = Default::default();
    match universe.insert_universe().await{
        Ok(result) => {
            db_client.database(format!("{}_{}",universe_name, result.inserted_id.as_object_id().unwrap().to_string()).as_str())
                .collection::<Universe>("universe")
                .insert_one(universe)
                .await
                .unwrap();
            Ok(())

        }
        Err(_) => {Ok(())}
    }
}