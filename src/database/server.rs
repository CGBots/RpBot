// use mongodb::bson::oid::ObjectId;
// use serde::{Deserialize, Serialize};
// 
// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
// pub struct Server{
//     #[serde(rename = "_id")]
//     pub _id : ObjectId,
//     pub universe_id : ObjectId, // Referencing _id alias universe_id of universe structure.
//     pub server_id : u64,
//     pub is_main_server : bool,
//     pub admin_role_id : u64,
//     pub moderator_role_id : u64,
//     pub spectator_role_id : u64,
//     pub player_role_id : u64,
//     pub everyone_role_id : u64,
//     pub admin_category_id : u64,
//     pub nrp_category_id : u64,
//     pub rp_category_id : u64,
//     pub road_category_id : u64,
//     pub index_forum_id : u64,
//     pub character_channel_id : u64
// }
// 
// impl Server{
//     pub fn get_server_by_id(server_id: u64) -> Option<&Server>{}
//     let result = Universe::get_universe_by_server_id(self.server_ids[0]).await.unwrap();
//         match result {
//              None => {
//                  let db_client = DB_CLIENT.lock().unwrap().clone().unwrap();
//                  let result = db_client.database(RPBOT_DB_NAME).collection::<Universe>(UNIVERSE_COLLECTION_NAME).insert_one(self).await;
//                  Ok(())
//             }
//             Some(_) => {Err("create_universe__already_exist_for_this_server".to_string())}
//         }
// }