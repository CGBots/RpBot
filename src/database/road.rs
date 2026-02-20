use serde_with::DisplayFromStr;
use mongodb::bson::oid::ObjectId;
use mongodb::results::InsertOneResult;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::database::db_client::{connect_db, DB_CLIENT};
use crate::database::db_namespace::ROAD_COLLECTION_NAME;
use crate::database::modifiers::Modifier;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Road{
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    pub universe_id: ObjectId,
    #[serde_as(as = "DisplayFromStr")]
    pub server_id: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub role_id: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub channel_id: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub place_one_id: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub place_two_id: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub distance: u64,
    pub modifiers: Vec<Modifier>
}

impl Road{
    pub async fn insert(self) -> mongodb::error::Result<InsertOneResult> {
        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
        db_client
            .database(&*self.universe_id.to_string())
            .collection::<Road>(ROAD_COLLECTION_NAME)
            .insert_one(self)
            .await
    }
}