set edit:rprompt = (constantly "")
set edit:prompt = (constantly "% ")

set edit:completion:arg-completer[exhaustive] = { |@words|
    var index = (count $words)
    set index = (- $index 1)

    tmp E:_CLAP_IFS = "\n"
    tmp E:_CLAP_COMPLETE_INDEX = (to-string $index)
    tmp E:COMPLETE = "elvish"
    put (exhaustive -- $@words) | to-lines
}


