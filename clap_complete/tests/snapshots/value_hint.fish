complete -c my-app -l choice -r -f -a "{bash	,fish	,zsh	}"
complete -c my-app -l unknown -r
complete -c my-app -l other -r -f
complete -c my-app -s p -l path -r -F
complete -c my-app -s f -l file -r -F
complete -c my-app -s d -l dir -r -f -a "(__fish_complete_directories)"
complete -c my-app -s e -l exe -r -F
complete -c my-app -l cmd-name -r -f -a "(__fish_complete_command)"
complete -c my-app -s c -l cmd -r -f -a "(__fish_complete_command)"
complete -c my-app -s u -l user -r -f -a "(__fish_complete_users)"
complete -c my-app -s H -l host -r -f -a "(__fish_print_hostnames)"
complete -c my-app -l url -r -f
complete -c my-app -l email -r -f
complete -c my-app -s h -l help -d 'Print help'
