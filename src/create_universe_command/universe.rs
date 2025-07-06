use serde_with::{DisplayFromStr, serde_as};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc};
use mongodb::Cursor;
use mongodb::results::InsertOneResult;
use serde::{Deserialize, Serialize};
use crate::database::db_client::DB_CLIENT;
use crate::database::db_namespace::{RPBOT_DB_NAME, UNIVERSE_COLLECTION_NAME};

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Universe {
    #[serde(rename = "_id")]
    pub universe_id: ObjectId,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub server_ids: Vec<u64>,
    pub name: String,
    #[serde_as(as = "DisplayFromStr")]
    pub creator_id: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub global_time_modifier: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub creation_timestamp: u128,
    pub default_locale: String,
}

impl Universe{
    #[allow(dead_code)]
    pub async fn get_universe_by_server_id(server_id: u64) -> mongodb::error::Result<Cursor<Universe>> {
        let db_client = DB_CLIENT.lock().unwrap().clone();
        let filter = doc! { "server_ids": {"$in": [server_id.to_string()] } };
        db_client.database(RPBOT_DB_NAME).collection::<Universe>(UNIVERSE_COLLECTION_NAME).find(filter).await
    }
    
    /// Method to add the universe struct in the database.
    /// WARNING it verify nothing
    pub async fn insert_universe(&self) -> mongodb::error::Result<InsertOneResult> {
        let db_client = DB_CLIENT.lock().unwrap().clone();
        db_client.database(RPBOT_DB_NAME).collection::<Universe>(UNIVERSE_COLLECTION_NAME).insert_one(self).await
    }
}

#[cfg(test)]
mod test{
    use std::time::SystemTime;
    use mongodb::bson::doc;
    use mongodb::results::{DeleteResult, InsertOneResult};
    use crate::create_universe_command::universe::Universe;
    use crate::database::db_client::{DB_CLIENT};
    use crate::database::db_namespace::{RPBOT_DB_NAME, UNIVERSE_COLLECTION_NAME};

    static SERVER_ID: u64 = 1;

    async fn insert_universe() -> Result<InsertOneResult, String> {
        DB_CLIENT.lock().unwrap().connect_db().await.unwrap();
        let universe = Universe{
            universe_id: Default::default(),
            server_ids: vec!(SERVER_ID),
            name: "test".to_string(),
            creator_id: 0,
            global_time_modifier: 100,
            creation_timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis(),
            default_locale: "".to_string(),
        };
        match universe.insert_universe().await {
            Ok(universe) => {Ok(universe)}
            Err(e) => {
                println!("{}", e);
                Err(e.to_string())
            }
        }
    }

    async fn delete_previously_setup() -> DeleteResult {
        let db_client = DB_CLIENT.lock().unwrap().clone();
        let filter = doc! { "server_ids": {"$in": [SERVER_ID.to_string()] } };
        db_client.database(RPBOT_DB_NAME).collection::<Universe>(UNIVERSE_COLLECTION_NAME).delete_many(filter).await.unwrap()
    }

    #[tokio::test]
    async fn test_create_universe() {
        let insertion_result = insert_universe().await;
        match insertion_result {
            Ok(_) => {assert!(true)}
            Err(e) => {
                println!("{}", e);
                assert!(false)
            }
        }
        delete_previously_setup().await;
    }

    #[tokio::test]
    async fn test_delete_previously_setup() {
        let _ = insert_universe().await;
        let result = delete_previously_setup().await;
        assert_ne!(result.deleted_count, 0);
    }

    #[tokio::test]
    async fn test_recover_universe_data() {
        let _ = insert_universe().await;
        let result = Universe::get_universe_by_server_id(1).await;
        DB_CLIENT.lock().unwrap().clone().database("RpBot");
        delete_previously_setup().await;
        match result {
            Ok(data) => {
                let universe_data = data.deserialize_current().unwrap();
                println!("{:?}", universe_data)
            }
            Err(_) => {assert!(false, "get data failed")}
        }
    }
}