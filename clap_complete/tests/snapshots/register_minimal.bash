
_clap_complete_my_app() {
    local IFS=$'/013'
    local SUPPRESS_SPACE=0
    if compopt +o nospace 2> /dev/null; then
        SUPPRESS_SPACE=1
    fi
    if [[ ${SUPPRESS_SPACE} == 1 ]]; then
        SPACE_ARG="--no-space"
    else
        SPACE_ARG="--space"
    fi
    COMPREPLY=( $("my-app" complete --index ${COMP_CWORD} --type ${COMP_TYPE} ${SPACE_ARG} --ifs="$IFS" -- "${COMP_WORDS[@]}") )
    if [[ $? != 0 ]]; then
        unset COMPREPLY
    elif [[ $SUPPRESS_SPACE == 1 ]] && [[ "${COMPREPLY-}" =~ [=/:]$ ]]; then
        compopt -o nospace
    fi
}
complete -o nospace -o bashdefault -F _clap_complete_my_app my-app

