use crate::roads::create_road_sub_command::create_road;
use crate::discord::poise_structs::{Context, Error};

pub mod create_road_sub_command;

#[poise::command(slash_command, subcommands("create_road"), subcommand_required)]
pub async fn road(ctx: Context<'_>) -> Result<(), Error>{
    Ok(())
}
