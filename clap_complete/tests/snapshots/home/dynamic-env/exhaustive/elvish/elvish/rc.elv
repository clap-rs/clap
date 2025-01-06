set edit:rprompt = (constantly "")
set edit:prompt = (constantly "% ")

set edit:completion:arg-completer[exhaustive] = { |@words|
    var index = (count $words)
    set index = (- $index 1)

    put (env _CLAP_IFS="\n" _CLAP_COMPLETE_INDEX=(to-string $index) COMPLETE="elvish" exhaustive -- $@words) | to-lines
}


