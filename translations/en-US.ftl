ping = ping
    .description = allows to ping the bot and get the sending delay

create_universe = create_universe
    .description = Create a new Universe. A guild can be attached to only one universe at a time.
    .universe_name = universe_name
    .universe_name-description = Name of the new universe

start = start
    .description = Show startup instructions on the channel.

create_universe__already_exist_for_this_server = This server is already include in a universe.
universes_unavailable = There are no existing universe, or there are not available.
not-in-guild-error = Command must be invoked in a guild.
already_bind = This guild is already bind to a universe.
guild_linked = This guild is now linked to the universe ***{$universe_name}***.
choose_universe = Chose the universe in the list below.
exceed_limit_number_of_universes = You have reached the maximum number of universes you can create.
universe_created = L'univers {$universe_name} à bien été créé.
    Pour continuer, nous te recommandons désormais de setup les rôles avec la commande /roles.
start_message = Thank you for choosing RpBot to manage your RP server.
    The first step is to create a universe: `/{$create_universe}`
    Next, you’ll need to set up the roles required for administration: `/`
    Once that’s done, you can create your first locations: `/`
    Then your first routes: `/`