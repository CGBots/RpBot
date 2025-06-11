use std::env;
use std::ops::Deref;
use std::sync::Mutex;
use urlencoding::encode;

#[derive(Clone)]
pub struct Client{
    inner: Option<mongodb::Client>
}

impl Deref for Client {
    type Target = Option<mongodb::Client>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Client {
    const fn new(client: Option<mongodb::Client>) -> Self {
        Client{ inner: client }
    }
}

struct DbClient {
    inner: Mutex<Client>
}

impl DbClient {
    const fn new() -> Self {
        DbClient{
            inner: Mutex::new(Client::new(None)),
        }
    }
    
    pub(crate) async fn connect_db(&mut self) -> Result<(), mongodb::error::Error>{
        let user = env::var("MongoDbUser").expect("Expected a database user in the environment");
        let user = encode(&user);
        let password = env::var("MongoDbPassword").expect("Expected a database password in the environment");
        let password = encode(&password);
        let url = format!("mongodb://{user}:{password}@localhost:27017/?authSource=admin");
        match mongodb::Client::with_uri_str(url).await {
            Ok(client) => { self.inner = Mutex::new(Client::new(Some(client))); Ok(())}
            Err(e) => { panic!("MongoDB connection error: {e}"); }
        }
    }

    fn get_client_guard(&self) -> Client {
        self.inner.lock().unwrap().clone()
    }
}

// Implement Deref but return the **MutexGuard**
impl Deref for DbClient {
    type Target = mongodb::Client;
    fn deref(&self) -> &Self::Target {
        self.inner.lock().unwrap().inner.as_ref().unwrap()
    }
}

pub static DB_CLIENT: DbClient = DbClient::new();

#[cfg(test)]
mod test {
    use crate::database::db_client::{DB_CLIENT};

    #[tokio::test]
    async fn test_connect_db(){
        match DB_CLIENT.database("test").list_collection_names().await {
            Ok(_) => {assert!(true)}
            Err(_) => {assert!(false)}
        };
    }
}