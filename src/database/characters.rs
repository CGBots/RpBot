use serde_with::DisplayFromStr;
use mongodb::bson::oid::ObjectId;
use mongodb::results::InsertOneResult;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::database::db_client::{connect_db, DB_CLIENT};
use crate::database::db_namespace::{CHARACTER_COLLECTION_NAME};
use crate::database::stats::Stat;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    #[serde_as(as = "DisplayFromStr")]
    pub user_id: u64,
    pub universe_id: ObjectId,
    pub name: String,
    pub stats: Vec<Stat>
}

impl Character {
    pub async fn update(self) -> mongodb::error::Result<InsertOneResult> {
        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
        db_client
            .database(self.universe_id.to_string().as_str())
            .collection::<Character>(CHARACTER_COLLECTION_NAME)
            .insert_one(self)
            .await
    }
}