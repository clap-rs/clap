# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_exhaustive_global_optspecs
	string join \n generate= empty-choice= h/help
end

function __fish_exhaustive_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_exhaustive_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_exhaustive_using_subcommand
	set -l cmd (__fish_exhaustive_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c exhaustive -n "__fish_exhaustive_needs_command" -l generate -d 'generate' -r -f -a "bash\t''
elvish\t''
fish\t''
powershell\t''
zsh\t''"
complete -c exhaustive -n "__fish_exhaustive_needs_command" -l empty-choice -r -f -a ""
complete -c exhaustive -n "__fish_exhaustive_needs_command" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_needs_command" -f -a "empty"
complete -c exhaustive -n "__fish_exhaustive_needs_command" -f -a "global"
complete -c exhaustive -n "__fish_exhaustive_needs_command" -f -a "action"
complete -c exhaustive -n "__fish_exhaustive_needs_command" -f -a "quote"
complete -c exhaustive -n "__fish_exhaustive_needs_command" -f -a "value"
complete -c exhaustive -n "__fish_exhaustive_needs_command" -f -a "pacman"
complete -c exhaustive -n "__fish_exhaustive_needs_command" -f -a "last"
complete -c exhaustive -n "__fish_exhaustive_needs_command" -f -a "alias"
complete -c exhaustive -n "__fish_exhaustive_needs_command" -f -a "hint"
complete -c exhaustive -n "__fish_exhaustive_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand global; and not __fish_seen_subcommand_from one two help" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand global; and not __fish_seen_subcommand_from one two help" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand global; and not __fish_seen_subcommand_from one two help" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand global; and not __fish_seen_subcommand_from one two help" -f -a "one"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand global; and not __fish_seen_subcommand_from one two help" -f -a "two"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand global; and not __fish_seen_subcommand_from one two help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand global; and __fish_seen_subcommand_from one" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand global; and __fish_seen_subcommand_from one" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand global; and __fish_seen_subcommand_from one" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand global; and __fish_seen_subcommand_from one" -f -a "one-one"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand global; and __fish_seen_subcommand_from one" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand global; and __fish_seen_subcommand_from two" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand global; and __fish_seen_subcommand_from two" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand global; and __fish_seen_subcommand_from two" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand global; and __fish_seen_subcommand_from help" -f -a "one"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand global; and __fish_seen_subcommand_from help" -f -a "two"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand global; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand action" -l set -d 'value' -r
complete -c exhaustive -n "__fish_exhaustive_using_subcommand action" -l choice -d 'enum' -r -f -a "first\t''
second\t''"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand action" -l set-true -d 'bool'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand action" -l count -d 'number'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand action" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -l choice -r -f -a "another shell\t'something with a space'
bash\t'bash (shell)'
fish\t'fish shell'
zsh\t'zsh shell'"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -l single-quotes -d 'Can be \'always\', \'auto\', or \'never\''
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -l double-quotes -d 'Can be "always", "auto", or "never"'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -l backticks -d 'For more information see `echo test`'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -l backslash -d 'Avoid \'\\n\''
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -l brackets -d 'List packages [filter]'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -l expansions -d 'Execute the shell command with $SHELL'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "cmd-single-quotes" -d 'Can be \'always\', \'auto\', or \'never\''
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "cmd-double-quotes" -d 'Can be "always", "auto", or "never"'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "cmd-backticks" -d 'For more information see `echo test`'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "cmd-backslash" -d 'Avoid \'\\n\''
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "cmd-brackets" -d 'List packages [filter]'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "cmd-expansions" -d 'Execute the shell command with $SHELL'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "escape-help" -d '\\tab	"\' New Line'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and __fish_seen_subcommand_from cmd-single-quotes" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and __fish_seen_subcommand_from cmd-double-quotes" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and __fish_seen_subcommand_from cmd-backticks" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and __fish_seen_subcommand_from cmd-backslash" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and __fish_seen_subcommand_from cmd-brackets" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and __fish_seen_subcommand_from cmd-expansions" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and __fish_seen_subcommand_from escape-help" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and __fish_seen_subcommand_from help" -f -a "cmd-single-quotes" -d 'Can be \'always\', \'auto\', or \'never\''
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and __fish_seen_subcommand_from help" -f -a "cmd-double-quotes" -d 'Can be "always", "auto", or "never"'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and __fish_seen_subcommand_from help" -f -a "cmd-backticks" -d 'For more information see `echo test`'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and __fish_seen_subcommand_from help" -f -a "cmd-backslash" -d 'Avoid \'\\n\''
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and __fish_seen_subcommand_from help" -f -a "cmd-brackets" -d 'List packages [filter]'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and __fish_seen_subcommand_from help" -f -a "cmd-expansions" -d 'Execute the shell command with $SHELL'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and __fish_seen_subcommand_from help" -f -a "escape-help" -d '\\tab	"\' New Line'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand quote; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand value" -l delim -r
complete -c exhaustive -n "__fish_exhaustive_using_subcommand value" -l tuple -r
complete -c exhaustive -n "__fish_exhaustive_using_subcommand value" -l require-eq -r
complete -c exhaustive -n "__fish_exhaustive_using_subcommand value" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand pacman; and not __fish_seen_subcommand_from one two help" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand pacman; and not __fish_seen_subcommand_from one two help" -f -a "one"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand pacman; and not __fish_seen_subcommand_from one two help" -f -a "two"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand pacman; and not __fish_seen_subcommand_from one two help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand pacman; and __fish_seen_subcommand_from one" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand pacman; and __fish_seen_subcommand_from two" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand pacman; and __fish_seen_subcommand_from help" -f -a "one"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand pacman; and __fish_seen_subcommand_from help" -f -a "two"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand pacman; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand last" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand alias" -s o -s O -l option -l opt -d 'cmd option' -r
complete -c exhaustive -n "__fish_exhaustive_using_subcommand alias" -s f -s F -l flag -l flg -d 'cmd flag'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand alias" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand hint" -l choice -r -f -a "bash\t''
fish\t''
zsh\t''"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand hint" -l unknown -r
complete -c exhaustive -n "__fish_exhaustive_using_subcommand hint" -l other -r -f
complete -c exhaustive -n "__fish_exhaustive_using_subcommand hint" -s p -l path -r -F
complete -c exhaustive -n "__fish_exhaustive_using_subcommand hint" -s f -l file -r -F
complete -c exhaustive -n "__fish_exhaustive_using_subcommand hint" -s d -l dir -r -f -a "(__fish_complete_directories)"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand hint" -s e -l exe -r -F
complete -c exhaustive -n "__fish_exhaustive_using_subcommand hint" -l cmd-name -r -f -a "(__fish_complete_command)"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand hint" -s c -l cmd -r -f -a "(__fish_complete_command)"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand hint" -s u -l user -r -f -a "(__fish_complete_users)"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand hint" -s H -l host -r -f -a "(__fish_print_hostnames)"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand hint" -l url -r -f
complete -c exhaustive -n "__fish_exhaustive_using_subcommand hint" -l email -r -f
complete -c exhaustive -n "__fish_exhaustive_using_subcommand hint" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and not __fish_seen_subcommand_from empty global action quote value pacman last alias hint help" -f -a "empty"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and not __fish_seen_subcommand_from empty global action quote value pacman last alias hint help" -f -a "global"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and not __fish_seen_subcommand_from empty global action quote value pacman last alias hint help" -f -a "action"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and not __fish_seen_subcommand_from empty global action quote value pacman last alias hint help" -f -a "quote"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and not __fish_seen_subcommand_from empty global action quote value pacman last alias hint help" -f -a "value"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and not __fish_seen_subcommand_from empty global action quote value pacman last alias hint help" -f -a "pacman"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and not __fish_seen_subcommand_from empty global action quote value pacman last alias hint help" -f -a "last"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and not __fish_seen_subcommand_from empty global action quote value pacman last alias hint help" -f -a "alias"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and not __fish_seen_subcommand_from empty global action quote value pacman last alias hint help" -f -a "hint"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and not __fish_seen_subcommand_from empty global action quote value pacman last alias hint help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and __fish_seen_subcommand_from global" -f -a "one"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and __fish_seen_subcommand_from global" -f -a "two"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and __fish_seen_subcommand_from quote" -f -a "cmd-single-quotes" -d 'Can be \'always\', \'auto\', or \'never\''
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and __fish_seen_subcommand_from quote" -f -a "cmd-double-quotes" -d 'Can be "always", "auto", or "never"'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and __fish_seen_subcommand_from quote" -f -a "cmd-backticks" -d 'For more information see `echo test`'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and __fish_seen_subcommand_from quote" -f -a "cmd-backslash" -d 'Avoid \'\\n\''
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and __fish_seen_subcommand_from quote" -f -a "cmd-brackets" -d 'List packages [filter]'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and __fish_seen_subcommand_from quote" -f -a "cmd-expansions" -d 'Execute the shell command with $SHELL'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and __fish_seen_subcommand_from quote" -f -a "escape-help" -d '\\tab	"\' New Line'
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and __fish_seen_subcommand_from pacman" -f -a "one"
complete -c exhaustive -n "__fish_exhaustive_using_subcommand help; and __fish_seen_subcommand_from pacman" -f -a "two"
