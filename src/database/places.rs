use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::results::InsertOneResult;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::database::db_client::{connect_db, DB_CLIENT};
use crate::database::db_namespace::{PLACES_COLLECTION_NAME};
use crate::database::modifiers::Modifier;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Place{
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    pub universe_id: ObjectId,
    pub server_id: u64,
    pub category_id: u64,
    pub role: u64,
    pub name: String,
    pub modifiers: Vec<Modifier>,
}

impl Place{
    pub async fn insert_place(&self) -> mongodb::error::Result<InsertOneResult> {
        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
        db_client
            .database(&*self.universe_id.to_string())
            .collection::<Place>(PLACES_COLLECTION_NAME)
            .insert_one(self)
            .await
    }
}