placeholder = Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla eget neque arcu. Integer sed turpis.

support = contact.cgbots@gmail.com

ping = ping
    .description = allows to ping the bot and get the sending delay

create_universe = create_universe
    .description = Create a new Universe. A guild can be attached to only one universe at a time.
    .universe_name = universe_name
    .universe_name-description = Name of the new universe
    .setup_type = {setup.setup_type}
    .setup_type-description = {setup.setup_type-description}

start = start
    .description = Show startup instructions on the channel.

universe = universe
    .description = universe

admin_role_name = Admin
moderator_role_name = Moderator
spectator_role_name = Spectator
player_role_name = Player

road_channel_name = Roads


admin_category_name = Administration
setup__admin_category_not_created = The category {$admin_category_name} could not be created.
nrp_category_name =  Non-RP
setup__nrp_category_not_created = The category {$nrp_category_name} could not be created.
rp_category_name = RolePlay
setup__rp_category_not_created = The category {$rp_category_name} could not be created.
log_channel_name = Logs
setup__log_channel_not_created = The channel {$log_channel_name} could not be created.
commands_channel_name = Commands
setup__commands_channel_not_created = The channel {$commands_channel_name} could not be created.
moderation_channel_name = Moderation
setup__moderation_channel_not_created = The channel {$moderation_channel_name} could not be created.
nrp_general_channel_name = General
setup__nrp_general_channel_not_created = The channel {$nrp_general_channel_name} could not be created.
rp_character_channel_name = Character Sheets
setup__rp_character_channel_not_created = The channel {$rp_character_channel_name} could not be created.
rp_wiki_channel_name = Wiki
setup__wiki_channel_not_created = The channel {$rp_wiki_channel_name} could not be created.


setup = setup
    .description = Used to create roles and channnels.
    .setup_type = setup_type
    .setup_type-description = Full -> admin, character, creation, and wiki channels. Minimal -> necessary only.

FullSetup = Full
PartialSetup = Partial

cancel_setup = Cancel
continue_setup = Continue

continue_setup_message = The guild appears to be already set up. Items that do not exist or no longer exist will be created.

setup__server_not_found = The guild does not appear to be registered with the bot. Please contact support at {$support}
setup__server_already_setup_timeout = The selection is timeout.
setup__canceled = The setup has been cancelled.
setup__admin_role_not_created = The {$admin_role_name} role could not be created.
setup__moderator_role_not_created = The {$moderator_role_name} role could not be created.
setup__spectator_role_not_created = The {$spectator_role_name} role could not be created.
setup__player_role_not_created = The {$player_role_name} role could not be created.
setup__reorder_went_wrong = Role reordering failed. Please make sure the roles are ordered as follows: RpBot > {$admin_role_name} > {$moderator_role_name} > {$spectator_role_name} > {$player_role_name}
setup__road_category_not_created = The road category could not be created.
setup__server_update_failed = Saving the guild configuration failed.
setup__universe_not_found = This guild is not associated with any universe.
setup__setup_success_message = The setup completed successfully.
setup__setup_success_title = Setup complete.
setup__setup_error_message = The following errors have been detected:
    {$errors}

    Please verify the bot's rights. It should be Administrator.
setup__setup_error_title = guild setup error.





already_exist_for_this_server = This guild is already part of a universe.

exceed_limit_number_of_servers_per_universe = You have reached the maximum number of guilds allowed for this universe.
    To increase the limit, please upgrade to a higher subscription plan.

universes_unavailable = There are no existing universes, or they are not available.

not-in-guild-error = This command must be invoked in a guild.

already_bind = This guild is already bound to a universe.

guild_linked = This guild is now linked to the universe ***{$universe_name}***.

choose_universe = Choose the universe in the list below.

exceed_limit_number_of_universes = You have reached the maximum number of universes you can create.

universe_created = The universe {$universe_name} has been successfully created. The roles and categories is going to be created.

start_message = Thank you for choosing VerseEngine to manage your RP.
    The first step is to create a universe: `/{$universe} {$create_universe}`
    Next, you’ll need to set up the roles and channels required: `/{$universe} {$setup}`
    Once that’s done, you can create your first locations: `/`
    Then your first routes: `/`