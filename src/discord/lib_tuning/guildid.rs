use serenity::all::{GuildId, Http, Role, RoleId};
use serenity::json::{json, Value};
use crate::discord::lib_tuning::http::HttpRolePositions;

pub trait RolePositions {
    async fn reorder_roles(self,http: impl AsRef<Http>, roles: impl IntoIterator<Item = (RoleId, Option<u64>)>,) -> serenity::Result<Vec<Role>>;
}

impl RolePositions for GuildId {
    /// Reorders the roles in the guild.
    ///
    /// This function updates the positions of the roles within a guild. The new positions are specified
    /// as a collection of tuples where each tuple contains a [`RoleId`] and an optional position index.
    /// A `None` position will maintain the current role's position.
    ///
    /// # Parameters
    /// - `http`: An implementation of [`AsRef<Http>`] required to interact with the Discord API.
    /// - `roles`: An iterable collection of tuples containing:
    ///   - `RoleId`: The identifier of the role.
    ///   - `Option<u64>`: The new position of the role in the guild hierarchy. A value of `None` keeps
    ///     the current position unchanged.
    ///
    /// # Returns
    /// - On success, returns a [`Vec<Role>`] containing the updated roles in the new hierarchy order.
    /// - On failure, returns a [`serenity::Result`] containing the error that occurred.
    ///
    /// # Examples
    /// ```rust
    /// use serenity::model::id::RoleId;
    /// use serenity::prelude::*;
    ///
    /// let guild_id = GuildId(123456789012345678);
    /// let role1_id = RoleId(987654321098765432);
    /// let role2_id = RoleId(876543210987654321);
    ///
    /// let roles = vec![
    ///     (role1_id, Some(1)),
    ///     (role2_id, Some(0)),
    /// ];
    ///
    /// let result = guild_id.reorder_roles(http, roles).await;
    /// match result {
    ///     Ok(updated_roles) => {
    ///         println!("Roles reordered successfully!");
    ///     }
    ///     Err(why) => {
    ///         println!("Error reordering roles: {:?}", why);
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    /// - Requires the bot to have the `MANAGE_ROLES` permission in the guild.
    /// - The position value must be a valid index within the range of available positions.
    /// - Depending on how many roles need reordering, this request may take some time to complete.
    ///
    /// [`AsRef<Http>`]: https://docs.rs/serenity/*/serenity/http/struct.Http.html
    /// [`RoleId`]: https://docs.rs/serenity/*/serenity/model/id/struct.RoleId.html
    /// [`Vec<Role>`]: https://docs.rs/serenity/*/serenity/model/guild/struct.Role.html
    /// [`serenity::Result`]: https://docs.rs/serenity/*/serenity/error/type.Result.html
    #[inline]
    async fn reorder_roles(
        self,
        http: impl AsRef<Http>,
        roles: impl IntoIterator<Item = (RoleId, Option<u64>)>,
    ) -> serenity::Result<Vec<Role>> {
        let items: Value = roles
            .into_iter()
            .map(|(id, index)| {
                json!({
                    "id": id.get(),
                    "position": index
                })
            })
            .collect::<Vec<_>>()
            .into();

        http.as_ref().edit_guild_role_positions(self, &items).await
    }
}
