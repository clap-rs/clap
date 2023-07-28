set edit:rprompt = (constantly "")
set edit:prompt = (constantly "% ")

use builtin;
use str;

set edit:completion:arg-completer[test] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'test'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'test'= {
            cand --generate 'generate'
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand action 'action'
            cand quote 'quote'
            cand value 'value'
            cand pacman 'pacman'
            cand last 'last'
            cand alias 'alias'
            cand hint 'hint'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'test;action'= {
            cand --set 'value'
            cand --choice 'enum'
            cand --set-true 'bool'
            cand --count 'number'
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'test;quote'= {
            cand --single-quotes 'Can be ''always'', ''auto'', or ''never'''
            cand --double-quotes 'Can be "always", "auto", or "never"'
            cand --backticks 'For more information see `echo test`'
            cand --backslash 'Avoid ''\n'''
            cand --brackets 'List packages [filter]'
            cand --expansions 'Execute the shell command with $SHELL'
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand cmd-single-quotes 'Can be ''always'', ''auto'', or ''never'''
            cand cmd-double-quotes 'Can be "always", "auto", or "never"'
            cand cmd-backticks 'For more information see `echo test`'
            cand cmd-backslash 'Avoid ''\n'''
            cand cmd-brackets 'List packages [filter]'
            cand cmd-expansions 'Execute the shell command with $SHELL'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'test;quote;cmd-single-quotes'= {
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'test;quote;cmd-double-quotes'= {
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'test;quote;cmd-backticks'= {
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'test;quote;cmd-backslash'= {
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'test;quote;cmd-brackets'= {
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'test;quote;cmd-expansions'= {
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'test;quote;help'= {
            cand cmd-single-quotes 'Can be ''always'', ''auto'', or ''never'''
            cand cmd-double-quotes 'Can be "always", "auto", or "never"'
            cand cmd-backticks 'For more information see `echo test`'
            cand cmd-backslash 'Avoid ''\n'''
            cand cmd-brackets 'List packages [filter]'
            cand cmd-expansions 'Execute the shell command with $SHELL'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'test;quote;help;cmd-single-quotes'= {
        }
        &'test;quote;help;cmd-double-quotes'= {
        }
        &'test;quote;help;cmd-backticks'= {
        }
        &'test;quote;help;cmd-backslash'= {
        }
        &'test;quote;help;cmd-brackets'= {
        }
        &'test;quote;help;cmd-expansions'= {
        }
        &'test;quote;help;help'= {
        }
        &'test;value'= {
            cand --delim 'delim'
            cand --tuple 'tuple'
            cand --require-eq 'require-eq'
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'test;pacman'= {
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand one 'one'
            cand two 'two'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'test;pacman;one'= {
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'test;pacman;two'= {
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'test;pacman;help'= {
            cand one 'one'
            cand two 'two'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'test;pacman;help;one'= {
        }
        &'test;pacman;help;two'= {
        }
        &'test;pacman;help;help'= {
        }
        &'test;last'= {
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'test;alias'= {
            cand -o 'cmd option'
            cand -O 'cmd option'
            cand --option 'cmd option'
            cand --opt 'cmd option'
            cand -f 'cmd flag'
            cand -F 'cmd flag'
            cand --flag 'cmd flag'
            cand --flg 'cmd flag'
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'test;hint'= {
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
            cand --global 'everywhere'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'test;help'= {
            cand action 'action'
            cand quote 'quote'
            cand value 'value'
            cand pacman 'pacman'
            cand last 'last'
            cand alias 'alias'
            cand hint 'hint'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'test;help;action'= {
        }
        &'test;help;quote'= {
            cand cmd-single-quotes 'Can be ''always'', ''auto'', or ''never'''
            cand cmd-double-quotes 'Can be "always", "auto", or "never"'
            cand cmd-backticks 'For more information see `echo test`'
            cand cmd-backslash 'Avoid ''\n'''
            cand cmd-brackets 'List packages [filter]'
            cand cmd-expansions 'Execute the shell command with $SHELL'
        }
        &'test;help;quote;cmd-single-quotes'= {
        }
        &'test;help;quote;cmd-double-quotes'= {
        }
        &'test;help;quote;cmd-backticks'= {
        }
        &'test;help;quote;cmd-backslash'= {
        }
        &'test;help;quote;cmd-brackets'= {
        }
        &'test;help;quote;cmd-expansions'= {
        }
        &'test;help;value'= {
        }
        &'test;help;pacman'= {
            cand one 'one'
            cand two 'two'
        }
        &'test;help;pacman;one'= {
        }
        &'test;help;pacman;two'= {
        }
        &'test;help;last'= {
        }
        &'test;help;alias'= {
        }
        &'test;help;hint'= {
        }
        &'test;help;help'= {
        }
    ]
    $completions[$command]
}

