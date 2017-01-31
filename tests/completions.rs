extern crate regex;
extern crate clap;

use clap::{App, Arg, SubCommand, Shell};
use regex::Regex;

static BASH: &'static str = r#"_myapp() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            myapp)
                cmd="myapp"
                ;;
            
            help)
                cmd+="_help"
                ;;
            test)
                cmd+="_test"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        myapp)
            opts=" -h -V  --help --version  <file>  test help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        
        myapp_help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        myapp_test)
            opts=" -h -V  --case --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --case)
                    COMPREPLY=("<case>")
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
    esac
}

complete -F _myapp -o bashdefault -o default myapp
"#;

static ZSH: &'static str = r#"#compdef myapp

_myapp() {
    typeset -A opt_args
    local ret=1

    local context curcontext="$curcontext" state line
    _arguments -s -S -C \
"-h[Prints help information]" \
"--help[Prints help information]" \
"-V[Prints version information]" \
"--version[Prints version information]" \
"1:: :_myapp_commands" \
"*:: :->myapp" \
&& ret=0
    case $state in
    (myapp)
        curcontext="${curcontext%:*:*}:myapp-command-$words[1]:"
        case $line[1] in
            (test)
_arguments -s -S -C \
"--case+[the case to test]" \
"-h[Prints help information]" \
"--help[Prints help information]" \
"-V[Prints version information]" \
"--version[Prints version information]" \
&& ret=0
;;
(help)
_arguments -s -S -C \
"-h[Prints help information]" \
"--help[Prints help information]" \
"-V[Prints version information]" \
"--version[Prints version information]" \
&& ret=0
;;
        esac
    ;;
esac
}

(( $+functions[_myapp_commands] )) ||
_myapp_commands() {
    local commands; commands=(
        "test:tests things" \
"help:Prints this message or the help of the given subcommand(s)" \
"FILE:some input file" \
    )
    _describe -t commands 'myapp commands' commands "$@"
}
(( $+functions[_myapp_help_commands] )) ||
_myapp_help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'myapp help commands' commands "$@"
}
(( $+functions[_myapp_test_commands] )) ||
_myapp_test_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'myapp test commands' commands "$@"
}

_myapp "$@""#;

static FISH: &'static str = r#"function __fish_using_command
    set cmd (commandline -opc)
    if [ (count $cmd) -eq (count $argv) ]
        for i in (seq (count $argv))
            if [ $cmd[$i] != $argv[$i] ]
                return 1
            end
        end
        return 0
    end
    return 1
end

complete -c myapp -n "__fish_using_command myapp" -s h -l help -d "Prints help information"
complete -c myapp -n "__fish_using_command myapp" -s V -l version -d "Prints version information"
complete -c myapp -n "__fish_using_command myapp" -f -a "test"
complete -c myapp -n "__fish_using_command myapp" -f -a "help"
complete -c myapp -n "__fish_using_command myapp test" -l case -d "the case to test"
complete -c myapp -n "__fish_using_command myapp test" -s h -l help -d "Prints help information"
complete -c myapp -n "__fish_using_command myapp test" -s V -l version -d "Prints version information"
complete -c myapp -n "__fish_using_command myapp help" -s h -l help -d "Prints help information"
complete -c myapp -n "__fish_using_command myapp help" -s V -l version -d "Prints version information"
"#;

static POWERSHELL: &'static str = r#"
@('myapp', './myapp') | %{
    Register-ArgumentCompleter -Native -CommandName $_ -ScriptBlock {
        param($wordToComplete, $commandAst, $cursorPosition)

        $command = '_myapp'
        $commandAst.CommandElements |
            Select-Object -Skip 1 |
            %{
                switch ($_.ToString()) {

                    'test' {
                        $command += '_test'
                        break
                    }

                    'help' {
                        $command += '_help'
                        break
                    }

                }
            }

        $completions = @()

        switch ($command) {

            '_myapp' {
                $completions = @('test', 'help', '-h', '-V', '--help', '--version')
            }

            '_myapp_test' {
                $completions = @('-h', '-V', '--case', '--help', '--version')
            }

            '_myapp_help' {
                $completions = @('-h', '-V', '--help', '--version')
            }

        }

        $completions |
            ?{ $_ -like "$wordToComplete*" } |
            Sort-Object |
            %{ New-Object System.Management.Automation.CompletionResult $_, $_, 'ParameterValue', $_ }
    }
}
"#;

fn compare(left: &str, right: &str) -> bool {
    let b = left == right;
    if !b {
        let re = Regex::new(" ").unwrap();
        println!("");
        println!("--> left");
        // println!("{}", left);
        println!("{}", re.replace_all(left, "\u{2022}"));
        println!("--> right");
        println!("{}", re.replace_all(right, "\u{2022}"));
        // println!("{}", right);
        println!("--")
    }
    b
}

fn build_app() -> App<'static, 'static> {
   App::new("myapp")
        .about("Tests completions")
        .arg(Arg::with_name("file")
            .help("some input file"))
        .subcommand(SubCommand::with_name("test")
            .about("tests things")
            .arg(Arg::with_name("case")
                .long("case")
                .takes_value(true)
                .help("the case to test")))
}

#[test]
fn bash() {
    let mut app = build_app();
    let mut buf = vec![];
    app.gen_completions_to("myapp", Shell::Bash, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, BASH));
}

#[test]
fn zsh() {
    let mut app = build_app();
    let mut buf = vec![];
    app.gen_completions_to("myapp", Shell::Zsh, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, ZSH));
}

#[test]
fn fish() {
    let mut app = build_app();
    let mut buf = vec![];
    app.gen_completions_to("myapp", Shell::Fish, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, FISH));
}

#[test]
fn powershell() {
    let mut app = build_app();
    let mut buf = vec![];
    app.gen_completions_to("myapp", Shell::PowerShell, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, POWERSHELL));
}
