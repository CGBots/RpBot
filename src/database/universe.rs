//! Module defining the `Universe` model and its database interactions.
//!
//! A **Universe** is a container for Discord servers, metadata, and settings
//! within the RPBot system. This module provides:
//! - The `Universe` struct representing the database document.
//! - CRUD methods for interacting with MongoDB.
//! - Utility functions for retrieving universes by server or creator.
//! - Constants defining free-tier limits.
//! - Unit tests for insertion, deletion, and retrieval.
//!
//! ⚠️ Note: Most methods currently do **not** perform authorization checks.

use futures::TryStreamExt;
use crate::database::db_client::{DB_CLIENT, connect_db};
use crate::database::db_namespace::{RPBOT_DB_NAME, SERVER_COLLECTION_NAME, STATS_COLLECTION_NAME, UNIVERSE_COLLECTION_NAME};
use mongodb::bson::{doc, from_document};
use mongodb::bson::oid::ObjectId;
use mongodb::IndexModel;
use mongodb::options::{IndexOptions};
use mongodb::results::{CreateIndexResult, InsertOneResult};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use tokio::join;
use crate::database::server::{Server};
use crate::database::stats::Stat;
use crate::discord::poise_structs::Error;

/// Maximum number of universes a creator can have in the free tier.
pub static FREE_LIMIT_UNIVERSE: usize = 2;

/// Maximum number of servers per universe in the free tier.
pub static FREE_LIMIT_SERVERS_PER_UNIVERSE: usize = 2;

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

    /// The display name of the universe.
    pub name: String,

    /// The user ID of the creator of this universe.
    #[serde_as(as = "DisplayFromStr")]
    pub creator_id: u64,

    /// A global time modifier used for time-based calculations.
    #[serde_as(as = "DisplayFromStr")]
    pub global_time_modifier: u32,

    /// Timestamp of when the universe was created (milliseconds since epoch).
    #[serde_as(as = "DisplayFromStr")]
    pub creation_timestamp: u128,
}

impl Universe {
    /// Retrieves all universes that include the given server ID.
    ///
    /// ⚠️ No authorization or validation is performed.
    ///
    /// # Arguments
    /// * `server_id` - Discord server ID to search for.
    ///
    /// # Returns
    /// A MongoDB cursor over matching `Universe` documents.
    pub async fn get_universe_by_server_id(
        server_id: u64,
    ) -> mongodb::error::Result<Option<Universe>> {
        let db_client = DB_CLIENT
            .get_or_init(|| async { connect_db().await.unwrap() })
            .await
            .clone();

        let pipeline = vec![
            doc! { "$match": { "server_id": server_id.to_string() } },
            doc! { "$lookup": {
            "from": UNIVERSE_COLLECTION_NAME,   // the UNIVERSE collection
            "localField": "universe_id",        // field in SERVER
            "foreignField": "_id",              // field in UNIVERSE
            "as": "universe"
        }},
            doc! { "$unwind": "$universe" }         // flatten the array
        ];

        let mut cursor = db_client
            .database(RPBOT_DB_NAME)
            .collection::<Server>(SERVER_COLLECTION_NAME)
            .aggregate(pipeline)
            .await?;


        if let Some(doc) = cursor.try_next().await? {
            // Extract the joined universe document
            let universe_doc = doc.get_document("universe").unwrap();
            let universe: Universe = from_document(universe_doc.clone())?;
            return Ok(Some(universe));
        }

        Ok(None)
    }


    /// Inserts the current `Universe` instance into MongoDB.
    ///
    /// ⚠️ No duplicate checks are performed.
    ///
    /// # Returns
    /// `InsertOneResult` indicating the insertion outcome.
    pub async fn insert_universe(&self) -> mongodb::error::Result<InsertOneResult> {
        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
        db_client
            .database(RPBOT_DB_NAME)
            .collection::<Universe>(UNIVERSE_COLLECTION_NAME)
            .insert_one(self)
            .await
    }

    /// Retrieves all universes created by a specific user.
    ///
    /// ⚠️ No validation of user ID or permissions.
    ///
    /// # Arguments
    /// * `user_id` - Creator's user ID.
    ///
    /// # Returns
    /// A `Vec<Universe>` of universes owned by the user.
    pub async fn get_creator_universes(user_id: u64) -> Vec<Universe> {
        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
        let filter = doc! { "creator_id": user_id.to_string() };
        db_client
            .database(RPBOT_DB_NAME)
            .collection::<Universe>(UNIVERSE_COLLECTION_NAME)
            .find(filter)
            .await
            .unwrap().try_collect().await.unwrap()
    }

    pub async fn check_universe_limit(user_id: u64) -> Result<bool, Error> {
        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
        let filter = doc! { "creator_id": user_id.to_string() };
        let result  = db_client
            .database(RPBOT_DB_NAME)
            .collection::<Universe>(UNIVERSE_COLLECTION_NAME)
            .count_documents(filter)
            .await?;

        Ok(result <= FREE_LIMIT_UNIVERSE as u64)
    }

    /// Adds a server to this universe.
    ///
    /// Uses `$addToSet` to prevent duplicates.
    ///
    /// ⚠️ No validation or authorization checks.
    ///
    /// # Arguments
    /// * `server_id` - Discord server ID to add.
    ///
    /// # Returns
    /// `UpdateResult` of the update operation.
    pub async fn add_server_to_universe(
        &self,
        mut server: Server,
    ) -> mongodb::error::Result<InsertOneResult> {
        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();

        let serv = server.universe_id(self.universe_id);

        db_client
            .database(RPBOT_DB_NAME)
            .collection::<Server>(SERVER_COLLECTION_NAME)
            .insert_one(serv)
            .await
    }

    /// Retrieves a universe by its ObjectId.
    ///
    /// ⚠️ Only parses string IDs; no further validation.
    ///
    /// # Arguments
    /// * `universe_id` - String representation of ObjectId.
    ///
    /// # Returns
    /// `Option<Universe>` if found, or `None`.erver
    pub async fn get_universe_by_id(
        universe_id: String,
    ) -> mongodb::error::Result<Option<Universe>> {
        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
        let object_id = ObjectId::parse_str(&universe_id).map_err(|e| println!("{}", e));
        let filter = doc! { "_id": object_id.unwrap() };
        db_client
            .database(RPBOT_DB_NAME)
            .collection::<Universe>(UNIVERSE_COLLECTION_NAME)
            .find_one(filter)
            .await
    }

    /// Creates a deep clone of the current `Universe`.
    #[allow(unused)]
    pub fn clone(&self) -> Self {
        Self {
            universe_id: self.universe_id.clone(),
            name: self.name.clone(),
            creator_id: self.creator_id.clone(),
            global_time_modifier: self.global_time_modifier.clone(),
            creation_timestamp: self.creation_timestamp.clone(),
        }
    }

    #[allow(unused)]
    pub async fn check_universe_ownership(server_id: u64, user_id: u64) -> Result<bool, String> {
        let result = Self::get_universe_by_server_id(server_id).await;
        match result {
            Ok(cursor) => {
                match cursor {
                    Some(universe) => {
                        if universe.creator_id == user_id { Ok(true) } else { Ok(false) }
                    }
                    None => { Err("check_universe_ownership__universe_not_found".to_string()) }
                }
            }
            Err(_) => { Err("check_universe_ownership__universe_not_found".to_string()) }
        }
    }

    pub async fn delete(&self) -> Result<&str, Error> {
        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
        let filter_universe = doc! { "_id": self.universe_id};
        let filter_server = doc!{"universe_id": self.universe_id};

        let drop_db = db_client
            .database(self.universe_id.to_string().as_str());

        let delete_universe = db_client
            .database(RPBOT_DB_NAME)
            .collection::<Universe>(UNIVERSE_COLLECTION_NAME);

        let delete_servers = db_client
            .database(RPBOT_DB_NAME)
            .collection::<Server>(SERVER_COLLECTION_NAME)
            ;

        let (db_drop_result, universe_delete, server_delete) = join!(drop_db.drop(), delete_universe.delete_one(filter_universe), delete_servers.delete_one(filter_server));
        if db_drop_result.is_err() || universe_delete.is_err() || server_delete.is_err(){
            return Err("universe_delete__failed".into());
        }
        Ok("universe_delete__passed")
    }


    pub async fn setup_constraints(&self) -> mongodb::error::Result<CreateIndexResult> {
        let db_client = DB_CLIENT .get_or_init(|| async { connect_db().await.unwrap() }) .await .clone();
        let index_keys = doc! {"name": 1};
        let index_options = IndexOptions::builder().unique(true).build();
        let index_model = IndexModel::builder()
            .keys(index_keys)
            .options(index_options)
            .build();
        db_client
            .database(self.universe_id.to_string().as_str())
            .collection::<Stat>(STATS_COLLECTION_NAME)
            .create_index(index_model)
            .await
    }

    pub async fn check_server_limit(self) -> Result<bool, &'static str> {
        let db_client = DB_CLIENT .get_or_init(|| async { connect_db().await.unwrap() }) .await .clone();
        let filter = doc!{"universe_id": self.universe_id};
        let servers_result_request = db_client
            .database(RPBOT_DB_NAME)
            .collection::<Server>(SERVER_COLLECTION_NAME)
            .count_documents(filter)
            .await;

        match servers_result_request {
            Ok(server_count) => {
                if server_count >= FREE_LIMIT_UNIVERSE as u64 {
                    return Ok(false)
                }
            }
            Err(_) => { return Err("universe__check_server_limit_failed".into()) }
        }

        Ok(true)
    }
}

/// Unit tests for the `Universe` model.
#[cfg(test)]
mod test {
    use crate::database::db_client::{connect_db, DB_CLIENT};
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
        let _ = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await;
        let universe = Universe {
            universe_id: Default::default(),
            name: "test".to_string(),
            creator_id: 0,
            global_time_modifier: 100,
            creation_timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
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
        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
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
                match data{
                    None => {assert!(false, "no universe found")}
                    Some(universe_data) => {println!("{:?}", universe_data)}
                }
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
