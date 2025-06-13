mod ping_command;
mod translation;
mod create_universe_command;
mod database;
mod discord;
mod bson_modifiers;

use discord::poise_structs::{Context, Data, Error};
use crate::database::db_client::DB_CLIENT;
use crate::discord::connect_bot::connect_bot;


#[tokio::main(flavor= "multi_thread")]
async fn main() {
    DB_CLIENT.lock().unwrap().connect_db().await;
    let _ = connect_bot().await;
}


#[cfg(test)]
mod tests {
    use multilateration::{multilaterate, Measurement, Point};

    #[test]
    fn test_multilateration() {
        let measurements = vec![
            Measurement::new(Point(vec![0.0, 0.0, 1.0, 1.3]), 3.0),
            Measurement::new(Point(vec![0.0, 3.0, 1.0, 1.2]), 3.0),
            Measurement::new(Point(vec![3.0, 0.0, 1.0, 1.2]), 3.0),
        ];

        let coordinates = multilaterate(measurements).unwrap().0;
        println!("Coordinates are: {:?}", coordinates);

    }
}