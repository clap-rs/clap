set edit:rprompt = (constantly "")
set edit:prompt = (constantly "% ")

set edit:completion:arg-completer[exhaustive] = { |@words|
    set E:_CLAP_IFS = "\n"

    var index = (count $words)
    set index = (- $index 1)
    set E:_CLAP_COMPLETE_INDEX = (to-string $index)

    put (exhaustive complete --shell elvish -- $@words) | to-lines
}


