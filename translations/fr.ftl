placeholder = Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla eget neque arcu. Integer sed turpis.

support = contact.cgbots@gmail.com

ping = ping
    .description = permet de ping le bot et d'avoir le délai d'envoi

create_universe = nouvel_univers
    .description = Permet de créer un nouvel univers. Un serveur ne peut être rattaché qu'à un univers à la fois.
    .universe_name = nom
    .universe_name-description = Nom du nouvel Univers
    .setup_type = {setup.setup_type}
    .setup_type-description = {setup.setup_type-description}
    
start = start
    .description = Affiche les instruction de démarrage dans le salon actuel.

universe = univers
    .description = univers

admin_role_name = Admin
moderator_role_name = Moderateur
spectator_role_name = Spectateur
player_role_name = Joueur

road_channel_name = Routes


admin_category_name = Administration
setup__admin_category_not_created = La catégorie {$admin_category_name} n'a pas pu être créée.
nrp_category_name = Hors RP
setup__nrp_category_not_created = La catégorie {$nrp_category_name} n'a pas pu être créée.
rp_category_name = RolePlay
setup__rp_category_not_created = La catégorie {$rp_category_name} n'a pas pu être créée.
log_channel_name = Logs
setup__log_channel_not_created = Le salon {$log_channel_name} n'a pas pu être créé.
commands_channel_name = Commandes
setup__commands_channel_not_created = Le salon {$commands_channel_name} n'a pas pu être créé.
moderation_channel_name = Modération
setup__moderation_channel_not_created = Le salon {$moderation_channel_name} n'a pas pu être créé.
nrp_general_channel_name = Général
setup__nrp_general_channel_not_created = Le salon {$nrp_general_channel_name} n'a pas pu être créé.
rp_character_channel_name = Fiches personnage
setup__rp_character_channel_not_created = Le salon {$rp_character_channel_name} n'a pas pu être créé.
rp_wiki_channel_name = Wiki
setup__wiki_channel_not_created = Le salon {$rp_wiki_channel_name} n'a pas pu être créé.


setup = setup
    .description = Permet de créer les salons nécessaires au fonctionnement du bot.
    .setup_type = type
    .setup_type-description = Setup complet -> admin, création des personnages, wiki. Minimal -> nécessaire au bon fonctionnement.

FullSetup = Complet
PartialSetup = Partiel

cancel_setup = Annuler
continue_setup = Continuer

continue_setup_message = Le serveur semble déjà configuré. Les éléments manquants seront créés.

setup__server_not_found = Le serveur ne semble pas être enregistré auprès du bot. Contactez le support à {$support}
setup__server_already_setup_timeout = Le délai de sélection est écoulé.
setup__canceled = La configuration a été annulée.
setup__admin_role_not_created = Le rôle {$admin_role_name} n'a pas pu être créé.
setup__moderator_role_not_created = Le rôle {$moderator_role_name} n'a pas pu être créé.
setup__spectator_role_not_created = Le rôle {$spectator_role_name} n'a pas pu être créé.
setup__player_role_not_created = Le rôle {$player_role_name} n'a pas pu être créé.
setup__reorder_went_wrong = La réorganisation des rôles a échoué. Vérifiez que les rôles sont organisés comme suit : RpBot > {$admin_role_name} > {$moderator_role_name} > {$spectator_role_name} > {$player_role_name}
setup__road_category_not_created = La catégorie des routes n'a pas pu être créée.
setup__server_update_failed = L'enregistrement des informations liées au serveur a échoué.
setup__universe_not_found = Le serveur n'est associé à aucun univers.
setup__setup_success_message = La configuration s'est terminée avec succès.
setup__setup_success_title = Configuration terminée.
setup__setup_error_message = Les erreurs suivantes ont été détectées :
    {$errors}

    Merci de vérifier les droits du bot. Il doit être administrateur.
setup__setup_error_title = Erreur lors de l'initialisation du serveur.





already_exist_for_this_server = Ce serveur fait déjà partie d'un univers.

exceed_limit_number_of_servers_per_universe = Vous avez atteint la limite maximale de serveurs pour cet univers.
    Pour augmenter la limite, merci de passer à un abonnement supérieur.

universes_unavailable = Vous n'avez pas encore créé d'univers ou ils sont indisponibles.

not-in-guild-error = Cette commande ne peut être effectuée que dans un serveur.

already_bind = Ce serveur est déjà lié à un univers.

guild_linked = Le serveur fait maintenant partie de l'univers ***{$universe_name}***.

choose_universe = Choisissez l'univers dans la liste ci-dessous.

exceed_limit_number_of_universes = Vous avez atteint la limite maximale d'univers que vous pouvez créer.

universe_created = L'univers {$universe_name} a bien été créé. Les rôles et les catégories sont en cour de création.

start_message = Merci d'avoir choisi RpBot pour administrer votre serveur RP.
    La première étape consiste à créer un univers : `/{$create_universe}`
    Ensuite, vous devrez configurer les rôles et salons nécessaires : `/{$setup}`
    Une fois ceci fait, vous pourrez créer vos premiers lieux : `/`
    Puis vos premières routes : `/`
