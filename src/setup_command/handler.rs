use log::{log, Level};
use poise::{CreateReply};
use serenity::all::{Channel, CreateEmbed, Role};
use serenity::model::Colour;
use tokio::join;
use crate::discord::poise_structs::{Context, Error};
use crate::tr;
use crate::setup_command::complementary_setup::complementary_setup;
use crate::setup_command::partial_setup::partial_setup;

#[derive(Debug, poise::ChoiceParameter)]
pub enum SetupType {
    FullSetup,
    PartialSetup
}

#[poise::command(slash_command, required_permissions = "ADMINISTRATOR", guild_only)]
pub async fn setup(ctx: Context<'_>, setup_type: SetupType) -> Result<(), Error> {
    ctx.defer().await.unwrap();
    
    let result = match setup_type {
        SetupType::FullSetup => { full_setup(ctx).await }
        SetupType::PartialSetup => { partial_setup(ctx).await }
    };

    match result{
        Ok(result) => {
            let message = ctx.reply_builder(
                CreateReply::default()
                    .embed(CreateEmbed::new()
                        .color(Colour::from_rgb(0, 255, 0))
                        .description(tr!(ctx, result.0))
                        .title(tr!(ctx, "setup__setup_success_title")))
            ).reply(true);
            ctx.send(message).await?;
        }
        Err(err) => {
            let errs: Vec<String> = err.iter()
                .map(|error|"► ".to_owned() + &tr!(ctx, error))
                .collect();

            let errors = errs.join("\n");

            let message = ctx.reply_builder(
                CreateReply::default()
                    .embed(CreateEmbed::new()
                        .color(Colour::from_rgb(255, 0, 0))
                        .description(tr!(ctx, "setup__setup_error_message", errors: errors.as_str()))
                        .title(tr!(ctx, "setup__error_title"))
                    )
            ).reply(true);
            let message_result = ctx.send(message).await;
            match message_result {
                Ok(_) => {}
                Err(_) => {
                    log!(Level::Error, "Error while sending error message:\
                     {}", tr!(ctx, "setup__setup_error_message", errors: errors.as_str()));
                }
            }
        }
    }

    Ok(())
}

async fn full_setup(ctx: Context<'_>) -> Result<(&str, Vec<Role>, Vec<Channel>), Vec<&str>> {
    let partial_setup = partial_setup(ctx);
    let complementary_setup = complementary_setup(ctx);
    let (partial_setup_result, complementary_setup_result) = join!(partial_setup, complementary_setup);

    if partial_setup_result.is_err() {
        //TODO rollback du complementary
    }

    if complementary_setup_result.is_err(){
        //TODO rollback du partial
    }

    let roles = partial_setup_result.clone().unwrap().1.extend(complementary_setup_result.clone().unwrap().1);
    let channels = partial_setup_result.clone().unwrap().2.extend(complementary_setup_result.clone().unwrap().2);

    Ok(("", vec![], vec![]))
}