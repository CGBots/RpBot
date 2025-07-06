mod ping_command;
mod translation;
mod create_universe_command;
mod database;
mod discord;
mod bson_modifiers;
mod setup_command;

use discord::poise_structs::{Context, Data, Error};
use crate::database::db_client::DB_CLIENT;
use crate::discord::connect_bot::connect_bot;


#[tokio::main(flavor= "multi_thread")]
async fn main() {
    let _ = DB_CLIENT.lock().unwrap().connect_db().await;
    let _ = connect_bot().await;
}