use futures::TryStreamExt;
use serde_with::DisplayFromStr;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::results::InsertOneResult;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serenity::all::{GuildChannel};
use crate::database::db_client::{connect_db, DB_CLIENT};
use crate::database::db_namespace::{PLACES_COLLECTION_NAME, ROADS_COLLECTION_NAME};
use crate::database::modifiers::Modifier;
use crate::database::road::Road;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Place{
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    pub universe_id: ObjectId,
    #[serde_as(as = "DisplayFromStr")]
    pub server_id: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub category_id: u64,
    #[serde_as(as = "DisplayFromStr")]
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

    pub async fn get_roads(self) -> Result<Vec<Road>, mongodb::error::Error>{
        let filter = doc!{
            "$or": [
                doc!{"place_one_id": self.category_id.to_string()},
                doc!{"place_two_id": self.category_id.to_string()}
            ]

        };
        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
        let cursor = db_client.database(self.universe_id.to_string().as_str())
            .collection::<Road>(ROADS_COLLECTION_NAME)
            .find(filter)
            .await;
        cursor.expect("get_roads__collect_failed").try_collect().await
    }
}

pub async fn get_places_by_universe_id(universe_id: String) -> mongodb::error::Result<mongodb::Cursor<Place>> {
    let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
    db_client
        .database(universe_id.as_str())
        .collection::<Place>(PLACES_COLLECTION_NAME)
        .find(doc!{})
        .await
}

pub async fn check_existing_place(universe_id: String, category_id: u64) -> mongodb::error::Result<Option<Place>> {
    let filter = doc!{"category_id": category_id.to_string()};
    let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
    db_client
        .database(universe_id.as_str())
        .collection::<Place>(PLACES_COLLECTION_NAME)
        .find_one(filter)
        .await
}

pub async fn get_place_by_role_id(universe_id: ObjectId, role_id: u64) -> mongodb::error::Result<Option<Place>> {
    let filter = doc!{"role": role_id.to_string()};
    let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
    db_client
        .database(universe_id.to_string().as_str())
        .collection::<Place>(PLACES_COLLECTION_NAME)
        .find_one(filter)
        .await
}

pub async fn get_place_by_category_id(universe_id: ObjectId, category_id: u64) -> mongodb::error::Result<Option<Place>> {
    let filter = doc!{"category_id": category_id.to_string()};
    let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
    db_client
        .database(universe_id.to_string().as_str())
        .collection::<Place>(PLACES_COLLECTION_NAME)
        .find_one(filter)
        .await
}