complete -c my-app -n "__fish_use_subcommand" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_use_subcommand" -f -a "foo"
complete -c my-app -n "__fish_use_subcommand" -f -a "bar"
complete -c my-app -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_seen_subcommand_from foo" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_seen_subcommand_from bar" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from foo bar help" -f -a "foo"
complete -c my-app -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from foo bar help" -f -a "bar"
complete -c my-app -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from foo bar help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
