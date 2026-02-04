use crate::database::server::Server;
use crate::discord::poise_structs::Context;
use crate::setup_command::complementary_setup::complementary_setup;
use crate::setup_command::partial_setup::partial_setup;
use serenity::all::{GuildChannel, Role};
use futures::future::BoxFuture;
use poise::futures_util::future::join_all;

pub async fn full_setup<'a>(ctx: Context<'_>, server: &'a mut Server) -> Result<(&'a str, Vec<Role>, Vec<GuildChannel>), Vec<&'a str>> {
    let partial_setup_result = partial_setup(ctx, server).await;

    if partial_setup_result.is_err() {
        return Err(partial_setup_result.unwrap_err());
    }

    let complementary_setup_result = complementary_setup(ctx, server).await;

    if complementary_setup_result.is_err() {
        let (_, roles, channels) = partial_setup_result.unwrap();

        let role_futures = roles.into_iter().map(|mut role| {
            Box::pin(async move {
                role.delete(ctx).await;
            }) as BoxFuture<'_, ()>
        });

        let channel_futures = channels.into_iter().map(|channel| {
            Box::pin(async move {
                channel.delete(ctx).await;
            }) as BoxFuture<'_, ()>
        });

        let all_futures = role_futures.chain(channel_futures);

        join_all(all_futures).await;

        return Err(complementary_setup_result.unwrap_err());
    }

    let mut roles = partial_setup_result.clone()?.1;
    roles.extend(complementary_setup_result.clone().unwrap().1);
    let mut channels = partial_setup_result.clone()?.2;
    channels.extend(complementary_setup_result.clone().unwrap().2);

    Ok(("setup__setup_success_message", roles, channels))
}