
_clap_complete_my_app() {
    local IFS=$'\013'
    local _CLAP_COMPLETE_INDEX=${COMP_CWORD}
    local _CLAP_COMPLETE_COMP_TYPE=${COMP_TYPE}
    if compopt +o nospace 2> /dev/null; then
        local _CLAP_COMPLETE_SPACE=false
    else
        local _CLAP_COMPLETE_SPACE=true
    fi
    COMPREPLY=( $( \
        IFS="$IFS" \
        _CLAP_COMPLETE_INDEX="$_CLAP_COMPLETE_INDEX" \
        _CLAP_COMPLETE_COMP_TYPE="$_CLAP_COMPLETE_COMP_TYPE" \
        _CLAP_COMPLETE_SPACE="$_CLAP_COMPLETE_SPACE" \
        "my-app" complete bash -- "${COMP_WORDS[@]}" \
    ) )
    if [[ $? != 0 ]]; then
        unset COMPREPLY
    elif [[ $_CLAP_COMPLETE_SPACE == false ]]; then
        if [[ ${#COMPREPLY[@]} -gt 0 ]] && [[ "${COMPREPLY[-1]}" == "_CLAP_COMPLETE_NOSPACE" ]]; then
            unset 'COMPREPLY[-1]'
            compopt -o nospace
        fi
    fi
}
if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -o nospace -o bashdefault -o nosort -F _clap_complete_my_app my-app
else
    complete -o nospace -o bashdefault -F _clap_complete_my_app my-app
fi

