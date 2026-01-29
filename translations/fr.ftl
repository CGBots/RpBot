support = contact.cgbots@gmail.com

ping = ping
    .description = permet de ping le bot et d'avoir le délai d'envoi

create_universe = nouvel_univers
    .description = Permet de créer un nouvel univers. Un serveur ne peut être rattaché qu'à un univers à la fois.
    .universe_name = nom
    .universe_name-description = Nom du nouvel Univers
    
start = start
    .description = Affiche les instruction de démarrage dans le salon actuel.




admin_role_name = Admin
moderator_role_name = Moderateur
spectator_role_name = Spectateur
player_role_name = Joueur

road_channel_name = Routes





setup = setup
    .description = Permet de créer les salons necessaire au fonctionnement du bot.
    .setup_type = type
    .setup_type-description = Setup complet -> admin, création des personnages, wiki. Minimal -> nécessaire au bon fonctionnement.

FullSetup = Complet
PartialSetup = Partiel

cancel_setup = Annuler
continue_setup = Continuer

continue_setup_message = Le serveur semble déjà setup. Les éléments qui n'existe pas ou plus seront créés.

setup__server_not_found = Le serveur semble ne pas être enregistré auprès du bot. Contacte le support à {$support}
setup__server_already_setup_timeout = Le délai de séléction est écoulé.
setup__canceled = Le setup à été annulé.
setup__admin_role_not_created = Le rôle {$admin_role_name} n'as pas pu être créé.
setup__moderator_role_not_created = Le rôle {$moderator_role_name} n'as pas pu être créé.
setup__spectator_role_not_created = Le rôle {$spectator_role_name} n'as pas pu être créé.
setup__player_role_not_created = Le rôle {$player_role_name} n'as pas pu être créé.
setup__reorder_went_wrong = La réorganisation des rôles à échoué. Vérifiez que les rôles sont organisés comme suit: RpBot > {$admin_role_name} > {$moderator_role_name} > {$spectator_role_name} > {$player_role_name}
setup__road_category_not_created = La catégoriies des routes n'as pas pu être créé.
setup__server_update_failed = L'enregistrement des informations liés au serveur à échoué.
setup__universe_not_found = Le serveur n'est associé à aucun univers.
setup__setup_success_message = Le setup s'est terminé avec succès.
setup__setup_success_title = Setup terminé.
setup__setup_error_message = Les erreurs suivantes ont été détectés :
    {$errors}

    Merci de vérifier les droits du bot. Il doit être administrateur.
setup__error_title = Erreur a l'initialisation du serveur.





already_exist_for_this_server = Ce serveur fais déjà partie d'un univers.

exceed_limit_number_of_servers_per_universe = Vous avez atteind la limite maximale de server pour cet univers.
    Pour augmenter la limite merci de passer à un abonnement supérieur.

universes_unavailable = Tu n'as pas encore créé de serveur ou ils sont indisponibles.

not-in-guild-error = Cette commande ne peut être effectuée que dans un serveur.

already_bind = Ce serveur est déjà lié à un univers.

guild_linked = Le serveur fais maintenant partie de l'univers ***{$universe_name}***.

choose_universe = Choisissez l'univers dans la liste ci-dessous.

exceed_limit_number_of_universes = Tu as atteind la limite maximale d'univers que tu peut créer.

universe_created = L'univers {$universe_name} à bien été créé.
    Pour continuer, nous te recommandons désormais de setup les rôles avec la commande `/{$setup}`.

start_message = Merci d'avoir choisis RpBot pour administrer votre serveur RP.
    La première étape consiste à créer un univers : `/{$create_universe}`
    Ensuite vous devrez faire le setup des roles necessaires à l'administration : `/{$setup}`
    Une fois ceci fait, vous pourrez créer vos premier lieux : `/`
    Puis vos premières routes : `/`
