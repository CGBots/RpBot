use serenity::all::{GuildId, Http, LightMethod, Request, Role, Route};
use serenity::json::{to_vec, Value};

pub(crate) trait HttpRolePositions {
    async fn edit_guild_role_positions(
        &self,
        guild_id: GuildId,
        value: &Value,
    ) -> serenity::Result<Vec<Role>>;
}

impl HttpRolePositions for Http {
    /// Edits the positions of roles in a guild.
    ///
    /// This function allows modifying the position of multiple roles within a guild.
    /// The new role positions should be provided as a JSON value, passed in the `value` parameter,
    /// following the expected structure for role position updates.
    ///
    /// # Parameters
    /// - `guild_id`: The ID of the guild in which the role positions are to be updated.
    /// - `value`: A reference to a `Value` containing the new role positions in the correct structure.
    ///
    /// # Returns
    /// - On success: A `Result` containing a `Vec<Role>` representing the updated roles with their new positions.
    /// - On failure: A `Result` containing an error of type `serenity::Error`.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The provided JSON structure in `value` is invalid or not serializable.
    /// - The HTTP request to the Discord API fails.
    /// - The response from the Discord API cannot be parsed into a `Vec<Role>`.
    ///
    /// # Examples
    /// ```rust
    /// use serenity::model::id::GuildId;
    /// use serde_json::json;
    ///
    /// let guild_id = GuildId(123456789012345678);
    /// let new_positions = json!([
    ///     { "id": "987654321098765432", "position": 2 },
    ///     { "id": "876543210987654321", "position": 1 }
    /// ]);
    ///
    /// let result = client.edit_guild_role_positions(guild_id, &new_positions).await;
    ///
    /// match result {
    ///     Ok(updated_roles) => println!("Role positions updated successfully: {:?}", updated_roles),
    ///     Err(err) => eprintln!("Failed to update role positions: {:?}", err),
    /// }
    /// ```
    ///
    /// # Notes
    /// - Users must have the **Manage Roles** permission in the guild to use this endpoint.
    /// - Role positions should be updated carefully to avoid unwanted hierarchy changes.
    async fn edit_guild_role_positions(
        &self,
        guild_id: GuildId,
        value: &Value,
    ) -> serenity::Result<Vec<Role>> {
        let body = to_vec(value)?;

        let result = self.request(Request::new(
            Route::GuildRoles { guild_id },
            LightMethod::Patch
        )
            .body(Some(body))
        ).await;

        match result{
            Ok(res) => {Ok(serde_json::from_str::<Vec<Role>>(res.text().await.unwrap().as_str()).unwrap())}
            Err(err) => {Err(err)}
        }
    }
}