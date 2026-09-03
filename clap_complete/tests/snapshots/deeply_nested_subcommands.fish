# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_my_app_global_optspecs
    string join \n h/help
end

function __fish_my_app_needs_command
    # Figure out if the current invocation already has a command.
    set -l cmd (commandline -opc)
    set -e cmd[1]
    argparse -s (__fish_my_app_global_optspecs) -- $cmd 2>/dev/null
    or return
    if set -q argv[1]
        # Also print the command, so this can be used to figure out what it is.
        echo $argv[1]
        return 1
    end
    return 0
end

function __fish_my_app_using_subcommand
    set -l cmd (__fish_my_app_needs_command)
    test -z "$cmd"
    and return 1
    contains -- $cmd[1] $argv
end

complete -c my-app -n "__fish_my_app_needs_command" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_my_app_needs_command" -f -a "first"
complete -c my-app -n "__fish_my_app_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_my_app_using_subcommand first; and not __fish_seen_subcommand_from second help" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_my_app_using_subcommand first; and not __fish_seen_subcommand_from second help" -f -a "second"
complete -c my-app -n "__fish_my_app_using_subcommand first; and not __fish_seen_subcommand_from second help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_my_app_using_subcommand first; and __fish_seen_subcommand_from second; and not __fish_seen_subcommand_from third help" -l second-flag
complete -c my-app -n "__fish_my_app_using_subcommand first; and __fish_seen_subcommand_from second; and not __fish_seen_subcommand_from third help" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_my_app_using_subcommand first; and __fish_seen_subcommand_from second; and not __fish_seen_subcommand_from third help" -f -a "third"
complete -c my-app -n "__fish_my_app_using_subcommand first; and __fish_seen_subcommand_from second; and not __fish_seen_subcommand_from third help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_my_app_using_subcommand first; and __fish_seen_subcommand_from second; and __fish_seen_subcommand_from third" -l third-flag
complete -c my-app -n "__fish_my_app_using_subcommand first; and __fish_seen_subcommand_from second; and __fish_seen_subcommand_from third" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_my_app_using_subcommand first; and __fish_seen_subcommand_from second; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from third help" -f -a "third"
complete -c my-app -n "__fish_my_app_using_subcommand first; and __fish_seen_subcommand_from second; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from third help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_my_app_using_subcommand first; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from second help" -f -a "second"
complete -c my-app -n "__fish_my_app_using_subcommand first; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from second help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_my_app_using_subcommand first; and __fish_seen_subcommand_from help; and __fish_seen_subcommand_from second; and not __fish_seen_subcommand_from third" -f -a "third"
complete -c my-app -n "__fish_my_app_using_subcommand help; and not __fish_seen_subcommand_from first help" -f -a "first"
complete -c my-app -n "__fish_my_app_using_subcommand help; and not __fish_seen_subcommand_from first help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_my_app_using_subcommand help; and __fish_seen_subcommand_from first; and not __fish_seen_subcommand_from second" -f -a "second"
complete -c my-app -n "__fish_my_app_using_subcommand help; and __fish_seen_subcommand_from first; and __fish_seen_subcommand_from second; and not __fish_seen_subcommand_from third" -f -a "third"
