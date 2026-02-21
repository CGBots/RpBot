placeholder = Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla eget neque arcu. Integer sed turpis.

support = contact.cgbots@gmail.com ou @cgbots sur discord

ping = ping
    .description = permet de ping le bot et d'avoir le délai d'envoi
    
start = start
    .description = Affiche les instruction de démarrage dans le salon actuel.
start_message = Start Message
    .title = Merci d'utiliser
    .message = Pour commencer à utiliser le bot, commencez par créer un nouvel univers.
    Utilisez la commande `/{$universe} {$create_universe} [nom de votre univers] [type de setup]`
    Le type de setup détermine quels salons seront créés.
    Dans un setup partiel, seule la catégorie route et les rôles seront créés.
    Dans un setup complet, les catégories Admin, hors rp, rp et leurs selons sont également créés en plus.

#Stats
stat_insert__failed = Échec de l'insertion des statistiques
    .title = Ajout de la stat échouée
    .description = La stat n'as pas pu être ajouté.
#Reply
reply__reply_failed = Échec de l'envoi de la réponse
    .title = Réponse échouée
    .description = La réponse à échouée
#Universe
universe = univers
    .description = univers
check_universe_ownership__universe_not_found = Univers introuvable
    .title = Univers introuvable
    .mesage = L'univers spécifié n'a pas été trouvé
universe_delete__failed = Échec de la suppression de l'univers
    .title = Echec de suppression
    .description = L'univers n'as pas pû être supprimé.
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
universe_delete__passed = Univers supprimé avec succès
    .title = Univers supprimé
    .description = L'univers à bien été supprimé.
universe__check_server_limit_failed = Échec de la vérification de la limite de serveurs
    .title = Erreur de vérification
    .message = Impossible de vérifier la limite de serveurs pour cet univers
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}

#Create universe
create_universe = nouvel_univers
    .description = Permet de créer un nouvel univers. Un serveur ne peut être rattaché qu'à un univers à la fois.
    .universe_name = nom
    .universe_name-description = Nom du nouvel Univers
    .setup_type = type_de_setup
    .setup_type-description = Type de configuration pour ce serveur
create_universe__check_universe_limit_failed = Échec de la vérification de la limite d'univers
    .title = Erreur de vérification
    .message = Impossible de vérifier la limite d'univers
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
create_universe__universe_limit_reached = Limite d'univers atteinte
    .title = Limite atteinte
    .message = Vous avez atteint le nombre maximum d'univers autorisés
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
create_universe__get_server_failed = Échec de la récupération du serveur
    .title = Erreur serveur
    .message = Impossible de récupérer les informations du serveur
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
create_universe__already_exist_for_this_server = Un univers existe déjà pour ce serveur
    .title = Univers existant
    .message = Ce serveur est déjà lié à un univers
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
create_universe__setup_constraints_failed = Échec de la vérification des contraintes de configuration
    .title = Erreur de contraintes
    .message = Les contraintes de configuration n'ont pas pu être vérifiées
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
create_universe__server_insert_failed = Échec de l'insertion du serveur
    .title = Erreur d'insertion
    .message = Impossible d'insérer le serveur dans la base de données
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
create_universe__universe_insert_failed = Échec de l'insertion de l'univers
    .title = Erreur de création
    .message = Impossible de créer l'univers dans la base de données
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
create_universe__speed_stat_insert_failed = Échec de l'insertion des statistiques de vitesse
    .title = Erreur de statistiques
    .message = Impossible d'insérer les statistiques de vitesse
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
create_universe__universe_successfully_created = Univers créé avec succès
    .title = Succès
    .message = Votre nouvel univers a été créé avec succès

#Add server to universe
add_server = ajouter
    .description = ajoute un serveur à l'univers
    .setup_type = setup_type
    .setup_type-description = Type de configuration pour ce serveur
add_server_to_universe__already_bind = Serveur déjà lié à un univers
    .title = Serveur déjà lié
    .message = Ce serveur est déjà rattaché à un univers
add_server_to_universe__universes_unavailable = Aucun univers disponible
    .title = Univers indisponibles
    .message = Aucun univers n'est disponible pour ce serveur
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
choose_universe =
    exceed_limit_number_of_servers_per_universe = Limite de serveurs par univers dépassée
    .title = Limite dépassée
    .message = Le nombre maximum de serveurs pour cet univers a été atteint.
            Si vous avez besoin d'aller au delà de cette limite, merci d'en faire la demande au support: {$support}
add_server_to_universe__guild_linked = Serveur lié à l'univers
    .title = Serveur lié
    .message = Le serveur a été lié à l'univers avec succès

#Server
id__nothing_to_delete = Rien à supprimer
    .title = Rien à supprimer
    .message = Aucun élément à supprimer n'a été trouvé
id__role_delete_success = Rôle supprimé avec succès
    .title = Suppression réussie
    .message = Le rôle a été supprimé avec succès
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
id__role_delete_failed = Échec de la suppression du rôle
    .title = Erreur de suppression
    .message = Impossible de supprimer le rôle
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
id__channel_delete_sucess = Salon supprimé avec succès
    .title = Suppression réussie
    .message = Le salon a été supprimé avec succès
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
id__channel_delete_failed = Échec de la suppression du salon
    .title = Erreur de suppression
    .message = Impossible de supprimer le salon
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}

#Setup
SetupType = SetupType
    .FullSetup = Complet
    .PartialSetup = Partiel
cancel_setup = Annuler
continue_setup = Continuer 
setup__continue_setup_message = Continuer la configuration ?
    .title = Continuer la configuration
    .message = Voulez-vous continuer la configuration malgré un précédent setup ?  Les salon et rôles inexistants seront créés.
setup__server_already_setup_timeout = Délai de configuration dépassé
    .title = Délai dépassé
    .message = Le délai pour continuer la configuration a expiré
setup_server__cancelled = Configuration annulée
    .title = Configuration annulée
    .message = La configuration du serveur a été annulée
setup_server__success = Configuration réussie
    .title = Succès
    .message = Le serveur a été configuré avec succès
setup_server__failed = Échec de la configuration
    .title = Erreur
    .message = La configuration du serveur a échoué
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
setup__full_setup_success = Configuration complète réussie
    .title = Configuration terminée
    .message = La configuration complète du serveur a été effectuée avec succès
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
admin_category_name = Administration
    .title = Administration
    .message = Catégorie d'administration
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
setup__admin_category_not_created = Catégorie d'administration non créée
    .title = Erreur de création
    .message = Impossible de créer la catégorie d'administration
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
nrp_category_name = Hors RP
setup__nrp_category_not_created = Catégorie Hors RP non créée
    .title = Erreur de création
    .message = Impossible de créer la catégorie Hors RP
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
rp_category_name = RP
setup__rp_category_not_created = Catégorie RP non créée
    .title = Erreur de création
    .message = Impossible de créer la catégorie RP
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
setup__roles_setup_failed = Échec de la configuration des rôles
    .title = Erreur de configuration
    .message = La configuration des rôles a échoué
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
log_channel_name = Logs
setup__log_channel_not_created = Salon de logs non créé
    .title = Erreur de création
    .message = Impossible de créer le salon de log
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
commands_channel_name = Commandes
setup__commands_channel_not_created = Salon de commandes non créé
    .title = Erreur de création
    .message = Impossible de créer le salon de commandes
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
moderation_channel_name = Moderation
setup__moderation_channel_not_created = Salon de modération non créé
    .title = Erreur de création
    .message = Impossible de créer le salon de modération
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
nrp_general_channel_name = General
setup__nrp_general_channel_not_created = Salon général Hors RP non créé
    .title = Erreur de création
    .message = Impossible de créer le salon général Hors RP
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
rp_character_channel_name = Fiches personnages
setup__rp_character_channel_not_created = Salon de fiches personnages non créé
    .title = Erreur de création
    .message = Impossible de créer le salon de fiches personnages
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
rp_wiki_channel_name = Wiki
setup__wiki_channel_not_created = Salon wiki non créé
    .title = Erreur de création
    .message = Impossible de créer le salon wiki
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
setup__rollback_failed = Échec de l'annulation des modifications
    .title = Erreur d'annulation
    .message = Impossible d'annuler les modifications effectuées
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
setup__channel_setup_failed = Échec de la configuration des salons
    .title = Erreur de configuration
    .message = La configuration des salons a échoué
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
guild_only = Commande réservée aux serveurs.
admin_role_name = Administrateur
setup__admin_role_not_created = Rôle Administrateur non créé
    .title = Erreur de création
    .message = Impossible de créer le rôle Administrateur
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
moderator_role_name = Modérateur
setup__moderator_role_not_created = Rôle Modérateur non créé
    .title = Erreur de création
    .message = Impossible de créer le rôle Modérateur
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
spectator_role_name = Spectateur
setup__spectator_role_not_created = Rôle Spectateur non créé
    .title = Erreur de création
    .message = Impossible de créer le rôle Spectateur
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
player_role_name = Joueur
setup__player_role_not_created = Rôle Joueur non créé
    .title = Erreur de création
    .message = Impossible de créer le rôle Joueur
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
setup__error_during_role_creation = Erreur lors de la création des rôles
    .title = Erreur de création
    .message = Une erreur s'est produite lors de la création des rôles
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
setup__reorder_went_wrong = Erreur lors du réordonnancement
    .title = Erreur de réordonnancement
    .message = Une erreur s'est produite lors du réordonnancement des rôles
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
road_channel_name = Routes
setup__road_category_not_created = Catégorie Routes non créée
    .title = Erreur de création
    .message = Impossible de créer la catégorie Routes
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
setup__server_update_failed = Échec de la mise à jour du serveur
    .title = Erreur de mise à jour
    .message = Impossible de mettre à jour les informations du serveur
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
setup__setup_success_message = Configuration terminée avec succès
    .title = Succès
    .message = La configuration a été effectuée avec succès

#create place
create_placce = nouvel_endroit
    .description = Créé une catégorie correpondant à une ville, regrouppant plusieurs lieux d'interaction
    .name = nom
    .name-description = nom du lieu
create_place__server_not_found = Serveur inconnu
    .title = Server inconnu
    .message = Le serveur semble ne pas être enregistré. Faites /{$universe} {$add_server} [type de setup]
create_place__database_not_found = Base de données introuvable
    .title = Connexion échouée
    .message = La connexion à la base de donénes à échouée.
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
create_place__role_not_created = Création de rôle échouée
    .title = Création de rôle échouée
    .message = Le rôle du lieu n'as pas pu être créé correctement.
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
create_place__rollback_complete = Rollback terminé
    .title = Rollback effectué
    .message = Quelque chose s'est mal passé durant la création du lieu. Un rollback à été effectué.
create_role__rollback_failed = Rollback échoué
    .title = Rollback échoué
    .message = Quelque chose s'est mal passé durant la création du lieu et le rollback à échoué.
            Veuillez contacter le support: {$support}
create_place__success = Place créée
    .title = Place créée
    .message = La place à été créée avec succès.

#Create road
create_road = nouvelle_route
    .description = Commande pour créer une nouvelle route entre 2 lieux
    .place_one = lieu_un
    .place_one-description = Première extrémité de la route
    .place_two = lieu_deux
    .place_two-description = Seconde extrémité de la route
    .distance = distance
    .distance-description = Distance séparant les deux lieux en Km.
create_road__server_not_found = Serveur introuvable
    .title = Serveur introuvable
    .message = Le serveur ne semble pas être enregistré. Faites /{$universe} {$add_server} [type de setup]
create_road__database_error = Erreur de base de données
    .title = Erreur de base de données
    .message = Une erreur s'est produite lors de l'accès à la base de données.
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
create_place__place_one_not_found = Premier lieu introuvable
    .title = Premier lieu introuvable
    .message = Le premier lieu spécifié n'a pas été trouvé dans l'univers.
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
create_place__place_two_not_found = Second lieu introuvable
    .title = Second lieu introuvable
    .message = Le second lieu spécifié n'a pas été trouvé dans l'univers.
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
create_road__role_creation_failed = Erreur de création de rôle
    .title = Erreur de création de rôle
    .message = Le rôle de la route n'a pas pu être créé correctement.
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
create_road__create_channel_failed_rollback_success = Erreur de création de salon
    .title = Erreur de création de salon
    .message = Le salon n'a pas pu être créé, mais les modifications ont été annulées.
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
create_road__create_channel_failed_rollback_failed = Erreur critique
    .title = Erreur critique
    .message = La création du salon a échoué et le rollback n'a pas pu être effectué.
            Veuillez contacter le support: {$support}
create_road__insert_road_failed_rollback_success = Erreur d'insertion
    .title = Erreur d'insertion
    .message = La route n'a pas pu être sauvegardée, mais les modifications ont été annulées.
            Veuillez ressayer ou contacter le support si le problème persiste: {$support}
create_road__insert_road_failed_rollback_channel_failed = Erreur critique
    .title = Erreur critique
    .message = L'enregistrement de la route a échoué et l'annulation du salon a échoué.
            Veuillez contacter le support: {$support}
create_road__insert_road_failed_rollback_role_failed = Erreur critique
    .title = Erreur critique
    .message = L'enregistrement de la route a échoué et l'annulation du rôle a échoué.
            Veuillez contacter le support: {$support}
create_road__success = Route créée
    .title = Route créée
    .message = La route a été créée avec succès