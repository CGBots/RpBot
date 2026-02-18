use crate::database::universe::Universe;
use crate::discord::poise_structs::{Context, Error};

#[poise::command(slash_command, subcommands("create_place"), subcommand_required)]
pub async fn place(ctx: Context<'_>) -> Result<(), Error>{
    Ok(())
}

#[poise::command(slash_command, required_permissions= "ADMINISTRATOR", guild_only)]
pub async fn create_place(ctx: Context<'_>, name: String) -> Result<(), Error>{
    //check if universe and server are registered
    //if yes
    //  create the role for the channel
    //  create the new channel with the name
    //if succeed register the new place on the database
    //if something fail remove created role and/or category from discord
    let guild_id = ctx.guild_id().unwrap();
    let result = Universe::get_universe_by_server_id(guild_id.get()).await;
    match result {
        Ok(universe_result) => {
            match universe_result{
                None => {/*erreur, univers pas trouvé*/}
                Some(universe) => {/*univers*/}
            }
        }
        Err(_) => { /*erreur, base de donnée innaccessible*/} 
    }
    
    

    Ok(())
}