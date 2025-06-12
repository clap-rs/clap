# Chaînes de localisation clap principales - Français

# Aide et version
print-help = Afficher l'aide
print-version = Afficher la version
clap-print-help = Affiche ce message ou l'aide de la/des sous-commande(s) donnée(s)

# Formatage de l'usage
usage-header = Utilisation
usage-options = OPTIONS
usage-default-subcommand = COMMANDE

# Messages d'erreur
clap-error-arg-undefined = L'argument `{$id}` n'est pas défini
clap-error-group-undefined = Le groupe `{$id}` n'est pas défini
clap-error-command-undefined = La commande `{$name}` n'est pas définie

# Messages d'aide pour les fonctionnalités intégrées
help-short-help = Afficher l'aide (voir plus avec '--help')
help-long-help = Afficher l'aide (voir un résumé avec '-h')

# Formatage de l'aide
help-commands = Commande

# Messages d'erreur
error-unrecognized-subcommand = Sous-commande non reconnue '{ $subcommand }'

# Système d'erreur principal
error-unknown-cause = cause inconnue
error-label = erreur
error-tip = conseil

# Erreurs de conflit d'arguments
error-argument-cannot-be-used-multiple-times = l'argument '{ $argument }' ne peut pas être utilisé plusieurs fois
error-argument-cannot-be-used-with = l'argument '{ $argument }' ne peut pas être utilisé avec
error-subcommand-cannot-be-used-with = la sous-commande '{ $subcommand }' ne peut pas être utilisée avec
error-one-or-more-other-arguments = un ou plusieurs des autres arguments spécifiés

# Erreurs de valeur et d'assignation
error-equal-sign-needed = un signe égal est nécessaire lors de l'assignation de valeurs à '{ $argument }'
error-value-required-but-none-supplied = une valeur est requise pour '{ $argument }' mais aucune n'a été fournie
error-invalid-value-for-argument = valeur invalide '{ $value }' pour '{ $argument }'
error-possible-values = valeurs possibles

# Erreurs de sous-commande
error-requires-subcommand = '{ $command }' nécessite une sous-commande mais aucune n'a été fournie
error-subcommands = sous-commandes

# Arguments manquants
error-missing-required-arguments = les arguments requis suivants n'ont pas été fournis:

# Erreurs de nombre de valeurs
error-unexpected-value-no-more-expected = valeur inattendue '{ $value }' pour '{ $argument }' trouvée ; aucune autre n'était attendue
error-values-required-only-provided = { $min_values } valeurs requises par '{ $argument }' ; seulement { $actual_values } { $were_provided }
error-wrong-number-of-values = { $expected_values } valeurs requises pour '{ $argument }' mais { $actual_values } { $were_provided }
error-were-provided = ont été fournies
error-was-provided = a été fournie

# Erreurs d'argument inconnu
error-unexpected-argument = argument inattendu '{ $argument }' trouvé

# Messages d'aide
error-for-more-information-try = Pour plus d'informations, essayez '{ $help }'.

# Types de contexte pour les suggestions
error-context-subcommand = sous-commande
error-context-argument = argument
error-context-value = valeur
error-context-subcommands = sous-commandes
error-context-arguments = arguments
error-context-values = valeurs

# Messages de suggestion
error-similar-exists-singular = un { $context } similaire existe : '{ $suggestion }'
error-similar-exists-plural = des { $context } similaires existent : '{ $suggestion }'
