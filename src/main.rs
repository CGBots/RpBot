mod ping_command;
mod translation;
mod database;
mod discord;
mod bson_modifiers;
mod start_command;
pub mod add_server_to_universe_command;
pub mod create_universe_command;
pub mod setup_command;

use discord::poise_structs::{Context, Data, Error};
use crate::discord::connect_bot::connect_bot;


#[tokio::main(flavor= "multi_thread")]
async fn main() {
    let _ = crate::database::db_client::DB_CLIENT.get_or_init(|| async {
        crate::database::db_client::connect_db().await.expect("Failed to connect to database")
    }).await;
    let _ = connect_bot().await;
}