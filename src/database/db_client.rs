//! Provides a global MongoDB client for the RPBot system.
//!
//! This module exposes a lazily initialized, thread-safe MongoDB client
//! accessible globally via [`DB_CLIENT`]. It ensures that only one connection
//! is created and shared throughout the entire application.
//!
//! # Usage
//! ```no_run
//! use crate::database::db_client::DB_CLIENT;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Initialize the global database client
//!     DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await;
//!
//!     // Access collections through the client
//!     let db = DB_CLIENT.get().unwrap().database("test_db");
//!     println!("{:?}", db.list_collection_names().await.unwrap());
//! }
//! ```
#![allow(unused_doc_comments)]
use std::env;
use mongodb::bson::doc;
use mongodb::IndexModel;
use mongodb::options::IndexOptions;
use tokio::sync::OnceCell;
use urlencoding::encode;
use crate::database::db_namespace::{RPBOT_DB_NAME, SERVER_COLLECTION_NAME};
use crate::database::server::Server;

/// Connects to MongoDB and returns a client.
///
/// This method reads the environment variables `MONGODB_USER`
/// and `MONGODB_PASSWORD` to authenticate.
///
/// # Environment Variables
/// - `MONGODB_USER`: MongoDB username
/// - `MONGODB_PASSWORD`: MongoDB password
///
/// # Errors
/// Returns a [`mongodb::error::Error`] if the connection fails.
pub async fn connect_db() -> Result<mongodb::Client, mongodb::error::Error> {
    let user = env::var("MONGODB_USER").expect("Expected a database user in the environment");
    let user = encode(&user);
    let password = env::var("MONGODB_PASSWORD").expect("Expected a database password in the environment");
    let password = encode(&password);
    let url = format!("mongodb://{user}:{password}@localhost:27017/?authSource=admin");
    mongodb::Client::with_uri_str(url).await
}

pub async fn constraint(){
    let db_client = DB_CLIENT .get_or_init(|| async { connect_db().await.unwrap() }) .await .clone();
    let index_keys = doc! {"server_id": 1};
    let index_options = IndexOptions::builder().unique(true).build();
    let index_model = IndexModel::builder()
        .keys(index_keys)
        .options(index_options)
        .build();
    let result_server_index = db_client
        .database(RPBOT_DB_NAME)
        .collection::<Server>(SERVER_COLLECTION_NAME)
        .create_index(index_model)
        .await;
}

/// Lazily initialized, thread-safe global MongoDB client.
///
/// Access this object globally to perform database operations.
///
/// # Example
/// ```no_run
/// use crate::database::db_client::{DB_CLIENT, connect_db};
///
/// async fn init() {
///     DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await;
/// }
/// ```
pub static DB_CLIENT: OnceCell<mongodb::Client> = OnceCell::const_new();

#[cfg(test)]
mod test {
    use crate::database::db_client::{DB_CLIENT, connect_db};

    /// Ensures that the database connection initializes correctly.
    #[tokio::test]
    async fn test_connect_db() {
        let client = DB_CLIENT.get_or_init(|| async { connect_db().await.unwrap() }).await;
        match client.database("test").list_collection_names().await {
            Ok(_) => {assert!(true)}
            Err(_) => {assert!(false)}
        };
    }
}