placeholder = Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla eget neque arcu. Integer sed turpis.

support = contact.cgbots@gmail.com or @cgbots on discord

ping = ping
    .description = allows you to ping the bot and get the send delay

start = start
    .description = Displays startup instructions in the current channel.
start_message = Start Message
    .title = Thank you for using VerseEngine!
    .message = To start using the bot, begin by creating a new universe.
            Use the command `/{$universe} {$create_universe} [your universe name] [setup type]`
            The setup type determines which channels will be created.
            In a partial setup, only the road category and roles will be created.
            In a full setup, the Admin, out of rp, rp categories and their channels are also created.

#Stats
stat_insert__failed = Failed to insert statistics
    .title = Failed to add stat
    .description = The stat could not be added.
#Reply
reply__reply_failed = Failed to send reply
    .title = Reply failed
    .description = The reply failed
#Universe
universe = universe
    .description = universe
check_universe_ownership__universe_not_found = Universe not found
    .title = Universe not found
    .mesage = The specified universe was not found
universe_delete__failed = Failed to delete universe
    .title = Deletion failed
    .description = The universe could not be deleted.
            Please try again or contact support if the problem persists: {$support}
universe_delete__passed = Universe successfully deleted
    .title = Universe deleted
    .description = The universe has been successfully deleted.
universe__check_server_limit_failed = Failed to verify server limit
    .title = Verification error
    .message = Unable to verify the server limit for this universe
            Please try again or contact support if the problem persists: {$support}

#Create universe
create_universe = new_universe
    .description = Allows you to create a new universe. A server can only be attached to one universe at a time.
    .universe_name = name
    .universe_name-description = Name of the new Universe
    .setup_type = setup_type
    .setup_type-description = Configuration type for this server
create_universe__check_universe_limit_failed = Failed to verify universe limit
    .title = Verification error
    .message = Unable to verify the universe limit
            Please try again or contact support if the problem persists: {$support}
create_universe__universe_limit_reached = Universe limit reached
    .title = Limit reached
    .message = You have reached the maximum number of allowed universes
            Please try again or contact support if the problem persists: {$support}
create_universe__get_server_failed = Failed to retrieve server
    .title = Server error
    .message = Unable to retrieve server information
            Please try again or contact support if the problem persists: {$support}
create_universe__already_exist_for_this_server = A universe already exists for this server
    .title = Existing universe
    .message = This server is already linked to a universe
            Please try again or contact support if the problem persists: {$support}
create_universe__setup_constraints_failed = Failed to verify configuration constraints
    .title = Constraints error
    .message = Configuration constraints could not be verified
            Please try again or contact support if the problem persists: {$support}
create_universe__server_insert_failed = Failed to insert server
    .title = Insertion error
    .message = Unable to insert the server into the database
            Please try again or contact support if the problem persists: {$support}
create_universe__universe_insert_failed = Failed to insert universe
    .title = Creation error
    .message = Unable to create the universe in the database
            Please try again or contact support if the problem persists: {$support}
create_universe__speed_stat_insert_failed = Failed to insert speed statistics
    .title = Statistics error
    .message = Unable to insert speed statistics
            Please try again or contact support if the problem persists: {$support}
create_universe__universe_successfully_created = Universe successfully created
    .title = Success
    .message = Your new universe has been successfully created

#Add server to universe
add_server = add
    .description = adds a server to the universe
    .setup_type = setup_type
    .setup_type-description = Configuration type for this server
add_server_to_universe__already_bind = Server already linked to a universe
    .title = Server already linked
    .message = This server is already attached to a universe
add_server_to_universe__universes_unavailable = No universe available
    .title = Universes unavailable
    .message = No universe is available for this server
            Please try again or contact support if the problem persists: {$support}
choose_universe =
    exceed_limit_number_of_servers_per_universe = Server limit per universe exceeded
    .title = Limit exceeded
    .message = The maximum number of servers for this universe has been reached.
            If you need to go beyond this limit, please request it from support: {$support}
add_server_to_universe__guild_linked = Server linked to universe
    .title = Server linked
    .message = The server has been successfully linked to the universe

#Server
id__nothing_to_delete = Nothing to delete
    .title = Nothing to delete
    .message = No item to delete was found
id__role_delete_success = Role successfully deleted
    .title = Deletion successful
    .message = The role has been successfully deleted
            Please try again or contact support if the problem persists: {$support}
id__role_delete_failed = Failed to delete role
    .title = Deletion error
    .message = Unable to delete the role
            Please try again or contact support if the problem persists: {$support}
id__channel_delete_sucess = Channel successfully deleted
    .title = Deletion successful
    .message = The channel has been successfully deleted
            Please try again or contact support if the problem persists: {$support}
id__channel_delete_failed = Failed to delete channel
    .title = Deletion error
    .message = Unable to delete the channel
            Please try again or contact support if the problem persists: {$support}

#Setup
SetupType = SetupType
    .FullSetup = Full
    .PartialSetup = Partial
cancel_setup = Cancel
continue_setup = Continue 
setup__continue_setup_message = Continue setup?
    .title = Continue setup
    .message = Do you want to continue the setup despite a previous setup? Missing channels and roles will be created.
setup__server_already_setup_timeout = Setup timeout exceeded
    .title = Timeout exceeded
    .message = The time to continue the setup has expired
setup_server__cancelled = Setup cancelled
    .title = Setup cancelled
    .message = Server setup has been cancelled
setup_server__success = Setup successful
    .title = Success
    .message = The server has been successfully configured
setup_server__failed = Setup failed
    .title = Error
    .message = Server setup failed
            Please try again or contact support if the problem persists: {$support}
setup__full_setup_success = Full setup successful
    .title = Setup completed
    .message = Full server setup has been successfully completed
            Please try again or contact support if the problem persists: {$support}
admin_category_name = Administration
    .title = Administration
    .message = Administration category
            Please try again or contact support if the problem persists: {$support}
setup__admin_category_not_created = Administration category not created
    .title = Creation error
    .message = Unable to create the administration category
            Please try again or contact support if the problem persists: {$support}
nrp_category_name = Out of RP
setup__nrp_category_not_created = Out of RP category not created
    .title = Creation error
    .message = Unable to create the Out of RP category
            Please try again or contact support if the problem persists: {$support}
rp_category_name = RP
setup__rp_category_not_created = RP category not created
    .title = Creation error
    .message = Unable to create the RP category
            Please try again or contact support if the problem persists: {$support}
setup__roles_setup_failed = Role setup failed
    .title = Setup error
    .message = Role setup failed
            Please try again or contact support if the problem persists: {$support}
log_channel_name = Logs
setup__log_channel_not_created = Logs channel not created
    .title = Creation error
    .message = Unable to create the log channel
            Please try again or contact support if the problem persists: {$support}
commands_channel_name = Commands
setup__commands_channel_not_created = Commands channel not created
    .title = Creation error
    .message = Unable to create the commands channel
            Please try again or contact support if the problem persists: {$support}
moderation_channel_name = Moderation
setup__moderation_channel_not_created = Moderation channel not created
    .title = Creation error
    .message = Unable to create the moderation channel
            Please try again or contact support if the problem persists: {$support}
nrp_general_channel_name = General
setup__nrp_general_channel_not_created = Out of RP general channel not created
    .title = Creation error
    .message = Unable to create the Out of RP general channel
            Please try again or contact support if the problem persists: {$support}
rp_character_channel_name = Character sheets
setup__rp_character_channel_not_created = Character sheets channel not created
    .title = Creation error
    .message = Unable to create the character sheets channel
            Please try again or contact support if the problem persists: {$support}
rp_wiki_channel_name = Wiki
setup__wiki_channel_not_created = Wiki channel not created
    .title = Creation error
    .message = Unable to create the wiki channel
            Please try again or contact support if the problem persists: {$support}
setup__rollback_failed = Failed to rollback changes
    .title = Rollback error
    .message = Unable to rollback the changes made
            Please try again or contact support if the problem persists: {$support}
setup__channel_setup_failed = Channel setup failed
    .title = Setup error
    .message = Channel setup failed
            Please try again or contact support if the problem persists: {$support}
guild_only = Command reserved for servers.
admin_role_name = Administrator
setup__admin_role_not_created = Administrator role not created
    .title = Creation error
    .message = Unable to create the Administrator role
            Please try again or contact support if the problem persists: {$support}
moderator_role_name = Moderator
setup__moderator_role_not_created = Moderator role not created
    .title = Creation error
    .message = Unable to create the Moderator role
            Please try again or contact support if the problem persists: {$support}
spectator_role_name = Spectator
setup__spectator_role_not_created = Spectator role not created
    .title = Creation error
    .message = Unable to create the Spectator role
            Please try again or contact support if the problem persists: {$support}
player_role_name = Player
setup__player_role_not_created = Player role not created
    .title = Creation error
    .message = Unable to create the Player role
            Please try again or contact support if the problem persists: {$support}
setup__error_during_role_creation = Error during role creation
    .title = Creation error
    .message = An error occurred during role creation
            Please try again or contact support if the problem persists: {$support}
setup__reorder_went_wrong = Error during reordering
    .title = Reordering error
    .message = An error occurred during role reordering
            Please try again or contact support if the problem persists: {$support}
road_channel_name = Roads
setup__road_category_not_created = Roads category not created
    .title = Creation error
    .message = Unable to create the Roads category
            Please try again or contact support if the problem persists: {$support}
setup__server_update_failed = Failed to update server
    .title = Update error
    .message = Unable to update server information
            Please try again or contact support if the problem persists: {$support}
setup__setup_success_message = Setup completed successfully
    .title = Success
    .message = The setup has been completed successfully

#create place
create_placce = new_place
    .description = Creates a category corresponding to a city, grouping multiple interaction locations
    .name = name
    .name-description = name of the place
create_place__server_not_found = Unknown server
    .title = Unknown server
    .message = The server does not appear to be registered. Run /{$universe} {$add_server} [setup type]
create_place__database_not_found = Database not found
    .title = Connection failed
    .message = The database connection failed.
            Please try again or contact support if the problem persists: {$support}
create_place__role_not_created = Role creation failed
    .title = Role creation failed
    .message = The place role could not be created correctly.
            Please try again or contact support if the problem persists: {$support}
create_place__rollback_complete = Rollback completed
    .title = Rollback performed
    .message = Something went wrong during place creation. A rollback has been performed.
create_role__rollback_failed = Rollback failed
    .title = Rollback failed
    .message = Something went wrong during place creation and the rollback failed.
            Please contact support: {$support}
create_place__success = Place created
    .title = Place created
    .message = The place has been successfully created.

#Create road
create_road = create_road
    .description = Command to create a new road between 2 places
    .place_one = place_one
    .place_one-description = First end of the road
    .place_two = place_two
    .place_two-description = Second end of the road
    .distance = distance
    .distance-description = Distance between the two places in Km.
create_road__server_not_found = Server not found
    .title = Server not found
    .message = The server does not appear to be registered. Run /{$universe} {$add_server} [setup type]
create_road__database_error = Database error
    .title = Database error
    .message = An error occurred while accessing the database.
                        Please try again or contact support if the problem persists: {$support}
create_place__place_one_not_found = First place not found
    .title = First place not found
    .message = The first specified place was not found in the universe.
                        Please try again or contact support if the problem persists: {$support}
create_place__place_two_not_found = Second place not found
    .title = Second place not found
    .message = The second specified place was not found in the universe.
                        Please try again or contact support if the problem persists: {$support}
create_road__role_creation_failed = Role creation error
    .title = Role creation error
    .message = The road role could not be created correctly.
                        Please try again or contact support if the problem persists: {$support}
create_road__create_channel_failed_rollback_success = Channel creation error
    .title = Channel creation error
    .message = The channel could not be created, but the changes have been rolled back.
                        Please try again or contact support if the problem persists: {$support}
create_road__create_channel_failed_rollback_failed = Critical error
    .title = Critical error
    .message = Channel creation failed and the rollback could not be performed.
                        Please contact support: {$support}
create_road__insert_road_failed_rollback_success = Insertion error
    .title = Insertion error
    .message = The road could not be saved, but the changes have been rolled back.
                        Please try again or contact support if the problem persists: {$support}
create_road__insert_road_failed_rollback_channel_failed = Critical error
    .title = Critical error
    .message = The road registration failed and the channel rollback failed.
                        Please contact support: {$support}
create_road__insert_road_failed_rollback_role_failed = Critical error
    .title = Critical error
    .message = The road registration failed and the role rollback failed.
                        Please contact support: {$support}
create_road__success = Road created
    .title = Road created
    .message = The road has been successfully created