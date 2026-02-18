use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::database::stats::StatValue;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Modifier{
    pub priority: i32,
    pub stat: ObjectId,
    pub variable_name: String,
    pub value: StatValue,
    pub formula: String,
    pub end_timestamp: Option<u64>,
}

impl Modifier{

}
