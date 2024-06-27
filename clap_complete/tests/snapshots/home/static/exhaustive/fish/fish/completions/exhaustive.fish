complete -c exhaustive -n "__fish_use_subcommand" -l generate -d 'generate' -r -f -a "{bash\t'',elvish\t'',fish\t'',powershell\t'',zsh\t''}"
complete -c exhaustive -n "__fish_use_subcommand" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_use_subcommand" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_use_subcommand" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_use_subcommand" -f -a "action"
complete -c exhaustive -n "__fish_use_subcommand" -f -a "quote"
complete -c exhaustive -n "__fish_use_subcommand" -f -a "value"
complete -c exhaustive -n "__fish_use_subcommand" -f -a "pacman"
complete -c exhaustive -n "__fish_use_subcommand" -f -a "last"
complete -c exhaustive -n "__fish_use_subcommand" -f -a "alias"
complete -c exhaustive -n "__fish_use_subcommand" -f -a "hint"
complete -c exhaustive -n "__fish_use_subcommand" -f -a "complete" -d 'Register shell completions for this program'
complete -c exhaustive -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c exhaustive -n "__fish_seen_subcommand_from action" -l set -d 'value' -r
complete -c exhaustive -n "__fish_seen_subcommand_from action" -l choice -d 'enum' -r -f -a "{first\t'',second\t''}"
complete -c exhaustive -n "__fish_seen_subcommand_from action" -l set-true -d 'bool'
complete -c exhaustive -n "__fish_seen_subcommand_from action" -l count -d 'number'
complete -c exhaustive -n "__fish_seen_subcommand_from action" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_seen_subcommand_from action" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_seen_subcommand_from action" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_seen_subcommand_from quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -l choice -r -f -a "{bash\t'bash (shell)',fish\t'fish shell',zsh\t'zsh shell'}"
complete -c exhaustive -n "__fish_seen_subcommand_from quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -l single-quotes -d 'Can be \'always\', \'auto\', or \'never\''
complete -c exhaustive -n "__fish_seen_subcommand_from quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -l double-quotes -d 'Can be "always", "auto", or "never"'
complete -c exhaustive -n "__fish_seen_subcommand_from quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -l backticks -d 'For more information see `echo test`'
complete -c exhaustive -n "__fish_seen_subcommand_from quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -l backslash -d 'Avoid \'\\n\''
complete -c exhaustive -n "__fish_seen_subcommand_from quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -l brackets -d 'List packages [filter]'
complete -c exhaustive -n "__fish_seen_subcommand_from quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -l expansions -d 'Execute the shell command with $SHELL'
complete -c exhaustive -n "__fish_seen_subcommand_from quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_seen_subcommand_from quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c exhaustive -n "__fish_seen_subcommand_from quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_seen_subcommand_from quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "cmd-single-quotes" -d 'Can be \'always\', \'auto\', or \'never\''
complete -c exhaustive -n "__fish_seen_subcommand_from quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "cmd-double-quotes" -d 'Can be "always", "auto", or "never"'
complete -c exhaustive -n "__fish_seen_subcommand_from quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "cmd-backticks" -d 'For more information see `echo test`'
complete -c exhaustive -n "__fish_seen_subcommand_from quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "cmd-backslash" -d 'Avoid \'\\n\''
complete -c exhaustive -n "__fish_seen_subcommand_from quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "cmd-brackets" -d 'List packages [filter]'
complete -c exhaustive -n "__fish_seen_subcommand_from quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "cmd-expansions" -d 'Execute the shell command with $SHELL'
complete -c exhaustive -n "__fish_seen_subcommand_from quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "escape-help" -d '\\tab	"\' New Line'
complete -c exhaustive -n "__fish_seen_subcommand_from quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c exhaustive -n "__fish_seen_subcommand_from quote cmd-single-quotes" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_seen_subcommand_from quote cmd-single-quotes" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_seen_subcommand_from quote cmd-single-quotes" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_seen_subcommand_from quote cmd-double-quotes" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_seen_subcommand_from quote cmd-double-quotes" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_seen_subcommand_from quote cmd-double-quotes" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_seen_subcommand_from quote cmd-backticks" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_seen_subcommand_from quote cmd-backticks" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_seen_subcommand_from quote cmd-backticks" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_seen_subcommand_from quote cmd-backslash" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_seen_subcommand_from quote cmd-backslash" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_seen_subcommand_from quote cmd-backslash" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_seen_subcommand_from quote cmd-brackets" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_seen_subcommand_from quote cmd-brackets" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_seen_subcommand_from quote cmd-brackets" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_seen_subcommand_from quote cmd-expansions" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_seen_subcommand_from quote cmd-expansions" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_seen_subcommand_from quote cmd-expansions" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_seen_subcommand_from quote escape-help" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_seen_subcommand_from quote escape-help" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_seen_subcommand_from quote escape-help" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_seen_subcommand_from quote help; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "cmd-single-quotes" -d 'Can be \'always\', \'auto\', or \'never\''
complete -c exhaustive -n "__fish_seen_subcommand_from quote help; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "cmd-double-quotes" -d 'Can be "always", "auto", or "never"'
complete -c exhaustive -n "__fish_seen_subcommand_from quote help; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "cmd-backticks" -d 'For more information see `echo test`'
complete -c exhaustive -n "__fish_seen_subcommand_from quote help; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "cmd-backslash" -d 'Avoid \'\\n\''
complete -c exhaustive -n "__fish_seen_subcommand_from quote help; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "cmd-brackets" -d 'List packages [filter]'
complete -c exhaustive -n "__fish_seen_subcommand_from quote help; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "cmd-expansions" -d 'Execute the shell command with $SHELL'
complete -c exhaustive -n "__fish_seen_subcommand_from quote help; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "escape-help" -d '\\tab	"\' New Line'
complete -c exhaustive -n "__fish_seen_subcommand_from quote help; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c exhaustive -n "__fish_seen_subcommand_from value" -l delim -r
complete -c exhaustive -n "__fish_seen_subcommand_from value" -l tuple -r
complete -c exhaustive -n "__fish_seen_subcommand_from value" -l require-eq -r
complete -c exhaustive -n "__fish_seen_subcommand_from value" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_seen_subcommand_from value" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_seen_subcommand_from value" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_seen_subcommand_from pacman; and not __fish_seen_subcommand_from one two help" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_seen_subcommand_from pacman; and not __fish_seen_subcommand_from one two help" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_seen_subcommand_from pacman; and not __fish_seen_subcommand_from one two help" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_seen_subcommand_from pacman; and not __fish_seen_subcommand_from one two help" -f -a "one"
complete -c exhaustive -n "__fish_seen_subcommand_from pacman; and not __fish_seen_subcommand_from one two help" -f -a "two"
complete -c exhaustive -n "__fish_seen_subcommand_from pacman; and not __fish_seen_subcommand_from one two help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c exhaustive -n "__fish_seen_subcommand_from pacman one" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_seen_subcommand_from pacman one" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_seen_subcommand_from pacman one" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_seen_subcommand_from pacman two" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_seen_subcommand_from pacman two" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_seen_subcommand_from pacman two" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_seen_subcommand_from pacman help; and not __fish_seen_subcommand_from one two help" -f -a "one"
complete -c exhaustive -n "__fish_seen_subcommand_from pacman help; and not __fish_seen_subcommand_from one two help" -f -a "two"
complete -c exhaustive -n "__fish_seen_subcommand_from pacman help; and not __fish_seen_subcommand_from one two help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c exhaustive -n "__fish_seen_subcommand_from last" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_seen_subcommand_from last" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_seen_subcommand_from last" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_seen_subcommand_from alias" -s o -s O -l option -l opt -d 'cmd option' -r
complete -c exhaustive -n "__fish_seen_subcommand_from alias" -s f -s F -l flag -l flg -d 'cmd flag'
complete -c exhaustive -n "__fish_seen_subcommand_from alias" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_seen_subcommand_from alias" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_seen_subcommand_from alias" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_seen_subcommand_from hint" -l choice -r -f -a "{bash\t'',fish\t'',zsh\t''}"
complete -c exhaustive -n "__fish_seen_subcommand_from hint" -l unknown -r
complete -c exhaustive -n "__fish_seen_subcommand_from hint" -l other -r -f
complete -c exhaustive -n "__fish_seen_subcommand_from hint" -s p -l path -r -F
complete -c exhaustive -n "__fish_seen_subcommand_from hint" -s f -l file -r -F
complete -c exhaustive -n "__fish_seen_subcommand_from hint" -s d -l dir -r -f -a "(__fish_complete_directories)"
complete -c exhaustive -n "__fish_seen_subcommand_from hint" -s e -l exe -r -F
complete -c exhaustive -n "__fish_seen_subcommand_from hint" -l cmd-name -r -f -a "(__fish_complete_command)"
complete -c exhaustive -n "__fish_seen_subcommand_from hint" -s c -l cmd -r -f -a "(__fish_complete_command)"
complete -c exhaustive -n "__fish_seen_subcommand_from hint" -s u -l user -r -f -a "(__fish_complete_users)"
complete -c exhaustive -n "__fish_seen_subcommand_from hint" -s H -l host -r -f -a "(__fish_print_hostnames)"
complete -c exhaustive -n "__fish_seen_subcommand_from hint" -l url -r -f
complete -c exhaustive -n "__fish_seen_subcommand_from hint" -l email -r -f
complete -c exhaustive -n "__fish_seen_subcommand_from hint" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_seen_subcommand_from hint" -s h -l help -d 'Print help'
complete -c exhaustive -n "__fish_seen_subcommand_from hint" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_seen_subcommand_from complete" -l shell -d 'Specify shell to complete for' -r -f -a "{bash\t'',fish\t''}"
complete -c exhaustive -n "__fish_seen_subcommand_from complete" -l register -d 'Path to write completion-registration to' -r -F
complete -c exhaustive -n "__fish_seen_subcommand_from complete" -l global -d 'everywhere'
complete -c exhaustive -n "__fish_seen_subcommand_from complete" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c exhaustive -n "__fish_seen_subcommand_from complete" -s V -l version -d 'Print version'
complete -c exhaustive -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from action quote value pacman last alias hint complete help" -f -a "action"
complete -c exhaustive -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from action quote value pacman last alias hint complete help" -f -a "quote"
complete -c exhaustive -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from action quote value pacman last alias hint complete help" -f -a "value"
complete -c exhaustive -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from action quote value pacman last alias hint complete help" -f -a "pacman"
complete -c exhaustive -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from action quote value pacman last alias hint complete help" -f -a "last"
complete -c exhaustive -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from action quote value pacman last alias hint complete help" -f -a "alias"
complete -c exhaustive -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from action quote value pacman last alias hint complete help" -f -a "hint"
complete -c exhaustive -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from action quote value pacman last alias hint complete help" -f -a "complete" -d 'Register shell completions for this program'
complete -c exhaustive -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from action quote value pacman last alias hint complete help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c exhaustive -n "__fish_seen_subcommand_from help quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help" -f -a "cmd-single-quotes" -d 'Can be \'always\', \'auto\', or \'never\''
complete -c exhaustive -n "__fish_seen_subcommand_from help quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help" -f -a "cmd-double-quotes" -d 'Can be "always", "auto", or "never"'
complete -c exhaustive -n "__fish_seen_subcommand_from help quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help" -f -a "cmd-backticks" -d 'For more information see `echo test`'
complete -c exhaustive -n "__fish_seen_subcommand_from help quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help" -f -a "cmd-backslash" -d 'Avoid \'\\n\''
complete -c exhaustive -n "__fish_seen_subcommand_from help quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help" -f -a "cmd-brackets" -d 'List packages [filter]'
complete -c exhaustive -n "__fish_seen_subcommand_from help quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help" -f -a "cmd-expansions" -d 'Execute the shell command with $SHELL'
complete -c exhaustive -n "__fish_seen_subcommand_from help quote; and not __fish_seen_subcommand_from cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help" -f -a "escape-help" -d '\\tab	"\' New Line'
complete -c exhaustive -n "__fish_seen_subcommand_from help pacman; and not __fish_seen_subcommand_from one two" -f -a "one"
complete -c exhaustive -n "__fish_seen_subcommand_from help pacman; and not __fish_seen_subcommand_from one two" -f -a "two"
