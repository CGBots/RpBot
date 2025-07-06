use std::collections::VecDeque;
use std::{env};
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as Mut;
use std::time::Duration;
use serenity::all::GatewayIntents;
use serenity::Client;
use serenity::client::ClientBuilder;
use crate::{translation};
use crate::create_universe_command::handler::create_universe;
use crate::discord::handler::Handler;
use crate::ping_command::handler::ping;
use crate::discord::poise_structs::Data;

#[cfg(not(test))]
static SHARD_NUMBER: u32 = 1;

#[cfg(test)]
pub(crate) static TEST_PASSED: Mutex<VecDeque<bool>> = Mutex::new(VecDeque::new());

pub async fn connect_bot() -> Result<Client, ()>{
    tracing_subscriber::fmt::init();
    let mut commands= vec![ping(), create_universe()];
    let translations = translation::read_ftl().expect("failed to read translation files");
    translation::apply_translations(&translations, &mut commands);
    
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {translations})
            })
        })
        .build();


    #[cfg(test)]
    #[allow(unused_results)]
    {
        let client = Arc::new(Mut::new(
            ClientBuilder::new(token, intents)
                .framework(framework)
                .event_handler(Handler)
                .await
                .expect("Err creating client"),
        ));

        TEST_PASSED.lock().unwrap().push_back(false);
        println!("start shards");

        let client_clone = Arc::clone(&client);

        tokio::spawn(async move {
            let client = client_clone.lock().await.start_shard(0, 1).await;
            if let Err(why) = client {
                println!("Client error: {why:?}");
            }
        });

        tokio::time::sleep(Duration::from_secs(3)).await; // Use async sleep
        Err(())
    }

    #[cfg(not(test))]
    {
        let mut client = ClientBuilder::new(token, intents)
                .framework(framework)
                .event_handler(Handler)
                .await
                .expect("Err creating client");
        
        if let Err(why) = client.start_shards(SHARD_NUMBER).await {
            println!("Client error: {why:?}");
        }
        Ok(client)
    }

}

#[cfg(test)]
mod test {
    use crate::discord::connect_bot::{connect_bot, TEST_PASSED};

    #[tokio::test]
    async fn test_discord_bot_connection(){
        let _ = connect_bot().await;
        assert_eq!(TEST_PASSED.try_lock().unwrap().pop_front().unwrap(), true);
    }
}