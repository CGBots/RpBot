use serde_with::DisplayFromStr;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::database::stats::Stat;

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    #[serde(rename = "_id")]
    _id: ObjectId,
    #[serde_as(as = "DisplayFromStr")]
    user_id: u64,
    universe_id: ObjectId,
    name: String,
    stats: Vec<Stat>
}

