use serenity::all::{Channel, Role};
use crate::discord::poise_structs::Context;

pub async fn complementary_setup(ctx: Context<'_>) -> Result<(&str, Vec<Role>, Vec<Channel>), Vec<&str>> {
    // TODO Mode Complet:
    //  + admin_category_id
    //    + Moderation
    //    + Commandes
    //    + Logs
    //    + Discussions
    //  + nrp_category_id
    //  + rp_category_id
    //    + character_channel_id
    //    + wiki_forum_id


    Ok(("", vec![], vec![]))
}