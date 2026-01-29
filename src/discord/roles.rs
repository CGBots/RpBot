use crate::discord::lib_tuning::guildid::RolePositions;
use lazy_static::lazy_static;
use serenity::all::{Builder, EditRole, GuildId, Permissions, Role, RoleId};
use serenity::model::permissions::{PRESET_GENERAL};
use crate::discord::poise_structs::Context;

lazy_static!(
    pub static ref AdminRolePermissions: Permissions =
        Permissions::ADMINISTRATOR;

    pub static ref ModeratorRolePermissions:Permissions =
        Permissions::empty()
        | PRESET_GENERAL
        | Permissions::VIEW_CHANNEL
        | Permissions::KICK_MEMBERS
        | Permissions::BAN_MEMBERS
        | Permissions::MANAGE_CHANNELS
        | Permissions::STREAM
        | Permissions::MANAGE_MESSAGES
        | Permissions::MUTE_MEMBERS
        | Permissions::DEAFEN_MEMBERS
        | Permissions::MOVE_MEMBERS
        | Permissions::MANAGE_NICKNAMES
        | Permissions::MANAGE_ROLES
        | Permissions::MANAGE_EVENTS
        | Permissions::MANAGE_THREADS
        | Permissions::CREATE_PUBLIC_THREADS
        | Permissions::SEND_MESSAGES_IN_THREADS
        | Permissions::USE_EMBEDDED_ACTIVITIES
        | Permissions::MODERATE_MEMBERS
        | Permissions::USE_SOUNDBOARD
        | Permissions::CREATE_EVENTS
        | Permissions::SEND_POLLS;
    
    pub static ref SpectatorRolePermissions: Permissions = PRESET_GENERAL;

    pub static ref PlayerRolePermissions: Permissions = PRESET_GENERAL;

    pub static ref EveryoneRolePermissions: Permissions = PRESET_GENERAL;
);


pub async fn create_role(ctx: Context<'_>, role_name: String, role_permissions: Permissions) -> serenity::Result<Role> {
    EditRole::new()
        .name(role_name)
        .permissions(role_permissions)
        .execute(ctx, (ctx.guild_id().unwrap(), None)).await
}

pub async fn edit_role_positions(
    ctx: Context<'_>,
    guild_id: GuildId,
    positions: Vec<(RoleId, Option<u64>)>,
) -> serenity::Result<Vec<Role>> {
    guild_id.reorder_roles(ctx, positions).await
}