//TODO Documenter ce code !

use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::results::InsertOneResult;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use crate::database::db_client::DB_CLIENT;
use crate::database::db_namespace::{RPBOT_DB_NAME, SERVER_COLLECTION_NAME, UNIVERSE_COLLECTION_NAME};
use crate::database::universe::Universe;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Server{
    #[serde(rename = "_id")]
    pub _id : ObjectId,
    
    #[serde_as(as = "DisplayFromStr")]
    pub universe_id : ObjectId, // Referencing _id alias universe_id of universe structure.

    #[serde_as(as = "DisplayFromStr")]
    pub server_id : u64,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub admin_role_id: Option<u64>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub moderator_role_id: Option<u64>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub spectator_role_id: Option<u64>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub player_role_id: Option<u64>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub everyone_role_id: Option<u64>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub admin_category_id: Option<u64>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub nrp_category_id: Option<u64>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub rp_category_id: Option<u64>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub road_category_id: Option<u64>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub index_forum_id: Option<u64>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub character_channel_id: Option<u64>
}

impl Server{
    pub fn clone(&self) -> Self {
        Self {
            _id: self._id.clone(),
            universe_id: self.universe_id.clone(),
            server_id: self.server_id.clone(),
            admin_role_id: self.admin_role_id.clone(),
            moderator_role_id: self.moderator_role_id.clone(),
            spectator_role_id: self.spectator_role_id.clone(),
            player_role_id: self.player_role_id.clone(),
            everyone_role_id: self.everyone_role_id.clone(),
            admin_category_id: self.admin_category_id.clone(),
            nrp_category_id: self.nrp_category_id.clone(),
            rp_category_id: self.rp_category_id.clone(),
            road_category_id: self.road_category_id.clone(),
            index_forum_id: self.index_forum_id.clone(),
            character_channel_id: self.character_channel_id.clone()
        }
    }

    pub async fn insert_server(&self, universe_db_name: &str) -> mongodb::error::Result<InsertOneResult> {
        let db_client = DB_CLIENT.lock().unwrap().clone();
        db_client
            .database(universe_db_name)
            .collection::<Server>(SERVER_COLLECTION_NAME)
            .insert_one(self)
            .await
    }


    pub async fn get_server_by_id(
        server_id: String,
    ) -> mongodb::error::Result<Option<Server>> {
        let db_client = DB_CLIENT.lock().unwrap().clone();
        let filter = doc! { "server_id": server_id };
        db_client
            .database(RPBOT_DB_NAME)
            .collection::<Server>(UNIVERSE_COLLECTION_NAME)
            .find_one(filter)
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::database::db_client::DB_CLIENT;
    use crate::database::universe::Universe;
    use std::time::SystemTime;
    use lazy_static::lazy_static;
    use super::*;

    static SERVER_ID: u64 = 1;

    lazy_static!{
        pub static ref UNIVERSE_ID: ObjectId = ObjectId::new();
    }
    
    async fn insert_universe() -> Result<InsertOneResult, String> {
        DB_CLIENT.lock().unwrap().connect_db().await.unwrap();
        let universe = Universe {
            universe_id: *UNIVERSE_ID,
            server_ids: vec![SERVER_ID],
            name: "test".to_string(),
            creator_id: 0,
            global_time_modifier: 100,
            creation_timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            default_locale: "".to_string(),
        };
        match universe.insert_universe().await {
            Ok(universe) => Ok(universe),
            Err(e) => {
                println!("{}", e);
                Err(e.to_string())
            }
        }
    }

    #[tokio::test]
    async fn test_insert_server() {
        insert_universe().await.unwrap();
        let result = Server{
            _id: Default::default(),
            universe_id: Default::default(),
            server_id: SERVER_ID,
            admin_role_id: None,
            moderator_role_id: None,
            spectator_role_id: None,
            player_role_id: None,
            everyone_role_id: None,
            admin_category_id: None,
            nrp_category_id: None,
            rp_category_id: None,
            road_category_id: None,
            index_forum_id: None,
            character_channel_id: None,
        }.insert_server("test").await;
        
        assert!(result.is_ok());
    }
}