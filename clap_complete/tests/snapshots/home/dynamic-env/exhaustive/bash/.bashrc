PS1='% '
. /etc/bash_completion

_clap_complete_exhaustive() {
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
        # Reassemble the word list by walking COMP_WORDS and COMP_LINE in
        # lockstep.  COMP_WORDS is split on both whitespace (IFS) and
        # COMP_WORDBREAKS characters; we want to undo only the COMP_WORDBREAKS
        # splits so that tokens like --opt=val are presented to the clap engine
        # as a single word.  We distinguish the two kinds of split by inspecting
        # the gap text between consecutive COMP_WORDS entries in COMP_LINE:
        #
        #   - If the first character of the gap is whitespace, the split was
        #     caused by a word boundary and must be kept.
        #   - Otherwise (gap is empty or starts with a non-whitespace character)
        #     the split was caused by COMP_WORDBREAKS alone and the two tokens
        #     are glued back together.
        #
        # COMP_LINE is used only as a read-only string; it is never eval'd.
        local _clap_line="${COMP_LINE:0:$COMP_POINT}"
        # Build the token list to reassemble.  COMP_WORDS is indexed up to
        # COMP_CWORD inclusive, but bash populates it from the full COMP_LINE,
        # so COMP_WORDS[COMP_CWORD] may be a word that lies after the cursor.
        # $2 tells us the actual word at the cursor (quote-stripped, truncated
        # at COMP_POINT); when it is non-empty, patch it into the slice.
        # When $2 is empty and the line ends with whitespace the cursor sits in
        # trailing whitespace: COMP_WORDS[COMP_CWORD] is the next word, not the
        # current one, so drop it; the trailing-whitespace check after the loop
        # will push the empty current word.
        local _clap_comp_words=("${COMP_WORDS[@]:0:COMP_CWORD+1}")
        if [[ -n "$2" ]]; then
            _clap_comp_words[COMP_CWORD]="$2"
        elif [[ "${_clap_line}" =~ [[:space:]]$ ]]; then
            _clap_comp_words=("${COMP_WORDS[@]:0:COMP_CWORD}")
        fi
        local _clap_words=()
        local _cword
        local _clap_pos=0
        local _clap_w _clap_gap _clap_rest _clap_gap_ch
        for _clap_w in "${_clap_comp_words[@]}"; do
            # The gap is the text in _clap_line between the previous token and
            # this one.  "${var%%"tok"*}" removes the longest suffix of var
            # that matches "tok"*, leaving the prefix before the first "tok".
            _clap_rest="${_clap_line:_clap_pos}"
            _clap_gap="${_clap_rest%%"${_clap_w}"*}"
            _clap_gap_ch="${_clap_gap:0:1}"
            if [[ ${#_clap_words[@]} -eq 0 || "${_clap_gap_ch}" == [[:space:]] ]]; then
                # No previous word yet, or the gap starts with whitespace:
                # this is a genuine word boundary — start a new word.
                _clap_words+=("${_clap_w}")
            else
                # Gap is empty or starts with a non-whitespace character:
                # a COMP_WORDBREAKS split — glue onto the previous word.
                _clap_words[${#_clap_words[@]}-1]+="${_clap_w}"
            fi
            _clap_pos=$(( _clap_pos + ${#_clap_gap} + ${#_clap_w} ))
        done
        # If the line up to the cursor ends with whitespace the current word
        # is empty.  COMP_WORDS omits it; push an explicit empty entry.
        if [[ "${_clap_line}" =~ [[:space:]]$ ]]; then
            _clap_words+=("")
        fi
        _cword=$(( ${#_clap_words[@]} - 1 ))
        _CLAP_COMPLETE_INDEX=$_cword
        words=("${_clap_words[@]}")
        # Bash splices each COMPREPLY entry in place of $2, which is only
        # the suffix of the current word after its last COMP_WORDBREAKS
        # character (the "tail").  Strip the head from every candidate so
        # that bash inserts the full token into the command line.
        local _clap_tail="${words[_cword]##*["${COMP_WORDBREAKS}"]}"
        _clap_head_len=$(( ${#words[_cword]} - ${#_clap_tail} ))
    fi
    # Capture output and exit code separately: `local var=$(cmd)` always
    # returns 0 because `local` itself succeeds, masking cmd's exit code.
    local _clap_out
    _clap_out=$( \
        _CLAP_COMPLETE_INDEX="$_CLAP_COMPLETE_INDEX" \
        _CLAP_COMPLETE_COMP_TYPE="$_CLAP_COMPLETE_COMP_TYPE" \
        _CLAP_COMPLETE_SPACE="$_CLAP_COMPLETE_SPACE" \
        COMPLETE="bash" \
        "exhaustive" -- "${words[@]}" \
    )
    if [[ $? != 0 ]]; then
        unset COMPREPLY
    else
        local _clap_completions=() _clap_c
        if [[ -n "$_clap_out" ]]; then
            mapfile -t _clap_completions <<< "$_clap_out"
        fi
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


