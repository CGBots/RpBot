ping = ping
    .description = permet de ping le bot et d'avoir le délai d'envoi

create_universe = nouvel_univers
    .description = Permet de créer un nouvel univers. Un serveur ne peut être rattaché qu'à un univers à la fois.
    .universe_name = nom
    .universe_name-description = Nom du nouvel Univers
    
start = start
    .description = Affiche les instruction de démarrage dans le salon actuel.

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
    Pour continuer, nous te recommandons désormais de setup les rôles avec la commande /roles.

start_message = Merci d'avoir choisis RpBot pour administrer votre serveur RP.
    La première étape consiste à créer un univers : `/{$create_universe}`
    Ensuite vous devrez faire le setup des roles necessaires à l'administration : `/`
    Une fois ceci fait, vous pourrez créer vos premier lieux : `/`
    Puis vos premières routes : `/`
