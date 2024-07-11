# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_my_app_global_optspecs
	string join \n c v h/help
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

complete -c my-app -n "__fish_my_app_needs_command" -s c
complete -c my-app -n "__fish_my_app_needs_command" -s v
complete -c my-app -n "__fish_my_app_needs_command" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_my_app_needs_command" -f -a "test" -d 'Subcommand with a second line'
complete -c my-app -n "__fish_my_app_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_my_app_using_subcommand test" -s d
complete -c my-app -n "__fish_my_app_using_subcommand test" -s c
complete -c my-app -n "__fish_my_app_using_subcommand test" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_my_app_using_subcommand help; and not __fish_seen_subcommand_from test help" -f -a "test" -d 'Subcommand with a second line'
complete -c my-app -n "__fish_my_app_using_subcommand help; and not __fish_seen_subcommand_from test help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
