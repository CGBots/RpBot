use serenity::all::{Http, PartialGuild, Role, RoleId};
use crate::discord::lib_tuning::guildid::RolePositions;

#[allow(unused)]
pub trait EditRolesPositions {
    async fn reorder_roles(&self, http: impl AsRef<Http>, roles: impl IntoIterator<Item = (RoleId, Option<u64>)>, ) -> serenity::Result<Vec<Role>>;
}

impl EditRolesPositions for PartialGuild{
    ///     Reorders the roles of a guild.
    ///
    ///     This function allows you to reorder the roles in a guild by providing new position values. 
    ///     Each role is identified by its `RoleId`, and an optional position (`u64`) can be specified. 
    ///     Roles not included in the input will remain in their current positions.
    ///
    ///     # Parameters
    ///     - `http`: An implementation of `AsRef<Http>` that is used to interact with the Discord HTTP API.
    ///     - `roles`: An iterator of tuples, where each tuple contains:
    ///         * `RoleId`: The ID of the role to reorder.
    ///         * `Option<u64>`: The new position for the role. If `None`, the position will not be changed.
    ///
    ///     # Returns
    ///     - `Result<Vec<Role>>`: On success, returns a vector of updated `Role` objects. If an error occurs, 
    ///       returns a `serenity::Error`.
    ///
    ///     # Errors
    ///     Returns an error if:
    ///     - The HTTP request fails.
    ///     - The bot lacks the required permissions to manage roles in the guild.
    ///     - The roles provided are invalid or in conflict.
    ///
    ///     # Example
    ///     ```rust
    ///     use serenity::model::id::RoleId;
    ///
    ///     let roles = vec![
    ///         (RoleId(123456789012345678), Some(1)),
    ///         (RoleId(234567890123456789), Some(2)),
    ///     ];
    ///
    ///     let result = guild.reorder_roles(http, roles).await;
    ///
    ///     match result {
    ///         Ok(updated_roles) => println!("Roles reordered successfully: {:?}", updated_roles),
    ///         Err(e) => eprintln!("Failed to reorder roles: {:?}", e),
    ///     }
    ///     ```
    #[inline]
    async fn reorder_roles(
        &self,
        http: impl AsRef<Http>,
        roles: impl IntoIterator<Item = (RoleId, Option<u64>)>,
    ) ->  serenity::Result<Vec<Role>>{
        self.id.reorder_roles(http, roles).await
    }
}