use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::database::db_client::{connect_db, DB_CLIENT};
use crate::database::db_namespace::STATS_COLLECTION_NAME;
use crate::database::modifiers::Modifier;
use crate::discord::poise_structs::Error;

pub static SPEED_STAT: &str = "speed";

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub enum StatValue {
    Int(u32),
    Float(f32),
    Text(String),
    Bool(bool)
}

impl StatValue {
    pub fn to_dynamic(&self) -> rhai::Dynamic {
        match self {
            StatValue::Int(v) => rhai::Dynamic::from(*v as i64),
            StatValue::Float(v) => rhai::Dynamic::from(*v as f64),
            StatValue::Text(v) => rhai::Dynamic::from(v.clone()),
            StatValue::Bool(v) => rhai::Dynamic::from(*v),
        }
    }
}


#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stat {
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    pub universe_id: ObjectId,
    pub name: String,
    pub base_value: StatValue,
    pub formula: Option<String>,
    pub min: Option<StatValue>,
    pub max: Option<StatValue>,
    pub modifiers: Vec<Modifier>
}

impl Stat {
    pub async fn insert_stat(&self) -> Result<Stat, Error>{
        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
        let result = db_client
            .database(&*self.universe_id.to_string())
            .collection::<Stat>(STATS_COLLECTION_NAME)
            .insert_one(self)
            .await;
        match result {
            Ok(_) => {
                Ok(self.clone()) }
            Err(_) => { Err("stat_insert__failed".into()) }
        }
    }

    pub async fn get_stat_by_name(universe_id: &str, name: &str) -> mongodb::error::Result<Option<Stat>> {
        let db_client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await.clone();
        let filter = doc! { "name": name };
        db_client
            .database(universe_id)
            .collection::<Stat>(STATS_COLLECTION_NAME)
            .find_one(filter)
            .await
    }
    pub fn is_within_bounds(&self) -> bool {
        if let Some(min) = &self.min {
            if self.base_value < *min {
                return false;
            }
        }
        if let Some(max) = &self.max {
            if self.base_value > *max {
                return false;
            }
        }
        true
    }
}

