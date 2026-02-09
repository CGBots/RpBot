//! Server model and database helpers.
//!
//! This module defines the `Server` document (as stored in MongoDB) and
//! convenience methods for inserting and retrieving server documents.
//!
//! A `Server` links a Discord guild to a universe and stores IDs for
//! roles/categories/channels used by the bot within that guild.
//!
//! Notes:
//! - The module uses `serde_with::DisplayFromStr` to serialize numeric
//!   identifiers as strings in MongoDB for compatibility.
//! - Database access is performed through the global `DB_CLIENT`.
//!
//! # Example
//! ```no_run
//! use crate::database::server::Server;
//!
//! // Build a new Server document and insert it into the universe-specific DB
//! let server = Server {
//!     _id: Default::default(),
//!     universe_id: Default::default(),
//!     server_id: 123456789012345678,
//!     admin_role_id: None,
//!     moderator_role_id: None,
//!     spectator_role_id: None,
//!     player_role_id: None,
//!     everyone_role_id: None,
//!     admin_category_id: None,
//!     nrp_category_id: None,
//!     rp_category_id: None,
//!     road_category_id: None,
//!     index_forum_id: None,
//!     character_channel_id: None,
//! };
//! let insert_res = server.insert_server("universe_db_name").await.unwrap();
//! ```

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IdType {
    Role,
    Channel,
    Category
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Id{
    pub id: Option<u64>,
    pub id_type: Option<IdType>
}

use mongodb::bson::{doc, to_document};
use mongodb::bson::oid::ObjectId;
use mongodb::results::{InsertOneResult, UpdateResult};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use crate::database::db_client::{DB_CLIENT, connect_db};
use crate::database::db_namespace::{SERVER_COLLECTION_NAME};

/// Represents a server (Discord guild) document stored in MongoDB.
///
/// This struct stores the link between a server and its universe and keeps
/// commonly used role/category/channel IDs so the bot can restore or
/// configure the guild accordingly.
///
/// All numeric identifiers are serialized as strings in the database via
/// `DisplayFromStr` to ensure compatibility with clients that expect string IDs.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Server {
    /// MongoDB ObjectId for this document.
    #[serde(rename = "_id")]
    pub _id: ObjectId,

    /// Reference to the universe document `_id` (stored as string).
    #[serde_as(as = "DisplayFromStr")]
    pub universe_id: ObjectId,

    /// Discord guild ID.
    #[serde_as(as = "DisplayFromStr")]
    pub server_id: u64,

    /// Optional role IDs used by the bot.
    pub admin_role_id: Id,

    pub moderator_role_id: Id,

    pub spectator_role_id: Id,

    pub player_role_id: Id,

    pub everyone_role_id: Id,

    /// Optional category / channel IDs used as configuration anchors.
    pub admin_category_id: Id,

    pub nrp_category_id: Id,

    pub rp_category_id: Id,

    pub road_category_id: Id,

    pub rp_wiki_channel_id: Id,

    pub character_channel_id: Id,

    pub log_channel_id: Id,

    pub moderation_channel_id: Id,

    pub commands_channel_id: Id,

    pub nrp_general_channel_id: Id,

    pub rp_character_channel_id: Id,

}

impl Server {
    /// Create a deep clone of the `Server` instance.
    ///
    /// Use this if you need an owned copy to mutate independently of the original.
    #[allow(unused)]
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
            rp_wiki_channel_id: self.rp_wiki_channel_id.clone(),
            character_channel_id: self.character_channel_id.clone(),
            log_channel_id: self.log_channel_id.clone(),
            moderation_channel_id: self.moderation_channel_id.clone(),
            commands_channel_id: self.commands_channel_id.clone(),
            nrp_general_channel_id: self.nrp_general_channel_id.clone(),
            rp_character_channel_id: self.rp_character_channel_id.clone(),
        }
    }

    /// Insert this `Server` document into the universe-specific MongoDB database.
    ///
    /// # Arguments
    ///
    /// * `universe_db_name` - Name of the MongoDB database to insert into (usually
    ///   derived from the universe document).
    ///
    /// # Returns
    ///
    /// * `Ok(InsertOneResult)` on success.
    /// * `Err(mongodb::error::Error)` when the insert fails.
    ///
    /// # Errors
    ///
    /// This method acquires a lock on the global `DB_CLIENT` and forwards the
    /// insert request to the specified database. Any MongoDB error is returned
    /// to the caller.
    pub async fn insert_server(&self, universe_db_name: &str) -> mongodb::error::Result<InsertOneResult> {
        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
        db_client
            .database(universe_db_name)
            .collection::<Server>(SERVER_COLLECTION_NAME)
            .insert_one(self)
            .await
    }

    /// Retrieve a `Server` document by its `server_id` (string form).
    ///
    /// # Arguments
    ///
    /// * `server_id` - The Discord guild ID as a string (serialized form in DB).
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Server))` if a matching document is found.
    /// * `Ok(None)` if no document matches the filter.
    /// * `Err(mongodb::error::Error)` on database errors.
    ///
    /// # Notes
    ///
    /// This convenience function queries the global RPBot database using the
    /// standard `RPBOT_DB_NAME`. If you want to search inside a universe-specific
    /// database, call the collection on that DB explicitly.
    pub async fn get_server_by_id(
        universe_id: String,
        server_id: String,
    ) -> mongodb::error::Result<Option<Server>> {
        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
        let filter = doc! { "server_id": server_id };
        db_client
            .database(universe_id.as_str())
            // NOTE: The original code used UNIVERSE_COLLECTION_NAME here.
            // It is likely that the correct collection is `SERVER_COLLECTION_NAME`.
            .collection::<Server>(SERVER_COLLECTION_NAME)
            .find_one(filter)
            .await
    }

    pub async fn update(&self) -> mongodb::error::Result<UpdateResult> {
        let mut doc = to_document(self).unwrap();
        doc.remove("_id");
        let filter = doc! {"_id": &self._id};
        let update = doc! {"$set": doc};

        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
        db_client
            .database(self.universe_id.to_string().as_str())
            .collection::<Server>(SERVER_COLLECTION_NAME)
            .update_one(filter, update).await
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

    lazy_static! {
        pub static ref UNIVERSE_ID: ObjectId = ObjectId::new();
    }

    /// Helper inserting a Universe document required by server tests.
    async fn insert_universe() -> Result<InsertOneResult, String> {
        let _ = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await;
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
        let result = Server {
            _id: Default::default(),
            universe_id: Default::default(),
            server_id: SERVER_ID,
            admin_role_id: Id::default(),
            moderator_role_id: Id::default(),
            spectator_role_id: Id::default(),
            player_role_id: Id::default(),
            everyone_role_id: Id::default(),
            admin_category_id: Id::default(),
            nrp_category_id: Id::default(),
            rp_category_id: Id::default(),
            road_category_id: Id::default(),
            rp_wiki_channel_id: Id::default(),
            character_channel_id: Id::default(),
            log_channel_id: Id::default(),
            moderation_channel_id: Id::default(),
            commands_channel_id: Id::default(),
            nrp_general_channel_id: Id::default(),
            rp_character_channel_id: Id::default(),
        }
            .insert_server("test")
            .await;

        assert!(result.is_ok());
    }
}
