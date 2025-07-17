use serenity::all::{ComponentInteractionCollector, ComponentInteractionDataKind, CreateActionRow, EditInteractionResponse};
use serenity::all::CreateSelectMenu;
use serenity::all::CreateSelectMenuKind;
use serenity::all::CreateSelectMenuOption;
use poise::{CreateReply};
use crate::discord::poise_structs::{Context, Error};
use crate::universe::Universe;
use futures::stream::TryStreamExt;

//TODO separer la business logic de l'hadler.

#[poise::command(slash_command, required_permissions= "ADMINISTRATOR", guild_only)]
pub async fn add_server(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.defer_ephemeral().await.unwrap();

    if let Ok(mut cursor) = Universe::get_universe_by_server_id(ctx.guild_id().unwrap().get()).await {
        if let Some(universe) = cursor.try_next().await? {
            ctx.send(
                CreateReply::default()
                    .content("Le serveur fais déjà parti d'un univers.")
                    .ephemeral(true)
            ).await?;
            return Ok(());
        }
    }

    let universes: Vec<Universe> = Universe::get_creator_universes(ctx.author().id.get()).await.unwrap().try_collect().await?;


    let mut options = vec![];
    for universe in universes {
        options.push(CreateSelectMenuOption::new(universe.name.clone(), universe.universe_id.to_string().clone()))
    }

    let action_row = CreateActionRow::SelectMenu(
        CreateSelectMenu::new(
            "selected_universe",
            CreateSelectMenuKind::String { options }
        )
    );

    let message = ctx.send(
        CreateReply::default()
            .content("test de select menu")
            .components(vec![action_row])
            .ephemeral(true)
    )
    .await?;



    while let Some(mci) = ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == "selected_universe")
        .await
    {

        if let ComponentInteractionDataKind::StringSelect { values } = &mci.data.kind {
            if let Some(selected) = values.get(0) {
                message.delete(ctx).await.unwrap_or_default();
                mci.defer_ephemeral(ctx.http()).await.unwrap_or_default();
                Universe::get_universe_by_id(selected.to_string()).await.unwrap().unwrap().add_server_to_universe(ctx.guild_id().unwrap().get()).await.unwrap();
                mci.edit_response(ctx.http(), EditInteractionResponse::new().content("Le serveur fais maintenant partie de l'univers.")).await.unwrap();
            }
        }
    }

    Ok(())
}