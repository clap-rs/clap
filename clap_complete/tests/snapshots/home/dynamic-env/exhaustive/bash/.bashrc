PS1='% '
. /etc/bash_completion

_clap_complete_exhaustive() {
    local IFS=$'\013'
    local _CLAP_COMPLETE_INDEX=${COMP_CWORD}
    local _CLAP_COMPLETE_COMP_TYPE=${COMP_TYPE}
    if compopt +o nospace 2> /dev/null; then
        local _CLAP_COMPLETE_SPACE=false
    else
        local _CLAP_COMPLETE_SPACE=true
    fi
    local words=("${COMP_WORDS[@]}")
    local _clap_head_len=0
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        # Build the word list by splitting COMP_LINE[0..COMP_POINT] on
        # whitespace using bash's own parser (via eval), so that
        # COMP_WORDBREAKS characters like '=' and ':' are kept inside their
        # tokens rather than split out.  This gives the clap engine an accurate
        # view of the whole command line, including preceding arguments like
        # --level=debug that are also affected by COMP_WORDBREAKS splitting.
        local _clap_line="${COMP_LINE:0:$COMP_POINT}"
        local _clap_words
        local _cword
        if eval "_clap_words=( $_clap_line )" 2>/dev/null; then
            # If the line ends with whitespace the current word is empty and
            # eval drops it; push an explicit empty entry so _cword is valid.
            if [[ "${_clap_line}" =~ [[:space:]]$ ]]; then
                _clap_words+=("")
            fi
            _cword=$(( ${#_clap_words[@]} - 1 ))
            _CLAP_COMPLETE_INDEX=$_cword
            words=("${_clap_words[@]}")
            # Compute _clap_head_len: the number of leading characters of the
            # current word that bash has already consumed into preceding
            # COMP_WORDS tokens (everything up to and including the last
            # COMP_WORDBREAKS character).  The tail is the longest suffix with
            # no break character; head_len = len(current_word) - len(tail).
            # Bash splices each COMPREPLY entry in place of $2 (the tail), so
            # we strip the first _clap_head_len characters from each candidate.
            local _clap_tail="${words[_cword]##*["${COMP_WORDBREAKS}"]}"
            _clap_head_len=$(( ${#words[_cword]} - ${#_clap_tail} ))
        else
            # eval failed (e.g. unclosed quote): fall back to COMP_WORDS.
            # In this case $2 is already the correct word-to-complete and
            # no COMP_WORDBREAKS stripping is needed (_clap_head_len stays 0).
            _cword=$COMP_CWORD
            _CLAP_COMPLETE_INDEX=$_cword
            words[$_cword]="$2"
        fi
    fi
    local _clap_completions
    _clap_completions=( $( \
        _CLAP_IFS="$IFS" \
        _CLAP_COMPLETE_INDEX="$_CLAP_COMPLETE_INDEX" \
        _CLAP_COMPLETE_COMP_TYPE="$_CLAP_COMPLETE_COMP_TYPE" \
        _CLAP_COMPLETE_SPACE="$_CLAP_COMPLETE_SPACE" \
        COMPLETE="bash" \
        "exhaustive" -- "${words[@]}" \
    ) )
    if [[ $? != 0 ]]; then
        unset COMPREPLY
    else
        local _clap_c
        COMPREPLY=()
        for _clap_c in "${_clap_completions[@]}"; do
            COMPREPLY+=("${_clap_c:_clap_head_len}")
        done
        if [[ $_CLAP_COMPLETE_SPACE == false ]] && [[ "${COMPREPLY-}" =~ [=/:]$ ]]; then
            compopt -o nospace
        fi
    fi
}
if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -o nospace -o bashdefault -o nosort -F _clap_complete_exhaustive exhaustive
else
    complete -o nospace -o bashdefault -F _clap_complete_exhaustive exhaustive
fi


