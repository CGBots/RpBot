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

/// Asynchronously creates a new role in the guild with the specified name and permissions.
///
/// # Arguments
///
/// * `ctx` - The context within which the command is executed. This includes information such as the current guild
///   and available API resources.
/// * `role_name` - A `String` representing the desired name for the new role.
/// * `role_permissions` - A `Permissions` object specifying the permissions to be assigned to the new role.
///
/// # Returns
///
/// This function returns a `Result<Role>` containing the newly created `Role` object on success. If an error occurs,
/// it returns a `serenity::Error` encapsulated within the `Result`.
///
/// # Errors
///
/// This function can return errors in the following scenarios:
/// - The guild could not be identified or retrieved from the `ctx`.
/// - An API call to Discord fails during the role creation or configuration process.
///
/// # Example
///
/// ```rust
/// use serenity::model::permissions::Permissions;
/// use serenity::prelude::Context;
///
/// async fn example(ctx: &Context<'_>) -> serenity::Result<()> {
///     let role_name = "Moderator".to_string();
///     let permissions = Permissions::ADMINISTRATOR;
///
///     let new_role = create_role(ctx, role_name, permissions).await?;
///     println!("Created role: {}", new_role.name);
///
///     Ok(())
/// }
/// ```
///
/// # Notes
/// - The guild ID is retrieved from the context, so this function assumes the context is tied to a specific guild.
/// - Ensure the bot has sufficient permissions, such as the `MANAGE_ROLES` permission, to create roles in the guild.
pub async fn create_role(ctx: &Context<'_>, role_name: String, role_permissions: Permissions) -> serenity::Result<Role> {
    EditRole::new()
        .name(role_name)
        .permissions(role_permissions)
        .execute(ctx, (ctx.guild_id().unwrap(), None)).await
}

/// Edits the positions of roles in a guild.
///
/// This function allows you to reorder roles within a guild by specifying their new
/// positions. Each role is identified by its `RoleId`, and its new position is
/// specified as an `Option<u64>`. The roles' positions are adjusted accordingly.
///
/// # Parameters
/// - `ctx`: A reference to the [`Context`] which provides the framework's context, 
///   including the cache and HTTP client.
/// - `guild_id`: The ID of the guild where the roles are being reordered.
/// - `positions`: A vector of tuples where each tuple consists of a [`RoleId`] 
///   and an `Option<u64>`. The `RoleId` identifies the role, and the `Option<u64>`
///   specifies the new position of the role. If `None` is provided, the position
///   will remain unchanged for that role.
///
/// # Returns
/// - On success, it returns a `Vec<Role>` containing the updated roles in
///   the guild, reflecting their new positions.
/// - On failure, it returns a [`serenity::Result`] containing an
///   error indicating what went wrong.
///
/// # Examples
/// ```rust
/// use serenity::model::id::{GuildId, RoleId};
/// use serenity::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> serenity::Result<()> {
///     let ctx = // Obtain the context, e.g., from an event handler.
///     let guild_id = GuildId(123456789012345678);
///     let roles_positions = vec![
///         (RoleId(111111111111111111), Some(1)),
///         (RoleId(222222222222222222), Some(2)),
///         (RoleId(333333333333333333), Some(0)), // Assign this role the lowest position.
///     ];
///
///     let updated_roles = edit_role_positions(&ctx, guild_id, roles_positions).await?;
///     println!("Updated roles: {:?}", updated_roles);
///
///     Ok(())
/// }
/// ```
///
/// # Errors
/// This function might fail if:
/// - The bot lacks the necessary permissions to manage roles in the guild.
/// - The provided `positions` vector contains invalid data (e.g., roles that do not
///   exist in the guild).
/// - A network error occurs while communicating with Discord's API.
///
/// # Notes
/// - The bot must have the "Manage Roles" permission in the guild to successfully
///   reorder roles.
/// - The position of a role decides its order in the role list and can affect permissions
///   when role hierarchies are involved.
///
/// [`Context`]: serenity::prelude::Context
/// [`RoleId`]: serenity::model::id::RoleId
/// [`serenity::Result`]: serenity::Result
pub async fn edit_role_positions(
    ctx: &Context<'_>,
    guild_id: GuildId,
    positions: Vec<(RoleId, Option<u64>)>,
) -> serenity::Result<Vec<Role>> {
    guild_id.reorder_roles(ctx, positions).await
}