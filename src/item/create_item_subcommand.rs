use futures::TryStreamExt;
use crate::database::items::Item;
use poise::{ChoiceParameter, CreateReply};
use serenity::all::{Attachment, CreateEmbed, CreateForumPost, CreateForumTag, CreateMessage};
use crate::database::server::{get_server_by_id, Server};
use crate::database::stats::Stat;
use crate::discord::channels::{ITEM_TAG, PLACE_TAG};
use crate::discord::poise_structs::{Context, Error};
use crate::item::ItemUsage;
use crate::tr;
use crate::utility::reply::reply;

#[poise::command(slash_command, guild_only, required_permissions= "ADMINISTRATOR")]
pub async fn create(
    ctx: Context<'_>,
    name: String,
    usage: ItemUsage,
    image: Option<Attachment>,
    description: Option<String>,
    into_wiki: bool,
) -> Result<(), Error> {
    let url = match image{
        None => {"".into()}
        Some(image) => {image.url}
    };
    let description = description.clone().unwrap_or("".to_string());

    let Some(server) = get_server_by_id(ctx.guild_id().unwrap().get()).await? else {return Err("item__server_not_found".into())};

    let embed = CreateEmbed::new()
        .title(name.clone())
        .description(description.clone())
        .field(tr!(ctx.clone(), "item_usage_title"), usage.name(), true)
        .thumbnail(url.clone());

    if into_wiki {
        let Ok(servers_cursor) = server.get_other_servers().await else {return Err("item_db_error".into())};
        let servers = servers_cursor.try_collect::<Vec<Server>>().await.unwrap();

        for server in servers{
            if let Some(wiki_channel_id) = server.rp_wiki_channel_id{
                let Ok(wiki_channel) = ctx.http().get_channel(wiki_channel_id.id.into()).await else {continue};
                let channel = wiki_channel.guild().unwrap().clone();
                let Some(item_tag) = channel.available_tags.iter().find(|tag| tag.name == ITEM_TAG) else {continue};
                let _ = channel.create_forum_post(ctx, CreateForumPost::new(name.clone(), CreateMessage::new().embed(embed.clone())).add_applied_tag(item_tag.id)).await?;
            }
        }
    };

    let result = Item{
        _id: Default::default(),
        universe_id: server.universe_id,
        item_name: name.clone(),
        item_usage: usage.clone(),
        effects: vec![],
        description: description.clone(),
        image: url.clone(),
        wiki_post_id: None,
    }.save().await;

    match result{
        Ok(_) => {}
        Err(_) => { let _ = reply(ctx, Err("create_item__db_error".into())).await; return Ok(()) }
    }



    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}