PS1='% '
. /etc/bash_completion
_exhaustive() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    for i in "${COMP_WORDS[@]:0:COMP_CWORD}"
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="exhaustive"
                ;;
            exhaustive,action)
                cmd="exhaustive__subcmd__action"
                ;;
            exhaustive,alias)
                cmd="exhaustive__subcmd__alias"
                ;;
            exhaustive,empty)
                cmd="exhaustive__subcmd__empty"
                ;;
            exhaustive,global)
                cmd="exhaustive__subcmd__global"
                ;;
            exhaustive,help)
                cmd="exhaustive__subcmd__help"
                ;;
            exhaustive,hint)
                cmd="exhaustive__subcmd__hint"
                ;;
            exhaustive,last)
                cmd="exhaustive__subcmd__last"
                ;;
            exhaustive,pacman)
                cmd="exhaustive__subcmd__pacman"
                ;;
            exhaustive,quote)
                cmd="exhaustive__subcmd__quote"
                ;;
            exhaustive,value)
                cmd="exhaustive__subcmd__value"
                ;;
            exhaustive__subcmd__global,help)
                cmd="exhaustive__subcmd__global__subcmd__help"
                ;;
            exhaustive__subcmd__global,one)
                cmd="exhaustive__subcmd__global__subcmd__one"
                ;;
            exhaustive__subcmd__global,two)
                cmd="exhaustive__subcmd__global__subcmd__two"
                ;;
            exhaustive__subcmd__global__subcmd__help,help)
                cmd="exhaustive__subcmd__global__subcmd__help__subcmd__help"
                ;;
            exhaustive__subcmd__global__subcmd__help,one)
                cmd="exhaustive__subcmd__global__subcmd__help__subcmd__one"
                ;;
            exhaustive__subcmd__global__subcmd__help,two)
                cmd="exhaustive__subcmd__global__subcmd__help__subcmd__two"
                ;;
            exhaustive__subcmd__global__subcmd__help__subcmd__one,one-one)
                cmd="exhaustive__subcmd__global__subcmd__help__subcmd__one__subcmd__one__subcmd__one"
                ;;
            exhaustive__subcmd__global__subcmd__one,help)
                cmd="exhaustive__subcmd__global__subcmd__one__subcmd__help"
                ;;
            exhaustive__subcmd__global__subcmd__one,one-one)
                cmd="exhaustive__subcmd__global__subcmd__one__subcmd__one__subcmd__one"
                ;;
            exhaustive__subcmd__global__subcmd__one__subcmd__help,help)
                cmd="exhaustive__subcmd__global__subcmd__one__subcmd__help__subcmd__help"
                ;;
            exhaustive__subcmd__global__subcmd__one__subcmd__help,one-one)
                cmd="exhaustive__subcmd__global__subcmd__one__subcmd__help__subcmd__one__subcmd__one"
                ;;
            exhaustive__subcmd__help,action)
                cmd="exhaustive__subcmd__help__subcmd__action"
                ;;
            exhaustive__subcmd__help,alias)
                cmd="exhaustive__subcmd__help__subcmd__alias"
                ;;
            exhaustive__subcmd__help,empty)
                cmd="exhaustive__subcmd__help__subcmd__empty"
                ;;
            exhaustive__subcmd__help,global)
                cmd="exhaustive__subcmd__help__subcmd__global"
                ;;
            exhaustive__subcmd__help,help)
                cmd="exhaustive__subcmd__help__subcmd__help"
                ;;
            exhaustive__subcmd__help,hint)
                cmd="exhaustive__subcmd__help__subcmd__hint"
                ;;
            exhaustive__subcmd__help,last)
                cmd="exhaustive__subcmd__help__subcmd__last"
                ;;
            exhaustive__subcmd__help,pacman)
                cmd="exhaustive__subcmd__help__subcmd__pacman"
                ;;
            exhaustive__subcmd__help,quote)
                cmd="exhaustive__subcmd__help__subcmd__quote"
                ;;
            exhaustive__subcmd__help,value)
                cmd="exhaustive__subcmd__help__subcmd__value"
                ;;
            exhaustive__subcmd__help__subcmd__global,one)
                cmd="exhaustive__subcmd__help__subcmd__global__subcmd__one"
                ;;
            exhaustive__subcmd__help__subcmd__global,two)
                cmd="exhaustive__subcmd__help__subcmd__global__subcmd__two"
                ;;
            exhaustive__subcmd__help__subcmd__global__subcmd__one,one-one)
                cmd="exhaustive__subcmd__help__subcmd__global__subcmd__one__subcmd__one__subcmd__one"
                ;;
            exhaustive__subcmd__help__subcmd__pacman,one)
                cmd="exhaustive__subcmd__help__subcmd__pacman__subcmd__one"
                ;;
            exhaustive__subcmd__help__subcmd__pacman,two)
                cmd="exhaustive__subcmd__help__subcmd__pacman__subcmd__two"
                ;;
            exhaustive__subcmd__help__subcmd__quote,cmd-backslash)
                cmd="exhaustive__subcmd__help__subcmd__quote__subcmd__cmd__subcmd__backslash"
                ;;
            exhaustive__subcmd__help__subcmd__quote,cmd-backticks)
                cmd="exhaustive__subcmd__help__subcmd__quote__subcmd__cmd__subcmd__backticks"
                ;;
            exhaustive__subcmd__help__subcmd__quote,cmd-brackets)
                cmd="exhaustive__subcmd__help__subcmd__quote__subcmd__cmd__subcmd__brackets"
                ;;
            exhaustive__subcmd__help__subcmd__quote,cmd-double-quotes)
                cmd="exhaustive__subcmd__help__subcmd__quote__subcmd__cmd__subcmd__double__subcmd__quotes"
                ;;
            exhaustive__subcmd__help__subcmd__quote,cmd-expansions)
                cmd="exhaustive__subcmd__help__subcmd__quote__subcmd__cmd__subcmd__expansions"
                ;;
            exhaustive__subcmd__help__subcmd__quote,cmd-single-quotes)
                cmd="exhaustive__subcmd__help__subcmd__quote__subcmd__cmd__subcmd__single__subcmd__quotes"
                ;;
            exhaustive__subcmd__help__subcmd__quote,escape-help)
                cmd="exhaustive__subcmd__help__subcmd__quote__subcmd__escape__subcmd__help"
                ;;
            exhaustive__subcmd__pacman,help)
                cmd="exhaustive__subcmd__pacman__subcmd__help"
                ;;
            exhaustive__subcmd__pacman,one)
                cmd="exhaustive__subcmd__pacman__subcmd__one"
                ;;
            exhaustive__subcmd__pacman,two)
                cmd="exhaustive__subcmd__pacman__subcmd__two"
                ;;
            exhaustive__subcmd__pacman__subcmd__help,help)
                cmd="exhaustive__subcmd__pacman__subcmd__help__subcmd__help"
                ;;
            exhaustive__subcmd__pacman__subcmd__help,one)
                cmd="exhaustive__subcmd__pacman__subcmd__help__subcmd__one"
                ;;
            exhaustive__subcmd__pacman__subcmd__help,two)
                cmd="exhaustive__subcmd__pacman__subcmd__help__subcmd__two"
                ;;
            exhaustive__subcmd__quote,cmd-backslash)
                cmd="exhaustive__subcmd__quote__subcmd__cmd__subcmd__backslash"
                ;;
            exhaustive__subcmd__quote,cmd-backticks)
                cmd="exhaustive__subcmd__quote__subcmd__cmd__subcmd__backticks"
                ;;
            exhaustive__subcmd__quote,cmd-brackets)
                cmd="exhaustive__subcmd__quote__subcmd__cmd__subcmd__brackets"
                ;;
            exhaustive__subcmd__quote,cmd-double-quotes)
                cmd="exhaustive__subcmd__quote__subcmd__cmd__subcmd__double__subcmd__quotes"
                ;;
            exhaustive__subcmd__quote,cmd-expansions)
                cmd="exhaustive__subcmd__quote__subcmd__cmd__subcmd__expansions"
                ;;
            exhaustive__subcmd__quote,cmd-single-quotes)
                cmd="exhaustive__subcmd__quote__subcmd__cmd__subcmd__single__subcmd__quotes"
                ;;
            exhaustive__subcmd__quote,escape-help)
                cmd="exhaustive__subcmd__quote__subcmd__escape__subcmd__help"
                ;;
            exhaustive__subcmd__quote,help)
                cmd="exhaustive__subcmd__quote__subcmd__help"
                ;;
            exhaustive__subcmd__quote__subcmd__help,cmd-backslash)
                cmd="exhaustive__subcmd__quote__subcmd__help__subcmd__cmd__subcmd__backslash"
                ;;
            exhaustive__subcmd__quote__subcmd__help,cmd-backticks)
                cmd="exhaustive__subcmd__quote__subcmd__help__subcmd__cmd__subcmd__backticks"
                ;;
            exhaustive__subcmd__quote__subcmd__help,cmd-brackets)
                cmd="exhaustive__subcmd__quote__subcmd__help__subcmd__cmd__subcmd__brackets"
                ;;
            exhaustive__subcmd__quote__subcmd__help,cmd-double-quotes)
                cmd="exhaustive__subcmd__quote__subcmd__help__subcmd__cmd__subcmd__double__subcmd__quotes"
                ;;
            exhaustive__subcmd__quote__subcmd__help,cmd-expansions)
                cmd="exhaustive__subcmd__quote__subcmd__help__subcmd__cmd__subcmd__expansions"
                ;;
            exhaustive__subcmd__quote__subcmd__help,cmd-single-quotes)
                cmd="exhaustive__subcmd__quote__subcmd__help__subcmd__cmd__subcmd__single__subcmd__quotes"
                ;;
            exhaustive__subcmd__quote__subcmd__help,escape-help)
                cmd="exhaustive__subcmd__quote__subcmd__help__subcmd__escape__subcmd__help"
                ;;
            exhaustive__subcmd__quote__subcmd__help,help)
                cmd="exhaustive__subcmd__quote__subcmd__help__subcmd__help"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        exhaustive)
            opts="-h --generate --empty-choice --help empty global action quote value pacman last alias hint help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --generate)
                    COMPREPLY=($(compgen -W "bash elvish fish powershell zsh" -- "${cur}"))
                    return 0
                    ;;
                --empty-choice)
                    COMPREPLY=($(compgen -W "" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__action)
            opts="-h --set-true --set --count --choice --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --set)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --choice)
                    COMPREPLY=($(compgen -W "first second" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__alias)
            opts="-F -f -O -o -h --flg --flag --opt --option --help [positional]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --option)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --opt)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -O)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__empty)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__global)
            opts="-h -V --global --help --version one two help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__global__subcmd__help)
            opts="one two help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__global__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__global__subcmd__help__subcmd__one)
            opts="one-one"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__global__subcmd__help__subcmd__one__subcmd__one__subcmd__one)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__global__subcmd__help__subcmd__two)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__global__subcmd__one)
            opts="-h -V --global --help --version one-one help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__global__subcmd__one__subcmd__help)
            opts="one-one help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__global__subcmd__one__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__global__subcmd__one__subcmd__help__subcmd__one__subcmd__one)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__global__subcmd__one__subcmd__one__subcmd__one)
            opts="-h -V --global --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__global__subcmd__two)
            opts="-h -V --global --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help)
            opts="empty global action quote value pacman last alias hint help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__action)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__alias)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__empty)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__global)
            opts="one two"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__global__subcmd__one)
            opts="one-one"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__global__subcmd__one__subcmd__one__subcmd__one)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__global__subcmd__two)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__hint)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__last)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__pacman)
            opts="one two"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__pacman__subcmd__one)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__pacman__subcmd__two)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__quote)
            opts="cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__quote__subcmd__cmd__subcmd__backslash)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__quote__subcmd__cmd__subcmd__backticks)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__quote__subcmd__cmd__subcmd__brackets)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__quote__subcmd__cmd__subcmd__double__subcmd__quotes)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__quote__subcmd__cmd__subcmd__expansions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__quote__subcmd__cmd__subcmd__single__subcmd__quotes)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__quote__subcmd__escape__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__help__subcmd__value)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__hint)
            opts="-p -f -d -e -c -u -H -h --choice --unknown --other --path --file --dir --exe --cmd-name --cmd --user --host --url --email --help [command_with_args]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --choice)
                    COMPREPLY=($(compgen -W "bash fish zsh" -- "${cur}"))
                    return 0
                    ;;
                --unknown)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --other)
                    COMPREPLY=("${cur}")
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o nospace
                    fi
                    return 0
                    ;;
                --path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --file)
                    local oldifs
                    if [ -n "${IFS+x}" ]; then
                        oldifs="$IFS"
                    fi
                    IFS=$'\n'
                    COMPREPLY=($(compgen -f "${cur}"))
                    if [ -n "${oldifs+x}" ]; then
                        IFS="$oldifs"
                    fi
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o filenames
                    fi
                    return 0
                    ;;
                -f)
                    local oldifs
                    if [ -n "${IFS+x}" ]; then
                        oldifs="$IFS"
                    fi
                    IFS=$'\n'
                    COMPREPLY=($(compgen -f "${cur}"))
                    if [ -n "${oldifs+x}" ]; then
                        IFS="$oldifs"
                    fi
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o filenames
                    fi
                    return 0
                    ;;
                --dir)
                    COMPREPLY=()
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o plusdirs
                    fi
                    return 0
                    ;;
                -d)
                    COMPREPLY=()
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o plusdirs
                    fi
                    return 0
                    ;;
                --exe)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -e)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cmd-name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cmd)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --user)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -u)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -H)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --url)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --email)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__last)
            opts="-h --help [first] [free]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__pacman)
            opts="-h --help one two help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__pacman__subcmd__help)
            opts="one two help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__pacman__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__pacman__subcmd__help__subcmd__one)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__pacman__subcmd__help__subcmd__two)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__pacman__subcmd__one)
            opts="-h --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__pacman__subcmd__two)
            opts="-h --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__quote)
            opts="-h --single-quotes --double-quotes --backticks --backslash --brackets --expansions --choice --help cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --choice)
                    COMPREPLY=($(compgen -W "another shell bash fish zsh" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__quote__subcmd__cmd__subcmd__backslash)
            opts="-h --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__quote__subcmd__cmd__subcmd__backticks)
            opts="-h --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__quote__subcmd__cmd__subcmd__brackets)
            opts="-h --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__quote__subcmd__cmd__subcmd__double__subcmd__quotes)
            opts="-h --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__quote__subcmd__cmd__subcmd__expansions)
            opts="-h --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__quote__subcmd__cmd__subcmd__single__subcmd__quotes)
            opts="-h --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__quote__subcmd__escape__subcmd__help)
            opts="-h --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__quote__subcmd__help)
            opts="cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions escape-help help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__quote__subcmd__help__subcmd__cmd__subcmd__backslash)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__quote__subcmd__help__subcmd__cmd__subcmd__backticks)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__quote__subcmd__help__subcmd__cmd__subcmd__brackets)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__quote__subcmd__help__subcmd__cmd__subcmd__double__subcmd__quotes)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__quote__subcmd__help__subcmd__cmd__subcmd__expansions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__quote__subcmd__help__subcmd__cmd__subcmd__single__subcmd__quotes)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__quote__subcmd__help__subcmd__escape__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__quote__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        exhaustive__subcmd__value)
            opts="-h --delim --tuple --require-eq --help [term]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --delim)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --tuple)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --require-eq)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _exhaustive -o nosort -o bashdefault -o default exhaustive
else
    complete -F _exhaustive -o bashdefault -o default exhaustive
fi

