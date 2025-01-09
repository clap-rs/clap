# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_my_app_global_optspecs
	string join \n c/config h/help V/version
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

complete -c my-app -n "__fish_my_app_needs_command" -s c -s C -l config -l conf -d 'some config file'
complete -c my-app -n "__fish_my_app_needs_command" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_my_app_needs_command" -s V -l version -d 'Print version'
complete -c my-app -n "__fish_my_app_needs_command" -a "test" -d 'tests things'
complete -c my-app -n "__fish_my_app_needs_command" -a "some_cmd" -d 'top level subcommand'
complete -c my-app -n "__fish_my_app_needs_command" -a "some_cmd_alias" -d 'top level subcommand'
complete -c my-app -n "__fish_my_app_needs_command" -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_my_app_using_subcommand test" -l case -d 'the case to test' -r
complete -c my-app -n "__fish_my_app_using_subcommand test" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_my_app_using_subcommand test" -s V -l version -d 'Print version'
complete -c my-app -n "__fish_my_app_using_subcommand some_cmd; and not __fish_seen_subcommand_from sub_cmd help" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_my_app_using_subcommand some_cmd; and not __fish_seen_subcommand_from sub_cmd help" -s V -l version -d 'Print version'
complete -c my-app -n "__fish_my_app_using_subcommand some_cmd; and not __fish_seen_subcommand_from sub_cmd help" -f -a "sub_cmd" -d 'sub-subcommand'
complete -c my-app -n "__fish_my_app_using_subcommand some_cmd; and not __fish_seen_subcommand_from sub_cmd help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_my_app_using_subcommand some_cmd; and __fish_seen_subcommand_from sub_cmd" -l config -d 'the other case to test' -r -f -a "Lest quotes\, aren\'t escaped.\t'help,with,comma'
Second to trigger display of options\t''"
complete -c my-app -n "__fish_my_app_using_subcommand some_cmd; and __fish_seen_subcommand_from sub_cmd" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c my-app -n "__fish_my_app_using_subcommand some_cmd; and __fish_seen_subcommand_from sub_cmd" -s V -l version -d 'Print version'
complete -c my-app -n "__fish_my_app_using_subcommand some_cmd; and __fish_seen_subcommand_from help" -f -a "sub_cmd" -d 'sub-subcommand'
complete -c my-app -n "__fish_my_app_using_subcommand some_cmd; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_my_app_using_subcommand some_cmd_alias; and not __fish_seen_subcommand_from sub_cmd help" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_my_app_using_subcommand some_cmd_alias; and not __fish_seen_subcommand_from sub_cmd help" -s V -l version -d 'Print version'
complete -c my-app -n "__fish_my_app_using_subcommand some_cmd_alias; and not __fish_seen_subcommand_from sub_cmd help" -f -a "sub_cmd" -d 'sub-subcommand'
complete -c my-app -n "__fish_my_app_using_subcommand some_cmd_alias; and not __fish_seen_subcommand_from sub_cmd help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_my_app_using_subcommand some_cmd_alias; and __fish_seen_subcommand_from sub_cmd" -l config -d 'the other case to test' -r -f -a "Lest quotes\, aren\'t escaped.\t'help,with,comma'
Second to trigger display of options\t''"
complete -c my-app -n "__fish_my_app_using_subcommand some_cmd_alias; and __fish_seen_subcommand_from sub_cmd" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c my-app -n "__fish_my_app_using_subcommand some_cmd_alias; and __fish_seen_subcommand_from sub_cmd" -s V -l version -d 'Print version'
complete -c my-app -n "__fish_my_app_using_subcommand some_cmd_alias; and __fish_seen_subcommand_from help" -f -a "sub_cmd" -d 'sub-subcommand'
complete -c my-app -n "__fish_my_app_using_subcommand some_cmd_alias; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_my_app_using_subcommand help; and not __fish_seen_subcommand_from test some_cmd help" -f -a "test" -d 'tests things'
complete -c my-app -n "__fish_my_app_using_subcommand help; and not __fish_seen_subcommand_from test some_cmd help" -f -a "some_cmd" -d 'top level subcommand'
complete -c my-app -n "__fish_my_app_using_subcommand help; and not __fish_seen_subcommand_from test some_cmd help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_my_app_using_subcommand help; and __fish_seen_subcommand_from some_cmd" -f -a "sub_cmd" -d 'sub-subcommand'
