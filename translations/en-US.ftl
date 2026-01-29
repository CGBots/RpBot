support = contact.cgbots@gmail.com

ping = ping
    .description = allows to ping the bot and get the sending delay

create_universe = create_universe
    .description = Create a new Universe. A guild can be attached to only one universe at a time.
    .universe_name = universe_name
    .universe_name-description = Name of the new universe

start = start
    .description = Show startup instructions on the channel.
admin_role_name = Admin
moderator_role_name = Moderator
spectator_role_name = Spectator
player_role_name = Player

road_channel_name = Roads





setup = setup
    .description = Used to create roles and channnels.
    .setup_type = setup_type
    .setup_type-description = Full -> admin, character, creation, and wiki channels. Minimal -> necessary only.

FullSetup = Full
PartialSetup = Partial

cancel_setup = Cancel
continue_setup = Continue

continue_setup_message = The server appears to be already set up. Items that do not exist or no longer exist will be created.
setup__server_not_found = The server does not appear to be registered with the bot. Please contact support at {$support}
setup__server_already_setup_timeout = The selection is timeout.
setup__canceled = The setup has been cancelled.
setup__admin_role_not_created = The {$admin_role_name} role could not be created.
setup__moderator_role_not_created = The {$moderator_role_name} role could not be created.
setup__spectator_role_not_created = The {$spectator_role_name} role could not be created.
setup__player_role_not_created = The {$player_role_name} role could not be created.
setup__reorder_went_wrong = Role reordering failed. Please make sure the roles are ordered as follows: RpBot > {$admin_role_name} > {$moderator_role_name} > {$spectator_role_name} > {$player_role_name}
setup__road_category_not_created = The road category could not be created.
setup__server_update_failed = Saving the server configuration failed.
setup__universe_not_found = This server is not associated with any universe.
setup__setup_success_message = The setup completed successfully.
setup__setup_success_title = Setup complete.
setup__setup_error_message = The following errors have been detected:
    {$errors}

    Please verify the bot's rights. It should be Administrator.
setup__error_title = Server setup error.





already_exist_for_this_server = This server is already include in a universe.

exceed_limit_number_of_servers_per_universe = You have reached the maximum number of servers allowed for this universe.
    To increase the limit, please upgrade to a higher subscription plan.

universes_unavailable = There are no existing universe, or there are not available.

not-in-guild-error = Command must be invoked in a guild.

already_bind = This guild is already bind to a universe.

guild_linked = This guild is now linked to the universe ***{$universe_name}***.

choose_universe = Chose the universe in the list below.

exceed_limit_number_of_universes = You have reached the maximum number of universes you can create.

universe_created = L'univers {$universe_name} à bien été créé.
    Pour continuer, nous te recommandons désormais de setup les rôles avec la commande `/{$setup}`.

start_message = Thank you for choosing RpBot to manage your RP server.
    The first step is to create a universe: `/{$create_universe}`
    Next, you’ll need to set up the roles required for administration: `/{$setup}`
    Once that’s done, you can create your first locations: `/`
    Then your first routes: `/`