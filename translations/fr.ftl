placeholder = Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla eget neque arcu. Integer sed turpis.
    .title = Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla eget neque arcu. Integer sed turpis.
    .message = Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla eget neque arcu. Integer sed turpis.

support = contact.cgbots@gmail.com ou @cgbots sur discord

ping = ping
    .description = permet de ping le bot et d'avoir le délai d'envoi
    
start = start
    .description = Affiche les instruction de démarrage dans le salon actuel.
start_message = Start Message
    .title = Merci d'utiliser
    .description = Pour commencer à utiliser le bot, commencez par créer un nouvel univers.
    Utilisez la commande `/{universe} {create_universe} [nom de votre univers] [type de setup]`
    Le type de setup détermine quels salons seront créés.
    Dans un setup partiel, seule la catégorie route et les rôles seront créés.
    Dans un setup complet, les catégories Admin, hors rp, rp et leurs selons sont également créés en plus.

#Stats
stat_insert__failed = Échec de l'insertion des statistiques
    .title = Ajout de la stat échouée
    .description = La stat n'as pas pu être ajouté.
#Reply
reply__reply_success = Succès
    .title = Succès
    .message = L'opération a été effectuée avec succès.
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
partial_setup__get_guild_roles_error = Échec de la récupération des rôles du serveur
    .title = Erreur de configuration
    .message = Impossible de récupérer les rôles du serveur.
            Veuillez réessayer ou contacter le support si le problème persiste : {$support}
setup__server_not_found = Serveur introuvable
    .title = Serveur introuvable
    .message = Ce serveur n'est pas enregistré dans notre base de données.
            Veuillez réessayer ou contacter le support si le problème persiste : {$support}
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

#Create character
create_character = nouveau_personnage
    .description = Commande pour créer un nouveau personnage. Limité à 1 par joueur.

character_modal_title = Créer un nouveau personnage
create_character__delete_character = Annuler
create_character__submit_character = Envoyer
create_character__modify_character = Modifier
create_character__refuse_character = Refuser
create_character__accept_character = Accepter
character_special_request = Requêtes spéciales
character_story = Histoire du personnage
character_description = Description physique
character_name = Nom du personnage
create_character__start_place = Lieu de départ
create_character__submit_notification = @here Une fiche de personnage est en attente de vérification :

character_reject_reason = Raison du refus

create_character__no_universe_found = Univers introuvable
    .title = Univers introuvable
    .message = Il n'y a pas d'univers existant pour ce serveur.
create_character__database_error = Erreur de base de données
    .title = Erreur de base de données
    .message = Impossible d'accéder à la base de données.
            Veuillez réessayer ou contacter le support si le problème persiste : {$support}
create_character__wrong_channel = Mauvais salon
    .title = Mauvais salon
    .message = Cette commande doit être utilisée dans le salon des fiches de personnage.
create_character__character_already_existing = Le personnage existe déjà
    .title = Le personnage existe déjà
    .message = Vous avez déjà un personnage. Vous ne pouvez pas en créer un autre.
CharacterModal = character_modal
    .character_name = Nom
    .character_description = Description du personnage
    .placeholder = Décrivez votre personnage ici...
    .character_story = Histoire du personnage
    .value = Il était une fois...
    .character_special_request = Requêtes spéciales
create_character__submitted = Personnage envoyé
    .title = Personnage envoyé
    .message = Votre fiche de personnage a été envoyée pour vérification. Veuillez attendre la décision d'un modérateur.
create_place__character_too_long = Fiche de personnage trop longue
    .title = Fiche de personnage trop longue
    .message = La fiche de personnage est trop longue pour être affichée. Veuillez réessayer.
character_instruction = Remplissez les champs suivants pour décrire votre personnage.
    ► Tous les champs de paragraphe sont limités à 1024 caractères.
    ► Un délai de 30 minutes est configuré par sécurité.
    Vous pouvez cliquer sur le bouton modifier pour changer votre brouillon avant de l'envoyer aux modérateurs.
create_character__timed_out = Délai dépassé
    .title = Délai dépassé
    .message = Le processus de création de personnage a expiré.
create_character__guild_only = Serveur uniquement
    .title = Serveur uniquement
    .message = Cette commande ne peut être utilisée qu'au sein d'un serveur.
create_character__delete_successfull = Annulé
    .title = Création de personnage annulée
    .message = Votre processus de création de personnage a été annulé avec succès.
delete_character = Personnage supprimé
    .title = Personnage supprimé
    .message = La fiche de personnage a été supprimée avec succès.
create_character__not_owner = Pas le propriétaire
    .title = Pas le propriétaire
    .message = Vous n'êtes pas le propriétaire de ce personnage. Vous ne pouvez pas effectuer cette action.
create_character__no_member = Membre introuvable
    .title = Erreur
    .message = Impossible de trouver les informations du membre.
create_character__no_permission = Permission refusée
    .title = Permission refusée
    .message = Vous n'avez pas les permissions requises (Modérateur ou Administrateur) pour effectuer cette action.
create_character__invalid_footer = Interaction invalide
    .title = Erreur
    .message = Les métadonnées de l'interaction sont invalides.
create_character__refused = Personnage refusé
    .title = Personnage refusé
    .message = Le personnage a été refusé par un modérateur.
accept_character = Personnage accepté
    .title = Personnage accepté
    .message = Le personnage a été accepté avec succès et ajouté à l'univers.
create_character__type_mismatch = Incompatibilité de type
    .title = Erreur de validation
    .message = L'une des valeurs de statistiques fournies ne correspond pas au type attendu.
create_character__invalid_place_selected = Lieu invalide sélectionné. Veuillez choisir une catégorie valide pour le personnage.
create_character__invalid_interaction = Données d'interaction invalides.
create_character__choose_place = Choisir un lieu
    .title = Choisir un lieu
    .message = Veuillez sélectionner la catégorie où le personnage sera situé.
character_stat_input = Statistiques du personnage
accept_character__no_player_role_id = Serveur non setup
    .title = Serveur non setup
    .message = Le role {player_role_name} n'as pas été trouvé.
