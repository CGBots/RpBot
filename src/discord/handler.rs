use poise::async_trait;
use serenity::all::{EventHandler, Ready};
use serenity::client::Context;
#[cfg(test)] use crate::discord::connect_bot::TEST_PASSED;

#[cfg(not(test))] use std::ops::Add;
#[cfg(not(test))] use serenity::all::ActivityData;
    

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    #[cfg(not(test))]
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        ctx.shard.set_activity(Some(ActivityData::listening("shard ".to_string().add(&ctx.shard_id.0.to_string()))));
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