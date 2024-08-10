PS1='% '
. /etc/bash_completion

_clap_complete_exhaustive() {
    export IFS=$'\013'
    export _CLAP_COMPLETE_INDEX=${COMP_CWORD}
    export _CLAP_COMPLETE_COMP_TYPE=${COMP_TYPE}
    if compopt +o nospace 2> /dev/null; then
        export _CLAP_COMPLETE_SPACE=false
    else
        export _CLAP_COMPLETE_SPACE=true
    fi
    COMPREPLY=( $("exhaustive" complete --shell bash -- "${COMP_WORDS[@]}") )
    if [[ $? != 0 ]]; then
        unset COMPREPLY
    elif [[ $SUPPRESS_SPACE == 1 ]] && [[ "${COMPREPLY-}" =~ [=/:]$ ]]; then
        compopt -o nospace
    fi
}
if [[ \"${{BASH_VERSINFO[0]}}\" -eq 4 && \"${{BASH_VERSINFO[1]}}\" -ge 4 || \"${{BASH_VERSINFO[0]}}\" -gt 4 ]]; then
    complete -o nospace -o bashdefault -o nosort -F _clap_complete_exhaustive exhaustive
else
    complete -o nospace -o bashdefault -F _clap_complete_exhaustive exhaustive
fi


