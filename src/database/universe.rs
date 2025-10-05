use futures::TryStreamExt;
use crate::database::db_client::DB_CLIENT;
use crate::database::db_namespace::{RPBOT_DB_NAME, UNIVERSE_COLLECTION_NAME};
use mongodb::Cursor;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::results::{InsertOneResult, UpdateResult};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

pub static FREE_LIMIT_UNIVERSE: usize = 2;

/// Represents a universe in the RPBot system.
///
/// A universe is a container for servers, settings, and metadata.
/// Each universe is uniquely identified and associated with a creator.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Universe {
    /// The unique MongoDB ObjectId for this universe.
    #[serde(rename = "_id")]
    pub universe_id: ObjectId,

    /// List of server IDs associated with this universe.
    /// Stored as strings in MongoDB for compatibility.
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub server_ids: Vec<u64>,

    /// The display name of the universe.
    pub name: String,

    /// The user ID of the creator of this universe.
    #[serde_as(as = "DisplayFromStr")]
    pub creator_id: u64,

    /// A global time modifier used for time-based calculations.
    #[serde_as(as = "DisplayFromStr")]
    pub global_time_modifier: u32,

    /// Timestamp of when the universe was created (in milliseconds since epoch).
    #[serde_as(as = "DisplayFromStr")]
    pub creation_timestamp: u128,

    /// The default locale/language for this universe (e.g., "en-US", "fr-FR").
    pub default_locale: String,
}

impl Universe {
    pub fn clone(&self) -> Self {
        Self {
            universe_id: self.universe_id.clone(),
            server_ids: self.server_ids.clone(),
            name: self.name.clone(),
            creator_id: self.creator_id,
            global_time_modifier: self.global_time_modifier,
            creation_timestamp: self.creation_timestamp,
            default_locale: self.default_locale.clone(),
        }
    }
}

impl Universe {
    /// Retrieves all universes that include the given server ID in their `server_ids` array.
    ///
    /// ⚠️ This method performs no validation or authorization checks.
    ///
    /// # Arguments
    /// * `server_id` - The ID of the server to search for.
    ///
    /// # Returns
    /// A MongoDB cursor over matching `Universe` documents.
    pub async fn get_universe_by_server_id(
        server_id: u64,
    ) -> mongodb::error::Result<Cursor<Universe>> {
        let db_client = DB_CLIENT.lock().unwrap().clone();
        let filter = doc! { "server_ids": {"$in": [server_id.to_string()] } };
        db_client
            .database(RPBOT_DB_NAME)
            .collection::<Universe>(UNIVERSE_COLLECTION_NAME)
            .find(filter)
            .await
    }

    /// Inserts the current `Universe` instance into the database.
    ///
    /// ⚠️ This method does not check for duplicates or validate the data.
    ///
    /// # Returns
    /// A MongoDB `InsertOneResult` indicating the outcome of the operation.
    pub async fn insert_universe(&self) -> mongodb::error::Result<InsertOneResult> {
        let db_client = DB_CLIENT.lock().unwrap().clone();
        db_client
            .database(RPBOT_DB_NAME)
            .collection::<Universe>(UNIVERSE_COLLECTION_NAME)
            .insert_one(self)
            .await
    }

    /// Retrieves all universes created by the specified user.
    ///
    /// ⚠️ This method does not validate the user ID or check permissions.
    ///
    /// # Arguments
    /// * `user_id` - The creator's user ID.
    ///
    /// # Returns
    /// A MongoDB cursor over matching `Universe` documents.
    pub async fn get_creator_universes(user_id: u64) -> Vec<Universe> {
        let db_client = DB_CLIENT.lock().unwrap().clone();
        let filter = doc! { "creator_id": user_id.to_string() };
        db_client
            .database(RPBOT_DB_NAME)
            .collection::<Universe>(UNIVERSE_COLLECTION_NAME)
            .find(filter)
            .await
            .unwrap().try_collect().await.unwrap()
    }

    /// Adds a server ID to the `server_ids` array of this universe.
    ///
    /// Uses `$addToSet` to ensure the server ID is only added if it doesn't already exist.
    ///
    /// ⚠️ This method does not validate the server ID or check for authorization.
    ///
    /// # Arguments
    /// * `server_id` - The server ID to associate with this universe.
    ///
    /// # Returns
    /// A MongoDB `UpdateResult` indicating the outcome of the update.
    pub async fn add_server_to_universe(
        &self,
        server_id: u64,
    ) -> mongodb::error::Result<UpdateResult> {
        let db_client = DB_CLIENT.lock().unwrap().clone();
        let filter = doc! { "_id": self.universe_id };
        let data_to_insert = doc! {"$addToSet": { "server_ids": server_id.to_string()}};
        db_client
            .database(RPBOT_DB_NAME)
            .collection::<Universe>(UNIVERSE_COLLECTION_NAME)
            .update_one(filter, data_to_insert)
            .await
    }

    /// Retrieves a universe document by its unique ID.
    ///
    /// ⚠️ This method does not validate the format of the ID beyond parsing.
    ///
    /// # Arguments
    /// * `universe_id` - A string representation of the universe's ObjectId.
    ///
    /// # Returns
    /// An `Option<Universe>` if found, or `None` if no match exists.
    ///
    /// # Errors
    /// Returns a MongoDB error if the ID is invalid or the query fails.
    pub async fn get_universe_by_id(
        universe_id: String,
    ) -> mongodb::error::Result<Option<Universe>> {
        let db_client = DB_CLIENT.lock().unwrap().clone();
        let object_id = ObjectId::parse_str(&universe_id).map_err(|e| println!("{}", e));
        let filter = doc! { "_id": object_id.unwrap() };
        db_client
            .database(RPBOT_DB_NAME)
            .collection::<Universe>(UNIVERSE_COLLECTION_NAME)
            .find_one(filter)
            .await
    }

    pub fn get_universe_database_name(&self) -> String{
        format!("{}_{}",self.name, self.universe_id)
    }
}

/// Unit tests for the `Universe` model and its database interactions.
#[cfg(test)]
mod test {
    use crate::database::db_client::DB_CLIENT;
    use crate::database::db_namespace::{RPBOT_DB_NAME, UNIVERSE_COLLECTION_NAME};
    use crate::database::universe::Universe;
    use mongodb::bson::doc;
    use mongodb::results::{DeleteResult, InsertOneResult};
    use std::time::SystemTime;

    static SERVER_ID: u64 = 1;

    /// Inserts a test universe into the database.
    ///
    /// Returns the result of the insertion, or an error string if it fails.
    async fn insert_universe() -> Result<InsertOneResult, String> {
        DB_CLIENT.lock().unwrap().connect_db().await.unwrap();
        let universe = Universe {
            universe_id: Default::default(),
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

    /// Deletes all universes that contain the test server ID.
    ///
    /// Used for cleanup after each test.
    async fn delete_previously_setup() -> DeleteResult {
        let db_client = DB_CLIENT.lock().unwrap().clone();
        let filter = doc! { "server_ids": {"$in": [SERVER_ID.to_string()] } };
        db_client
            .database(RPBOT_DB_NAME)
            .collection::<Universe>(UNIVERSE_COLLECTION_NAME)
            .delete_many(filter)
            .await
            .unwrap()
    }

    /// Tests that a universe can be successfully inserted into the database.
    #[tokio::test]
    async fn test_create_universe() {
        let insertion_result = insert_universe().await;
        match insertion_result {
            Ok(_) => {
                assert!(true)
            }
            Err(e) => {
                println!("{}", e);
                assert!(false)
            }
        }
        delete_previously_setup().await;
    }

    /// Tests that the cleanup function successfully deletes inserted universes.
    #[tokio::test]
    async fn test_delete_previously_setup() {
        let _ = insert_universe().await;
        let result = delete_previously_setup().await;
        assert_ne!(result.deleted_count, 0);
    }

    /// Tests that a universe can be retrieved by its associated server ID.
    #[tokio::test]
    async fn test_recover_universe_data() {
        let _ = insert_universe().await;
        let result = Universe::get_universe_by_server_id(1).await;
        delete_previously_setup().await;
        match result {
            Ok(data) => {
                println!("{:?}", data.current());
                let universe_data = data.deserialize_current().unwrap();
                println!("{:?}", universe_data)
            }
            Err(_) => {
                assert!(false, "get data failed")
            }
        }
    }

    /// Tests that universes can be retrieved by their creator ID.
    #[tokio::test]
    async fn test_recover_universe_by_creator_id() {
        let _ = insert_universe().await;
        let result = Universe::get_creator_universes(0).await;
        delete_previously_setup().await;
        if result.is_empty(){
            println!("no universes found");
            assert!(false)
        }
        println!("{:?}", result)
    }

    /// Tests that a universe can be retrieved by its ObjectId.
    #[tokio::test]
    async fn test_recover_universe_by_id() {
        let universe = insert_universe().await;
        let id = universe
            .unwrap()
            .inserted_id
            .as_object_id()
            .unwrap()
            .to_hex();
        let result = Universe::get_universe_by_id(id).await;
        println!("{:?}", result);
        delete_previously_setup().await;
        match result {
            Ok(data) => {
                let universe_data = data.unwrap();
                println!("{:?}", universe_data)
            }
            Err(_) => {
                assert!(false, "get data failed")
            }
        }
    }

    #[tokio::test]
    async fn test_recover_unexisting_universe_by_id() {
        let _ = insert_universe().await;
        let result = Universe::get_creator_universes(1).await;
        if !result.is_empty(){
            println!("universes found {:?}", result);
            assert!(false)
        }
        delete_previously_setup().await;
    }
}
