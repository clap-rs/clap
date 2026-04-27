set edit:rprompt = (constantly "")
set edit:prompt = (constantly "% ")

set edit:completion:arg-completer[exhaustive] = { |@words|
    var index = (count $words)
    set index = (- $index 1)

    env _CLAP_IFS="\n" _CLAP_COMPLETE_INDEX=(to-string $index) COMPLETE="elvish" exhaustive -- $@words | from-lines | each { |line|
        if (str:has-prefix $line "\x1f") {
            var value = (str:trim-prefix $line "\x1f")
            edit:complex-candidate $value &code-suffix=''
        } else {
            put $line
        }
    }
}

