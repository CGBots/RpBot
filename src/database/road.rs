use mongodb::bson::doc;
use serde_with::DisplayFromStr;
use mongodb::bson::oid::ObjectId;
use mongodb::results::InsertOneResult;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::database::db_client::{connect_db, DB_CLIENT};
use crate::database::db_namespace::ROADS_COLLECTION_NAME;
use crate::database::modifiers::Modifier;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Road{
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    pub universe_id: ObjectId,
    #[serde_as(as = "DisplayFromStr")]
    pub server_id: u64,
    pub server_two_id: Option<String>,
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
    pub secret: bool,
    pub modifiers: Vec<Modifier>
}

impl Road{
    pub async fn insert(self) -> mongodb::error::Result<InsertOneResult> {
        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
        db_client
            .database(&*self.universe_id.to_string())
            .collection::<Road>(ROADS_COLLECTION_NAME)
            .insert_one(self)
            .await
    }
}

pub async fn get_road_by_channel_id(universe_id: ObjectId, channel_id: u64) -> mongodb::error::Result<Option<Road>> {
    let filter = doc!{"channel_id": channel_id.to_string().as_str()};
    println!("channel_id: {}", channel_id);
    let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
    db_client
        .database(universe_id.to_string().as_str())
        .collection::<Road>(ROADS_COLLECTION_NAME)
        .find_one(filter)
        .await
}