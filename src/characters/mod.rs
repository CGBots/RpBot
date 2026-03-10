pub mod create_character_sub_command;

use crate::characters::create_character_sub_command::create_character;
use crate::discord::poise_structs::{Context, Error};

#[poise::command(slash_command, subcommands("create_character"), subcommand_required, rename = "character")]
pub async fn character(_ctx: Context<'_>) -> Result<(), Error>{
    Ok(())
}
