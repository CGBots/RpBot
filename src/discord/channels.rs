//! This module provides functions to create and manage permission sets for Discord categories and channels,
//! as well as utilities to create channels with customized configurations, using the Serenity library.

use serenity::all::{ChannelType, CreateChannel, GuildChannel, PermissionOverwrite, PermissionOverwriteType, Permissions, RoleId};
use serenity::builder::Builder;
use crate::discord::poise_structs::Context;


/// Creates a permission set for road categories.
///
/// This function generates permission overwrites where:
/// - `@everyone` and players cannot view the channel
/// - Spectators and moderators can view the channel
///
/// # Arguments
///
/// * `everyone_role_id` - The ID of the @everyone role
/// * `player_role_id` - The ID of the player role
/// * `spectator_role_id` - The ID of the spectator role
/// * `moderator_role_id` - The ID of the moderator role
///
/// # Returns
///
/// A vector of `PermissionOverwrite` objects configured for road categories
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

/// Creates a permission set for administration categories.
///
/// This function generates permission overwrites where:
/// - `@everyone`, spectators, and players cannot view the channel
/// - Only moderators can view the channel
///
/// # Arguments
///
/// * `everyone_role_id` - The ID of the @everyone role
/// * `spectator_role_id` - The ID of the spectator role
/// * `player_role_id` - The ID of the player role
/// * `moderator_role_id` - The ID of the moderator role
///
/// # Returns
///
/// A vector of `PermissionOverwrite` objects configured for admin categories
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

/// Creates a permission set for roleplay character channels.
///
/// This function generates permission overwrites where:
/// - Players cannot view the channel by default
///
/// # Arguments
///
/// * `player_role_id` - The ID of the player role
///
/// # Returns
///
/// A vector of `PermissionOverwrite` objects configured for RP character channels
pub fn get_rp_character_permission_set(player_role_id: RoleId) -> Vec<PermissionOverwrite> {
    vec![
        PermissionOverwrite {
            allow: Permissions::default(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(player_role_id)
        }
    ]
}

/// Creates a new Discord channel with the specified configuration.
///
/// This function creates a channel with custom permissions and optional category assignment.
/// Note: If the channel being created is itself a category, the `category` parameter is ignored.
///
/// # Arguments
///
/// * `ctx` - The Poise context containing HTTP client and guild information
/// * `channel_name` - The name of the channel to create
/// * `channel_type` - The type of channel (text, voice, category, etc.)
/// * `position` - The position of the channel in the channel list
/// * `permissions` - A vector of permission overwrites to apply to the channel
/// * `category` - Optional category ID to place the channel under (ignored if `channel_type` is `Category`)
///
/// # Returns
///
/// A `Result` containing the created `GuildChannel` on success, or a Serenity error on failure
///
/// # Errors
///
/// Returns an error if:
/// - The bot lacks permissions to create channels
/// - The guild ID is not available in the context
/// - The API request fails
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
