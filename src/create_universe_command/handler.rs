// use serenity::all::GuildId;
// use crate::create_universe_command::create_universe::Universe;
// use crate::discord::poise_structs::*;
// 
// #[poise::command(slash_command)]
// pub async fn create_universe(
//     ctx: Context<'_>
// ) -> Result<(), Error> {
//     //TODO 1 créer les objets
//     // 2 appeler insert_universe
//     // 3 relever si un univers existe déjà, et envoyer l'erreur sinon indiquer que l'univers à bien été créé
//     
//     //TODO dans un second temps
//     // proposer un déploiement partiel ou complet
//     // créer les roles et autres éléments avant d'insérer dans la base de données
//     
//     let universe = Universe{
//         universe_id: Default::default(),
//         server_ids: vec![ctx.guild_id().unwrap().get()],
//         name: "".to_string(),
//         creator_id: 0,
//     };
//     match universe.insert_universe().await{
//         Ok(_) => {Ok(())}
//         Err(_) => {Ok(())}
//     }
//     
// }