use serenity::all::{ChannelType, CreateChannel, GuildChannel, PermissionOverwrite, PermissionOverwriteType, Permissions, RoleId};
use serenity::builder::Builder;
use crate::discord::poise_structs::Context;

pub fn get_road_category_permission_set(everyone_role_id: RoleId, player_role_id: RoleId, spectator_role_id: RoleId, moderator_role_id: RoleId) -> Vec<PermissionOverwrite> {
    vec![
        PermissionOverwrite {
            allow: Permissions::default(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(player_role_id)
        },
        PermissionOverwrite {
            allow: Permissions::default(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(everyone_role_id)
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::default(),
            kind: PermissionOverwriteType::Role(spectator_role_id)
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::default(),
            kind: PermissionOverwriteType::Role(moderator_role_id)
        }
    ]
}

pub fn get_admin_category_permission_set(everyone_role_id: RoleId, spectator_role_id: RoleId, player_role_id: RoleId, moderator_role_id: RoleId) -> Vec<PermissionOverwrite>{
    vec ! [
        PermissionOverwrite{
            allow: Permissions::default(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(everyone_role_id)
        },
        PermissionOverwrite{
            allow: Permissions::default(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(spectator_role_id)
        },
        PermissionOverwrite{
            allow: Permissions::default(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(player_role_id)
        },
        PermissionOverwrite{
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::default(),
            kind: PermissionOverwriteType::Role(moderator_role_id)
        }
    ]
}

pub fn get_rp_character_permission_set(player_role_id: RoleId) -> Vec<PermissionOverwrite>{
    vec ! [
        PermissionOverwrite{
            allow: Permissions::default(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(player_role_id)
        }
    ]
}

//TODO
// Documentation
// ATTENTION ignore la catégorie si le channel créé est lui-même une catégorie.
pub async fn create_channel(ctx: Context<'_>, channel_name: String, channel_type: ChannelType, position: u16, permissions: Vec<PermissionOverwrite>, category: Option<u64>) -> serenity::Result<GuildChannel> {
    let mut channel = CreateChannel::new(channel_name)
        .kind(channel_type)
        .position(position)
        .permissions(permissions);

    if channel_type != ChannelType::Category {
        if let Some(cat) = category {
            channel = channel.category(cat);
        }
    }
    
    channel.execute(ctx.http(), ctx.guild_id().unwrap()).await
}
