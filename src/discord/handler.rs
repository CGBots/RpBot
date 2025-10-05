use poise::{async_trait, CreateReply};
use serenity::all::{ChannelType, CreateChannel, Context, CreateMessage, Guild, GuildChannel, Ready};
use poise::serenity_prelude::{EventHandler};
use crate::discord::poise_structs::*;
#[cfg(test)] use crate::discord::connect_bot::TEST_PASSED;

#[cfg(not(test))] use std::ops::Add;
#[cfg(not(test))] use serenity::all::ActivityData;
use crate::translation::{apply_translations, tr};

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: Option<bool>) {
        let channel = guild.create_channel(ctx.http, CreateChannel::new("README").kind(ChannelType::Text))
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