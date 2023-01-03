complete -c my-app -n "__fish_use_subcommand" -s c -s C -l config -l conf -d 'some config file'
complete -c my-app -n "__fish_use_subcommand" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_use_subcommand" -s V -l version -d 'Print version'
complete -c my-app -n "__fish_use_subcommand" -f -a "test" -d 'tests things'
complete -c my-app -n "__fish_use_subcommand" -f -a "some_cmd" -d 'tests other things'
complete -c my-app -n "__fish_use_subcommand" -f -a "some-cmd-with-hyphens"
complete -c my-app -n "__fish_use_subcommand" -f -a "some-hidden-cmd"
complete -c my-app -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_seen_subcommand_from test" -l case -d 'the case to test' -r
complete -c my-app -n "__fish_seen_subcommand_from test" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_seen_subcommand_from test" -s V -l version -d 'Print version'
complete -c my-app -n "__fish_seen_subcommand_from some_cmd" -l config -d 'the other case to test' -r
complete -c my-app -n "__fish_seen_subcommand_from some_cmd" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_seen_subcommand_from some_cmd" -s V -l version -d 'Print version'
complete -c my-app -n "__fish_seen_subcommand_from some-cmd-with-hyphens" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_seen_subcommand_from some-cmd-with-hyphens" -s V -l version -d 'Print version'
complete -c my-app -n "__fish_seen_subcommand_from some-hidden-cmd" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_seen_subcommand_from some-hidden-cmd" -s V -l version -d 'Print version'
complete -c my-app -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from test; and not __fish_seen_subcommand_from some_cmd; and not __fish_seen_subcommand_from some-cmd-with-hyphens; and not __fish_seen_subcommand_from some-hidden-cmd; and not __fish_seen_subcommand_from help" -f -a "test" -d 'tests things'
complete -c my-app -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from test; and not __fish_seen_subcommand_from some_cmd; and not __fish_seen_subcommand_from some-cmd-with-hyphens; and not __fish_seen_subcommand_from some-hidden-cmd; and not __fish_seen_subcommand_from help" -f -a "some_cmd" -d 'tests other things'
complete -c my-app -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from test; and not __fish_seen_subcommand_from some_cmd; and not __fish_seen_subcommand_from some-cmd-with-hyphens; and not __fish_seen_subcommand_from some-hidden-cmd; and not __fish_seen_subcommand_from help" -f -a "some-cmd-with-hyphens"
complete -c my-app -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from test; and not __fish_seen_subcommand_from some_cmd; and not __fish_seen_subcommand_from some-cmd-with-hyphens; and not __fish_seen_subcommand_from some-hidden-cmd; and not __fish_seen_subcommand_from help" -f -a "some-hidden-cmd"
complete -c my-app -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from test; and not __fish_seen_subcommand_from some_cmd; and not __fish_seen_subcommand_from some-cmd-with-hyphens; and not __fish_seen_subcommand_from some-hidden-cmd; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
