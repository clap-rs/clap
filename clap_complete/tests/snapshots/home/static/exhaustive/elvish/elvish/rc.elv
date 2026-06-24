set edit:rprompt = (constantly "")
set edit:prompt = (constantly "% ")

use builtin;
use str;

set edit:completion:arg-completer[exhaustive] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'exhaustive'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'exhaustive'= {
            cand --generate 'generate'
            cand --empty-choice 'empty-choice'
            cand -h 'Print help'
            cand --help 'Print help'
            cand empty 'empty'
            cand global 'global'
            cand action 'action'
            cand quote 'quote'
            cand value 'value'
            cand pacman 'pacman'
            cand last 'last'
            cand alias 'alias'
            cand hint 'hint'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'exhaustive;empty'= {
        }
        &'exhaustive;global'= {
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand one 'one'
            cand two 'two'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'exhaustive;global;one'= {
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand one-one 'one-one'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'exhaustive;global;one;one-one'= {
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'exhaustive;global;one;help'= {
            cand one-one 'one-one'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'exhaustive;global;one;help;one-one'= {
        }
        &'exhaustive;global;one;help;help'= {
        }
        &'exhaustive;global;two'= {
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'exhaustive;global;help'= {
            cand one 'one'
            cand two 'two'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'exhaustive;global;help;one'= {
            cand one-one 'one-one'
        }
        &'exhaustive;global;help;one;one-one'= {
        }
        &'exhaustive;global;help;two'= {
        }
        &'exhaustive;global;help;help'= {
        }
        &'exhaustive;action'= {
            cand --set 'value'
            cand --choice 'enum'
            cand --set-true 'bool'
            cand --count 'number'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'exhaustive;quote'= {
            cand --choice 'choice'
            cand --single-quotes 'Can be ''always'', ''auto'', or ''never'''
            cand --double-quotes 'Can be "always", "auto", or "never"'
            cand --backticks 'For more information see `echo test`'
            cand --backslash 'Avoid ''\n'''
            cand --brackets 'List packages [filter]'
            cand --expansions 'Execute the shell command with $SHELL'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand cmd-single-quotes 'Can be ''always'', ''auto'', or ''never'''
            cand cmd-double-quotes 'Can be "always", "auto", or "never"'
            cand cmd-backticks 'For more information see `echo test`'
            cand cmd-backslash 'Avoid ''\n'''
            cand cmd-brackets 'List packages [filter]'
            cand cmd-expansions 'Execute the shell command with $SHELL'
            cand escape-help '\tab	"'' New Line'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'exhaustive;quote;cmd-single-quotes'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'exhaustive;quote;cmd-double-quotes'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'exhaustive;quote;cmd-backticks'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'exhaustive;quote;cmd-backslash'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'exhaustive;quote;cmd-brackets'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'exhaustive;quote;cmd-expansions'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'exhaustive;quote;escape-help'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'exhaustive;quote;help'= {
            cand cmd-single-quotes 'Can be ''always'', ''auto'', or ''never'''
            cand cmd-double-quotes 'Can be "always", "auto", or "never"'
            cand cmd-backticks 'For more information see `echo test`'
            cand cmd-backslash 'Avoid ''\n'''
            cand cmd-brackets 'List packages [filter]'
            cand cmd-expansions 'Execute the shell command with $SHELL'
            cand escape-help '\tab	"'' New Line'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'exhaustive;quote;help;cmd-single-quotes'= {
        }
        &'exhaustive;quote;help;cmd-double-quotes'= {
        }
        &'exhaustive;quote;help;cmd-backticks'= {
        }
        &'exhaustive;quote;help;cmd-backslash'= {
        }
        &'exhaustive;quote;help;cmd-brackets'= {
        }
        &'exhaustive;quote;help;cmd-expansions'= {
        }
        &'exhaustive;quote;help;escape-help'= {
        }
        &'exhaustive;quote;help;help'= {
        }
        &'exhaustive;value'= {
            cand --delim 'delim'
            cand --tuple 'tuple'
            cand --require-eq 'require-eq'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'exhaustive;pacman'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand one 'one'
            cand two 'two'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'exhaustive;pacman;one'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'exhaustive;pacman;two'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'exhaustive;pacman;help'= {
            cand one 'one'
            cand two 'two'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'exhaustive;pacman;help;one'= {
        }
        &'exhaustive;pacman;help;two'= {
        }
        &'exhaustive;pacman;help;help'= {
        }
        &'exhaustive;last'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'exhaustive;alias'= {
            cand -o 'cmd option'
            cand -O 'cmd option'
            cand --option 'cmd option'
            cand --opt 'cmd option'
            cand -f 'cmd flag'
            cand -F 'cmd flag'
            cand --flag 'cmd flag'
            cand --flg 'cmd flag'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'exhaustive;hint'= {
            cand --choice 'choice'
            cand --unknown 'unknown'
            cand --other 'other'
            cand -p 'p'
            cand --path 'path'
            cand -f 'f'
            cand --file 'file'
            cand -d 'd'
            cand --dir 'dir'
            cand -e 'e'
            cand --exe 'exe'
            cand --cmd-name 'cmd-name'
            cand -c 'c'
            cand --cmd 'cmd'
            cand -u 'u'
            cand --user 'user'
            cand -H 'H'
            cand --host 'host'
            cand --url 'url'
            cand --email 'email'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'exhaustive;help'= {
            cand empty 'empty'
            cand global 'global'
            cand action 'action'
            cand quote 'quote'
            cand value 'value'
            cand pacman 'pacman'
            cand last 'last'
            cand alias 'alias'
            cand hint 'hint'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'exhaustive;help;empty'= {
        }
        &'exhaustive;help;global'= {
            cand one 'one'
            cand two 'two'
        }
        &'exhaustive;help;global;one'= {
            cand one-one 'one-one'
        }
        &'exhaustive;help;global;one;one-one'= {
        }
        &'exhaustive;help;global;two'= {
        }
        &'exhaustive;help;action'= {
        }
        &'exhaustive;help;quote'= {
            cand cmd-single-quotes 'Can be ''always'', ''auto'', or ''never'''
            cand cmd-double-quotes 'Can be "always", "auto", or "never"'
            cand cmd-backticks 'For more information see `echo test`'
            cand cmd-backslash 'Avoid ''\n'''
            cand cmd-brackets 'List packages [filter]'
            cand cmd-expansions 'Execute the shell command with $SHELL'
            cand escape-help '\tab	"'' New Line'
        }
        &'exhaustive;help;quote;cmd-single-quotes'= {
        }
        &'exhaustive;help;quote;cmd-double-quotes'= {
        }
        &'exhaustive;help;quote;cmd-backticks'= {
        }
        &'exhaustive;help;quote;cmd-backslash'= {
        }
        &'exhaustive;help;quote;cmd-brackets'= {
        }
        &'exhaustive;help;quote;cmd-expansions'= {
        }
        &'exhaustive;help;quote;escape-help'= {
        }
        &'exhaustive;help;value'= {
        }
        &'exhaustive;help;pacman'= {
            cand one 'one'
            cand two 'two'
        }
        &'exhaustive;help;pacman;one'= {
        }
        &'exhaustive;help;pacman;two'= {
        }
        &'exhaustive;help;last'= {
        }
        &'exhaustive;help;alias'= {
        }
        &'exhaustive;help;hint'= {
        }
        &'exhaustive;help;help'= {
        }
    ]
    $completions[$command]
}

