use std::env;
use std::ops::Deref;
use std::sync::{Mutex};
use lazy_static::lazy_static;
use urlencoding::encode;

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

pub struct DbClient {
    inner: Option<Client>
}

impl Deref for DbClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        match &self.inner {
            None => {panic!("Database not initialized")}
            Some(client) => {&client}
        }
    }
}

impl DbClient {
    fn new() -> Self{
        DbClient{inner: None}
    }

    /// To connect the database and create the unique client, just use the following line
    /// `DB_CLIENT.lock().unwrap().connect_db().await.unwrap();`
    /// DB_CLIENT should be initialised and can be accessed everywhere.
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

lazy_static!{
    pub static ref DB_CLIENT: Mutex<DbClient> = Mutex::new(DbClient::new());
}


#[cfg(test)]
mod test {
    use crate::database::db_client::{DB_CLIENT};

    #[tokio::test]
    async fn test_connect_db(){
        DB_CLIENT.lock().unwrap().connect_db().await.unwrap();
        match DB_CLIENT.lock().unwrap().database("test").list_collection_names().await {
            Ok(_) => {assert!(true)}
            Err(_) => {assert!(false)}
        };
    }
}