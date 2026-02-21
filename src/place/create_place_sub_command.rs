use serenity::all::{CreateChannel, EditRole, PermissionOverwrite, PermissionOverwriteType, Permissions, RoleId};
use serenity::all::ChannelType::Category;
use crate::database::places::Place;
use crate::database::server::{get_server_by_id, Server};
use crate::discord::poise_structs::{Context, Error};
use crate::utility::reply::reply;

#[poise::command(slash_command, required_permissions= "ADMINISTRATOR", guild_only)]
pub async fn create_place(ctx: Context<'_>, name: String) -> Result<(), Error>{
    ctx.defer().await?;
    let result = _create_place(&ctx, name).await;
    reply(ctx, result).await?;
    Ok(())
}

/// Asynchronously creates a new "place" within the given server context.
///
/// This function performs several steps to create a "place," which consists of:
/// - Creating a new server role.
/// - Creating a new channel category associated with the role.
/// - Ensuring proper permissions and relationships between the role and channel.
/// - Persisting the "place" data in the database.
///
/// If any step fails, the function attempts to roll back changes to leave the server in a consistent state.
///
/// # Arguments
/// - `ctx`: The context of the current operation, used to interact with the server and manage permissions.
/// - `name`: The desired name for the "place" (role and channel).
///
/// # Returns
/// - `Ok(&'static str)`: A success message indicating that the "place" was created successfully.
/// - `Err(Error)`: An error message/code describing why the operation failed.
///
/// # Errors
/// - `"create_place__server_not_found"`: The server was not found in the database.
/// - `"create_place__database_not_found"`: A database issue occurred while fetching the server.
/// - `"create_place__role_not_created"`: The role creation failed in the server.
/// - `"create_place__rollback_complete"`: Rollback successfully completed after a failure.
/// - `"create_role__rollback_failed"`: Rollback of either the role or channel failed.
///
/// # Rollback Behavior
/// - If an error occurs during the creation of the role or the channel:
///   - The function attempts to delete any created roles and channels.
///   - If rollback also fails, an appropriate error describing the failure is returned.
///
/// # Example
/// ```rust
/// let result = _create_place(&ctx, "My New Place".to_string()).await;
/// match result {
///     Ok(success_message) => println!("Success: {}", success_message),
///     Err(error_message) => eprintln!("Error: {}", error_message),
/// }
/// ```
pub async fn _create_place(ctx: &Context<'_>, name: String) -> Result<&'static str, Error>{
    let guild_id = ctx.guild_id().unwrap();
    let result = get_server_by_id(guild_id.get()).await;
    let server = match result {
        Ok(universe_result) => {
            match universe_result{
                None => {return Err("create_place__server_not_found".into())}
                Some(server) => {server}
            }
        }
        Err(_) => {return Err("create_place__database_not_found".into())}
    };

    let new_role = EditRole::new()
        .name(name.clone())
        .position(0)
        .audit_log_reason("Create new place");

    let mut role = match guild_id.create_role(ctx, new_role).await {
        Ok(role) => {role}
        Err(_) => {return Err("create_place__role_not_created".into())}
    };

    let permissions = vec![PermissionOverwrite {
        allow: Permissions::VIEW_CHANNEL
            | Permissions::SEND_MESSAGES
            | Permissions::READ_MESSAGE_HISTORY,
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Role(role.id),
    },
    PermissionOverwrite {
        allow: Permissions::empty(),
        deny: Permissions::VIEW_CHANNEL,
        kind: PermissionOverwriteType::Role(RoleId::new(guild_id.get())),
    }];

    let new_channel = CreateChannel::new(name.clone())
            .kind(Category)
            .permissions(permissions);

    let new_place = match guild_id.create_channel(ctx, new_channel).await {
        Ok(channel) => {channel}
        Err(_) => {
            match role.delete(ctx).await {
                Ok(_) => {return Err("create_place__rollback_complete".into())}
                Err(_) => {return Err("create_role__rollback_failed".into())}
            };
        }
    };

    let place = Place{
        _id: Default::default(),
        universe_id: server.universe_id,
        server_id: server.server_id,
        category_id: new_place.id.get(),
        role: role.id.get(),
        name: new_place.name.clone(),
        modifiers: vec![],
    };

    match place.insert_place().await{
        Ok(_) => {Ok("create_place__success")}
        Err(_) => {
            match role.delete(ctx).await {
                Ok(_) => {}
                Err(_) => {return Err("create_role__rollback_failed".into())}
            };

            match new_place.delete(ctx).await {
                Ok(_) => {Err("create_place__rollback_complete".into())}
                Err(_) => {Err("create_role__rollback_failed".into())}
            }
        }
    }
}