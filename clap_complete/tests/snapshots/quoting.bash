_my-app() {
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
                cmd="my__app"
                ;;
            my__app,cmd-backslash)
                cmd="my__app__subcmd__cmd__subcmd__backslash"
                ;;
            my__app,cmd-backticks)
                cmd="my__app__subcmd__cmd__subcmd__backticks"
                ;;
            my__app,cmd-brackets)
                cmd="my__app__subcmd__cmd__subcmd__brackets"
                ;;
            my__app,cmd-double-quotes)
                cmd="my__app__subcmd__cmd__subcmd__double__subcmd__quotes"
                ;;
            my__app,cmd-expansions)
                cmd="my__app__subcmd__cmd__subcmd__expansions"
                ;;
            my__app,cmd-single-quotes)
                cmd="my__app__subcmd__cmd__subcmd__single__subcmd__quotes"
                ;;
            my__app,help)
                cmd="my__app__subcmd__help"
                ;;
            my__app__subcmd__help,cmd-backslash)
                cmd="my__app__subcmd__help__subcmd__cmd__subcmd__backslash"
                ;;
            my__app__subcmd__help,cmd-backticks)
                cmd="my__app__subcmd__help__subcmd__cmd__subcmd__backticks"
                ;;
            my__app__subcmd__help,cmd-brackets)
                cmd="my__app__subcmd__help__subcmd__cmd__subcmd__brackets"
                ;;
            my__app__subcmd__help,cmd-double-quotes)
                cmd="my__app__subcmd__help__subcmd__cmd__subcmd__double__subcmd__quotes"
                ;;
            my__app__subcmd__help,cmd-expansions)
                cmd="my__app__subcmd__help__subcmd__cmd__subcmd__expansions"
                ;;
            my__app__subcmd__help,cmd-single-quotes)
                cmd="my__app__subcmd__help__subcmd__cmd__subcmd__single__subcmd__quotes"
                ;;
            my__app__subcmd__help,help)
                cmd="my__app__subcmd__help__subcmd__help"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        my__app)
            opts="-h -V --single-quotes --double-quotes --backticks --backslash --brackets --expansions --help --version cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
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
        my__subcmd__app__subcmd__cmd__subcmd__backslash)
            opts="-h --help"
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
        my__subcmd__app__subcmd__cmd__subcmd__backticks)
            opts="-h --help"
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
        my__subcmd__app__subcmd__cmd__subcmd__brackets)
            opts="-h --help"
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
        my__subcmd__app__subcmd__cmd__subcmd__double__subcmd__quotes)
            opts="-h --help"
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
        my__subcmd__app__subcmd__cmd__subcmd__expansions)
            opts="-h --help"
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
        my__subcmd__app__subcmd__cmd__subcmd__single__subcmd__quotes)
            opts="-h --help"
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
        my__subcmd__app__subcmd__help)
            opts="cmd-single-quotes cmd-double-quotes cmd-backticks cmd-backslash cmd-brackets cmd-expansions help"
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
        my__subcmd__app__subcmd__help__subcmd__cmd__subcmd__backslash)
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
        my__subcmd__app__subcmd__help__subcmd__cmd__subcmd__backticks)
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
        my__subcmd__app__subcmd__help__subcmd__cmd__subcmd__brackets)
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
        my__subcmd__app__subcmd__help__subcmd__cmd__subcmd__double__subcmd__quotes)
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
        my__subcmd__app__subcmd__help__subcmd__cmd__subcmd__expansions)
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
        my__subcmd__app__subcmd__help__subcmd__cmd__subcmd__single__subcmd__quotes)
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
        my__subcmd__app__subcmd__help__subcmd__help)
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
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _my-app -o nosort -o bashdefault -o default my-app
else
    complete -F _my-app -o bashdefault -o default my-app
fi
