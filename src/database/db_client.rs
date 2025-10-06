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
//!     DB_CLIENT.lock().unwrap().connect_db().await.unwrap();
//!
//!     // Access collections through the client
//!     let db = DB_CLIENT.lock().unwrap().database("test_db");
//!     println!("{:?}", db.list_collection_names().await.unwrap());
//! }
//! ```
#![allow(unused_doc_comments)]
use std::env;
use std::ops::Deref;
use std::sync::{Mutex};
use lazy_static::lazy_static;
use urlencoding::encode;

/// A wrapper around [`mongodb::Client`] providing `Clone` support.
///
/// The native MongoDB client is `Clone`, but this wrapper ensures
/// a consistent interface with our [`DbClient`] struct.
#[derive(Clone)]
pub struct Client{
    inner: mongodb::Client
}


impl Deref for Client {
    type Target = mongodb::Client;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Represents the global MongoDB client instance.
///
/// This struct wraps the `mongodb::Client` inside an `Option`
/// to support delayed initialization (lazy connection).
///
/// Access it globally through [`DB_CLIENT`].
pub struct DbClient {
    inner: Option<Client>
}

impl Deref for DbClient {
    type Target = Client;

    /// Returns a reference to the inner MongoDB client.
    ///
    /// # Panics
    /// Panics if the client has not been initialized
    /// by calling [`DbClient::connect_db`].
    fn deref(&self) -> &Self::Target {
        match &self.inner {
            None => {panic!("Database not initialized")}
            Some(client) => {&client}
        }
    }
}

impl DbClient {
    /// Creates an uninitialized [`DbClient`].
    fn new() -> Self{
        DbClient{inner: None}
    }

    /// Connects to MongoDB and initializes the global client.
    ///
    /// This method reads the environment variables `MONGODB_USER`
    /// and `MONGODB_PASSWORD` to authenticate.
    ///
    /// # Environment Variables
    /// - `MONGODB_USER`: MongoDB username
    /// - `MONGODB_PASSWORD`: MongoDB password
    ///
    /// # Example
    /// ```no_run
    /// use crate::database::db_client::DB_CLIENT;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     DB_CLIENT.lock().unwrap().connect_db().await.unwrap();
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns a [`mongodb::error::Error`] if the connection fails.
    pub async fn connect_db(&mut self) -> Result<(), mongodb::error::Error>{
        let user = env::var("MONGODB_USER").expect("Expected a database user in the environment");
        let user = encode(&user);
        let password = env::var("MONGODB_PASSWORD").expect("Expected a database password in the environment");
        let password = encode(&password);
        let url = format!("mongodb://{user}:{password}@localhost:27017/?authSource=admin");
        match mongodb::Client::with_uri_str(url).await {
            Ok(client) => { self.inner = Some(Client{inner: client}); Ok(()) },
            Err(e) => { Err(e) }
        }
    }
}


/// Lazily initialized, thread-safe global MongoDB client.
///
/// Access this object globally to perform database operations.
///
/// # Example
/// ```no_run
/// use crate::database::db_client::DB_CLIENT;
///
/// async fn init() {
///     DB_CLIENT.lock().unwrap().connect_db().await.unwrap();
/// }
/// ```
lazy_static!{
    pub static ref DB_CLIENT: Mutex<DbClient> = Mutex::new(DbClient::new());
}


#[cfg(test)]
mod test {
    use crate::database::db_client::{DB_CLIENT};

    /// Ensures that the database connection initializes correctly.
    #[tokio::test]
    async fn test_connect_db(){
        DB_CLIENT.lock().unwrap().connect_db().await.unwrap();
        match DB_CLIENT.lock().unwrap().database("test").list_collection_names().await {
            Ok(_) => {assert!(true)}
            Err(_) => {assert!(false)}
        };
    }
}