use crate::add_server_to_universe_command::logic::{add_server_to_universe, check_server_in_universe};
use crate::discord::poise_structs::{Context, Error};
use crate::translation::tr;
use crate::database::universe::Universe;
use poise::CreateReply;
use serenity::all::CreateSelectMenu;
use serenity::all::CreateSelectMenuKind;
use serenity::all::CreateSelectMenuOption;
use serenity::all::{
    ComponentInteractionCollector, ComponentInteractionDataKind, CreateActionRow,
    EditInteractionResponse,
};

#[poise::command(slash_command, required_permissions = "ADMINISTRATOR", guild_only)]
pub async fn add_server(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await.unwrap();

    match check_server_in_universe(ctx.guild_id().unwrap().get()).await {
        Ok(_) => {
            ctx.send(
                CreateReply::default()
                    .content(tr!(ctx, "already_bind"))
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }
        _ => {}
    }

    let universes: Vec<Universe> = Universe::get_creator_universes(ctx.author().id.get()).await;

    if universes.is_empty() {
        ctx.send(
            CreateReply::default()
                .content(tr!(ctx, "universes_unavailable"))
                .ephemeral(true),
            )
            .await?;
        return Ok(())
    }
    
    let mut options = vec![];    
    for universe in &universes {
        options.push(CreateSelectMenuOption::new(
            universe.name.clone(),
            universe.universe_id.to_string().clone(),
        ))
    }

    let action_row = CreateActionRow::SelectMenu(CreateSelectMenu::new(
        "selected_universe",
        CreateSelectMenuKind::String { options },
    ));

    let message = ctx
        .send(
            CreateReply::default()
                .content(tr!(ctx, "choose_universe"))
                .components(vec![action_row])
                .ephemeral(true),
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
                let universe = add_server_to_universe(selected.clone(), ctx.guild_id().unwrap().get()).await?;
                mci.edit_response(
                    ctx.http(),
                    EditInteractionResponse::new()
                        .content(tr!(ctx, "guild_linked", universe_name: universe.name)),
                )
                .await
                .unwrap();
            }
        }
    }

    Ok(())
}
