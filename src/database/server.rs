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

use std::cmp::PartialEq;
use log::{log, Level};
use mongodb::bson::{doc, to_document};
use mongodb::bson::oid::ObjectId;
use mongodb::results::{InsertOneResult, UpdateResult};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use crate::database::db_client::{DB_CLIENT, connect_db};
use crate::database::db_namespace::{RPBOT_DB_NAME, SERVER_COLLECTION_NAME};
use crate::discord::poise_structs::{Context, Error};

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum IdType {
    Role,
    Channel,
    Category
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Id{
    pub id: u64,
    pub id_type: IdType
}

impl From<(u64, IdType)> for Id {
    fn from((id, id_type): (u64, IdType)) -> Self {
        Id { id, id_type }
    }
}

impl From<u64> for Id {
    fn from(value: u64) -> Self {
        Id { id: value, id_type: IdType::Channel }  // Default to Channel type
    }
}

// Define a trait for the functionality
pub trait IdExt {
    async fn delete(&mut self, ctx: &Context<'_>) -> Result<&'static str, Error>;
}

// Implement the trait for Option<Id>
impl IdExt for Option<Id> {
    async fn delete(&mut self, ctx: &Context<'_>) -> Result<&'static str, Error> {
        match self {
            None => Err("id__nothing_to_delete".into()),
            Some(id) => {
                let guild_id = ctx.guild_id().ok_or_else(|| -> Error { "guild_only".into() })?;
                let http = ctx.http();

                match id.id_type {
                    IdType::Role => {
                        // Use HTTP via GuildId; do NOT borrow ctx.guild() (cache) across await.
                        match guild_id.delete_role(http, id.id).await {
                            Ok(_) => {
                                *self = None;
                                Ok("id__role_delete_success")
                            }
                            Err(_) => Err("id__role_delete_failed".into()),
                        }
                    }
                    _ => {
                        if http.delete_channel(id.id.into(), None).await.is_ok() {
                            *self = None;
                            Ok("id__channel_delete_sucess")
                        } else {
                            Err("id__channel_delete_failed".into())
                        }
                    }
                }
            }
        }
    }
}

/// Represents a server (Discord guild) document stored in MongoDB.
///
/// This struct stores the link between a server and its universe and keeps
/// commonly used role/category/channel IDs so the bot can restore or
/// configure the guild accordingly.
///
/// All numeric identifiers are serialized as strings in the database via
/// `DisplayFromStr` to ensure compatibility with clients that expect string IDs.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Server {
    /// MongoDB ObjectId for this document.
    #[serde(rename = "_id")]
    pub _id: ObjectId,

    /// Reference to the universe document `_id` (stored as string).
    pub universe_id: ObjectId,

    /// Discord guild ID.
    #[serde_as(as = "DisplayFromStr")]
    pub server_id: u64,

    /// Optional role IDs used by the bot.
    pub admin_role_id: Option<Id>,

    pub moderator_role_id: Option<Id>,

    pub spectator_role_id: Option<Id>,

    pub player_role_id: Option<Id>,

    pub everyone_role_id: Option<Id>,

    /// Optional category / channel IDs used as configuration anchors.
    pub admin_category_id: Option<Id>,

    pub nrp_category_id: Option<Id>,

    pub rp_category_id: Option<Id>,

    pub road_category_id: Option<Id>,

    pub rp_wiki_channel_id: Option<Id>,

    pub log_channel_id: Option<Id>,

    pub moderation_channel_id: Option<Id>,

    pub commands_channel_id: Option<Id>,

    pub nrp_general_channel_id: Option<Id>,

    pub rp_character_channel_id: Option<Id>,
}

impl Default for Server {
    fn default() -> Self {
        Server{
            _id: Default::default(),
            universe_id: Default::default(),
            server_id: 0,
            admin_role_id: None,
            moderator_role_id: None,
            spectator_role_id: None,
            player_role_id: None,
            everyone_role_id: None,
            admin_category_id: None,
            nrp_category_id: None,
            rp_category_id: None,
            road_category_id: None,
            rp_wiki_channel_id: None,
            log_channel_id: None,
            moderation_channel_id: None,
            commands_channel_id: None,
            nrp_general_channel_id: None,
            rp_character_channel_id: None,
        }
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
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
    pub async fn insert_server(&self) -> mongodb::error::Result<InsertOneResult> {
        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
        db_client
            .database(RPBOT_DB_NAME)
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
        server_id: String,
    ) -> mongodb::error::Result<Option<Server>> {
        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
        let filter = doc! { "server_id": server_id };
        db_client
            .database(RPBOT_DB_NAME)
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
            .database(RPBOT_DB_NAME)
            .collection::<Server>(SERVER_COLLECTION_NAME)
            .update_one(filter, update).await
    }

    pub fn universe_id(&mut self, universe_id: impl Into<ObjectId>) -> &mut Self {self.universe_id = universe_id.into(); self}
    pub fn server_id(&mut self, server_id: impl Into<u64>) -> &mut Self {self.server_id = server_id.into(); self}
    pub fn admin_role_id(&mut self, admin_role_id: impl Into<Id>) -> &mut Self {self.admin_role_id = Some(admin_role_id.into()); self}
    pub fn moderator_role_id(&mut self, moderator_role_id: impl Into<Id>) -> &mut Self {self.moderator_role_id = Some(moderator_role_id.into()); self}
    pub fn spectator_role_id(&mut self, spectator_role_id: impl Into<Id>) -> &mut Self {self.spectator_role_id = Some(spectator_role_id.into()); self}
    pub fn player_role_id(&mut self, player_role_id: impl Into<Id>) -> &mut Self {self.player_role_id = Some(player_role_id.into()); self}
    pub fn everyone_role_id(&mut self, everyone_role_id: impl Into<Id>) -> &mut Self {self.everyone_role_id = Some(everyone_role_id.into()); self}
    pub fn admin_category_id(&mut self, admin_category_id: impl Into<Id>) -> &mut Self {self.admin_category_id = Some(admin_category_id.into()); self}
    pub fn nrp_category_id(&mut self, nrp_category_id: impl Into<Id>) -> &mut Self {self.nrp_category_id = Some(nrp_category_id.into()); self}
    pub fn rp_category_id(&mut self, rp_category_id: impl Into<Id>) -> &mut Self {self.rp_category_id = Some(rp_category_id.into()); self}
    pub fn road_category_id(&mut self, road_category_id: impl Into<Id>) -> &mut Self {self.road_category_id = Some(road_category_id.into()); self}
    pub fn rp_wiki_channel_id(&mut self, rp_wiki_channel_id: impl Into<Id>) -> &mut Self {self.rp_wiki_channel_id = Some(rp_wiki_channel_id.into()); self}
    pub fn log_channel_id(&mut self, log_channel_id: impl Into<Id>) -> &mut Self {self.log_channel_id = Some(log_channel_id.into()); self}
    pub fn moderation_channel_id(&mut self, moderation_channel_id: impl Into<Id>) -> &mut Self {self.moderation_channel_id = Some(moderation_channel_id.into()); self}
    pub fn commands_channel_id(&mut self, commands_channel_id: impl Into<Id>) -> &mut Self {self.commands_channel_id = Some(commands_channel_id.into()); self}
    pub fn nrp_general_channel_id(&mut self, nrp_general_channel_id: impl Into<Id>) -> &mut Self{self.nrp_general_channel_id = Some(nrp_general_channel_id.into()); self}
    pub fn rp_character_channel_id(&mut self, rp_character_channel_id: impl Into<Id>) -> &mut Self {self.rp_character_channel_id = Some(rp_character_channel_id.into()); self}

    pub async fn rollback(&mut self, ctx: &Context<'_>, snapshot: Self) {
        use futures::future::join_all;

        let mut fields = [
            (&mut self.admin_role_id, snapshot.admin_role_id),
            (&mut self.moderator_role_id, snapshot.moderator_role_id),
            (&mut self.spectator_role_id, snapshot.spectator_role_id),
            (&mut self.player_role_id, snapshot.player_role_id),
            (&mut self.admin_category_id, snapshot.admin_category_id),
            (&mut self.nrp_category_id, snapshot.nrp_category_id),
            (&mut self.rp_category_id, snapshot.rp_category_id),
            (&mut self.road_category_id, snapshot.road_category_id),
            (&mut self.rp_wiki_channel_id, snapshot.rp_wiki_channel_id),
            (&mut self.log_channel_id, snapshot.log_channel_id),
            (&mut self.moderation_channel_id, snapshot.moderation_channel_id),
            (&mut self.commands_channel_id, snapshot.commands_channel_id),
            (&mut self.nrp_general_channel_id, snapshot.nrp_general_channel_id),
            (&mut self.rp_character_channel_id, snapshot.rp_character_channel_id),
        ];

        let delete_futures: Vec<_> = fields
            .iter_mut()
            .filter_map(|(field, snapshot_field)| {
                if **field != *snapshot_field {
                    Some(field.delete(ctx))
                } else {
                    None
                }
            })
            .collect();

        let results = join_all(delete_futures).await;
        results.iter().for_each(|r| {
            if let Err(err) = r {
                log!(
                    Level::Error,
                    "Error during setup and rollback.\nuniverse_id: {}\nserver_id: {}\n error: {}",
                    self.universe_id,
                    self.server_id,
                    err
                );
            }
        });
    }
    
    pub async fn snaphot(self, ctx: &Context<'_>) -> Self {
        let mut snapshot = self.clone();
        let guild_id = ctx.guild_id().unwrap();
        let roles = ctx.http().get_guild_roles(guild_id.into()).await.unwrap();
        let channels = ctx.http().get_channels(guild_id.into()).await.unwrap();


        let role_exists = |id: u64| roles.iter().any(|r| r.id.get() == id);

        if snapshot.admin_role_id.map(|x| role_exists(x.id)) == Some(false) {
            snapshot.admin_role_id = None;
        }
        if snapshot.moderator_role_id.map(|x| role_exists(x.id)) == Some(false) {
            snapshot.moderator_role_id = None;
        }
        if snapshot.spectator_role_id.map(|x| role_exists(x.id)) == Some(false) {
            snapshot.spectator_role_id = None;
        }
        if snapshot.player_role_id.map(|x| role_exists(x.id)) == Some(false) {
            snapshot.player_role_id = None;
        }
        if snapshot.everyone_role_id.map(|x| role_exists(x.id)) == Some(false) {
            snapshot.everyone_role_id = None;
        }

        let channel_exists = |id: u64| channels.iter().any(|r| r.id.get() == id);

        // ---- Channels/categories: check via get_channel ----

        if snapshot.road_category_id.map(|x| channel_exists(x.id)) == Some(false) {
            snapshot.road_category_id = None;
        }
        if snapshot.admin_category_id.map(|x| channel_exists(x.id)) == Some(false) {
            snapshot.admin_category_id = None;
        }
        if snapshot.nrp_category_id.map(|x| channel_exists(x.id)) == Some(false) {
            snapshot.nrp_category_id = None;
        }
        if snapshot.rp_category_id.map(|x| channel_exists(x.id)) == Some(false) {
            snapshot.rp_category_id = None;
        }
        if snapshot.log_channel_id.map(|x| channel_exists(x.id)) == Some(false) {
            snapshot.log_channel_id = None;
        }
        if snapshot.commands_channel_id.map(|x| channel_exists(x.id)) == Some(false) {
            snapshot.commands_channel_id = None;
        }
        if snapshot.moderation_channel_id.map(|x| channel_exists(x.id)) == Some(false) {
            snapshot.moderation_channel_id = None;
        }
        if snapshot.nrp_general_channel_id.map(|x| channel_exists(x.id)) == Some(false) {
            snapshot.nrp_general_channel_id = None;
        }
        if snapshot.rp_character_channel_id.map(|x| channel_exists(x.id)) == Some(false) {
            snapshot.rp_character_channel_id = None;
        }
        if snapshot.rp_wiki_channel_id.map(|x| channel_exists(x.id)) == Some(false) {
            snapshot.rp_wiki_channel_id = None;
        }

        snapshot
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

    #[tokio::test]
    async fn test_insert_server() {
        insert_universe().await.unwrap();
        let result = Server::default()
            .insert_server()
            .await;

        assert!(result.is_ok());
    }
}
