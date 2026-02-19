use crate::add_server_to_universe_command::logic::{check_server_in_universe};
use crate::discord::poise_structs::{Context, Error};
use crate::tr;
use crate::database::universe::Universe;
use poise::CreateReply;
use serenity::all::CreateSelectMenu;
use serenity::all::CreateSelectMenuKind;
use serenity::all::CreateSelectMenuOption;
use serenity::all::{ComponentInteractionCollector, ComponentInteractionDataKind, CreateActionRow};
use crate::database::server::Server;
use crate::setup_command::handler::{SetupType, _setup};
use crate::utility::reply::reply;

#[poise::command(slash_command, required_permissions = "ADMINISTRATOR", guild_only)]
pub async fn add_server(ctx: Context<'_>, setup_type: SetupType) -> Result<(), Error> {
    ctx.defer().await?;
    let result = _add_server(&ctx, setup_type).await;
    reply(ctx, result).await?;
    Ok(())
}

pub async fn _add_server(ctx: &Context<'_>, setup_type: SetupType) -> Result<&'static str, Error>{
    if check_server_in_universe(ctx.guild_id().unwrap().get()).await.is_ok() {
        return Ok("add_server_to_universe__already_bind");
    }

    let universes: Vec<Universe> = Universe::get_creator_universes(ctx.author().id.get()).await;

    if universes.is_empty() {
        return Err("add_server_to_universe__universes_unavailable".into());
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
                .content(tr!(*ctx, "choose_universe"))
                .components(vec![action_row])
                .ephemeral(true),
        )
        .await?;

    let serenity_context = ctx.serenity_context();

    let result = while let Some(mci) = ComponentInteractionCollector::new(serenity_context)
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == "selected_universe")
        .await
    {
        if let ComponentInteractionDataKind::StringSelect { values } = &mci.data.kind {
            if let Some(selected) = values.get(0) {
                let _ = message.delete(*ctx).await;

                let Some(universe) = Universe::get_universe_by_id(selected.to_string()).await? else {return Err("placeholder".into())};

                let res = universe.clone().check_server_limit().await?;

                if !res{
                    return Err("exceed_limit_number_of_servers_per_universe".into())
                }

                let server = Server{
                    _id: Default::default(),
                    universe_id: universe.universe_id,
                    server_id: ctx.guild_id().unwrap().get(),
                    admin_role_id: Default::default(),
                    moderator_role_id: Default::default(),
                    spectator_role_id: Default::default(),
                    player_role_id: Default::default(),
                    everyone_role_id: Default::default(),
                    admin_category_id: Default::default(),
                    nrp_category_id: Default::default(),
                    rp_category_id: Default::default(),
                    road_category_id: Default::default(),
                    rp_wiki_channel_id: Default::default(),
                    log_channel_id: Default::default(),
                    moderation_channel_id: Default::default(),
                    commands_channel_id: Default::default(),
                    nrp_general_channel_id: Default::default(),
                    rp_character_channel_id: Default::default(),
                }.insert_server().await?;
                _setup(&ctx, setup_type).await?;

                return Ok("add_server_to_universe__guild_linked");
            }
        }
    };

    Ok("")
}
