use poise::{async_trait};
#[allow(unused_imports)]
use serenity::all::{ChannelType, CreateChannel, Context, Guild, Ready};
use poise::serenity_prelude::{EventHandler};
#[cfg(test)] use crate::discord::connect_bot::TEST_PASSED;

#[allow(unused_imports)]
#[cfg(not(test))] use std::ops::Add;
#[allow(unused_imports)]
#[cfg(not(test))] use serenity::all::ActivityData;
#[allow(unused_imports)]
use crate::translation::{apply_translations, tr};

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: Option<bool>) {
        guild.create_channel(ctx.http, CreateChannel::new("README").kind(ChannelType::Text))
            .await
            .unwrap();
        //trigger the /start command here without mocking anything, only using the serenity context
    }

    #[cfg(test)]
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        match TEST_PASSED.lock(){
            Ok(mut mutex) => {mutex.push_front(true)}
            Err(e) => {println!("{:?}", e)}
        }
    }
}