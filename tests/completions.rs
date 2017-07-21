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
                cmd+="__help"
                ;;
            test)
                cmd+="__test"
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
        
        myapp__help)
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
        myapp__test)
            opts=" -h -V  --help --version --case  "
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
(( $+functions[_myapp__help_commands] )) ||
_myapp__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'myapp help commands' commands "$@"
}
(( $+functions[_myapp__test_commands] )) ||
_myapp__test_commands() {
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
complete -c myapp -n "__fish_using_command myapp" -f -a "test" -d "tests things"
complete -c myapp -n "__fish_using_command myapp" -f -a "help" -d "Prints this message or the help of the given subcommand(s)"
complete -c myapp -n "__fish_using_command myapp test" -l case -d "the case to test"
complete -c myapp -n "__fish_using_command myapp test" -s h -l help -d "Prints help information"
complete -c myapp -n "__fish_using_command myapp test" -s V -l version -d "Prints version information"
complete -c myapp -n "__fish_using_command myapp help" -s h -l help -d "Prints help information"
complete -c myapp -n "__fish_using_command myapp help" -s V -l version -d "Prints version information"
"#;

#[cfg(not(target_os = "windows"))]
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

#[cfg(target_os = "windows")]
static POWERSHELL: &'static str = r#"
@('myapp', './myapp', 'myapp.exe', '.\myapp', '.\myapp.exe', './myapp.exe') | %{
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

#[cfg(not(target_os = "windows"))]
static POWERSHELL_WUS: &'static str = r#"
@('my_app', './my_app') | %{
    Register-ArgumentCompleter -Native -CommandName $_ -ScriptBlock {
        param($wordToComplete, $commandAst, $cursorPosition)

        $command = '_my_app'
        $commandAst.CommandElements |
            Select-Object -Skip 1 |
            %{
                switch ($_.ToString()) {

                    'test' {
                        $command += '_test'
                        break
                    }

                    'some_cmd' {
                        $command += '_some_cmd'
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

            '_my_app' {
                $completions = @('test', 'some_cmd', 'help', '-h', '-V', '--help', '--version')
            }

            '_my_app_test' {
                $completions = @('-h', '-V', '--case', '--help', '--version')
            }

            '_my_app_some_cmd' {
                $completions = @('-h', '-V', '--config', '--help', '--version')
            }

            '_my_app_help' {
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

#[cfg(target_os = "windows")]
static POWERSHELL_WUS: &'static str = r#"
@('my_app', './my_app', 'my_app.exe', '.\my_app', '.\my_app.exe', './my_app.exe') | %{
    Register-ArgumentCompleter -Native -CommandName $_ -ScriptBlock {
        param($wordToComplete, $commandAst, $cursorPosition)
        $command = '_my_app'
        $commandAst.CommandElements |
            Select-Object -Skip 1 |
            %{
                switch ($_.ToString()) {
                    'test' {
                        $command += '_test'
                        break
                    }
                    'some_cmd' {
                        $command += '_some_cmd'
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
            '_my_app' {
                $completions = @('test', 'some_cmd', 'help', '-h', '-V', '--help', '--version')
            }
            '_my_app_test' {
                $completions = @('-h', '-V', '--case', '--help', '--version')
            }
            '_my_app_some_cmd' {
                $completions = @('-h', '-V', '--config', '--help', '--version')
            }
            '_my_app_help' {
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

static ZSH_WUS: &'static str = r#"#compdef my_app

_my_app() {
    typeset -A opt_args
    local ret=1

    local context curcontext="$curcontext" state line
    _arguments -s -S -C \
"-h[Prints help information]" \
"--help[Prints help information]" \
"-V[Prints version information]" \
"--version[Prints version information]" \
"1:: :_my_app_commands" \
"*:: :->my_app" \
&& ret=0
    case $state in
    (my_app)
        curcontext="${curcontext%:*:*}:my_app-command-$words[1]:"
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
(some_cmd)
_arguments -s -S -C \
"--config+[the other case to test]" \
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

(( $+functions[_my_app_commands] )) ||
_my_app_commands() {
    local commands; commands=(
        "test:tests things" \
"some_cmd:tests other things" \
"help:Prints this message or the help of the given subcommand(s)" \
"FILE:some input file" \
    )
    _describe -t commands 'my_app commands' commands "$@"
}
(( $+functions[_my_app__help_commands] )) ||
_my_app__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'my_app help commands' commands "$@"
}
(( $+functions[_my_app__some_cmd_commands] )) ||
_my_app__some_cmd_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'my_app some_cmd commands' commands "$@"
}
(( $+functions[_my_app__test_commands] )) ||
_my_app__test_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'my_app test commands' commands "$@"
}

_my_app "$@""#;

static FISH_WUS: &'static str = r#"function __fish_using_command
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

complete -c my_app -n "__fish_using_command my_app" -s h -l help -d "Prints help information"
complete -c my_app -n "__fish_using_command my_app" -s V -l version -d "Prints version information"
complete -c my_app -n "__fish_using_command my_app" -f -a "test" -d "tests things"
complete -c my_app -n "__fish_using_command my_app" -f -a "some_cmd" -d "tests other things"
complete -c my_app -n "__fish_using_command my_app" -f -a "help" -d "Prints this message or the help of the given subcommand(s)"
complete -c my_app -n "__fish_using_command my_app test" -l case -d "the case to test"
complete -c my_app -n "__fish_using_command my_app test" -s h -l help -d "Prints help information"
complete -c my_app -n "__fish_using_command my_app test" -s V -l version -d "Prints version information"
complete -c my_app -n "__fish_using_command my_app some_cmd" -l config -d "the other case to test"
complete -c my_app -n "__fish_using_command my_app some_cmd" -s h -l help -d "Prints help information"
complete -c my_app -n "__fish_using_command my_app some_cmd" -s V -l version -d "Prints version information"
complete -c my_app -n "__fish_using_command my_app help" -s h -l help -d "Prints help information"
complete -c my_app -n "__fish_using_command my_app help" -s V -l version -d "Prints version information"
"#;

static BASH_WUS: &'static str = r#"_my_app() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            my_app)
                cmd="my_app"
                ;;
            
            help)
                cmd+="__help"
                ;;
            some_cmd)
                cmd+="__some_cmd"
                ;;
            test)
                cmd+="__test"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        my_app)
            opts=" -h -V  --help --version  <file>  test some_cmd help"
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
        
        my_app__help)
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
        my_app__some_cmd)
            opts=" -h -V  --help --version --config  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --config)
                    COMPREPLY=("<config>")
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        my_app__test)
            opts=" -h -V  --help --version --case  "
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

complete -F _my_app -o bashdefault -o default my_app
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

fn build_app() -> App<'static, 'static> { build_app_with_name("myapp") }

fn build_app_with_name(s: &'static str) -> App<'static, 'static> {
    App::new(s)
        .about("Tests completions")
        .arg(Arg::new("file").help("some input file"))
        .subcommand(App::new("test")
                        .about("tests things")
                        .arg(Arg::new("case")
                                 .long("case")
                                 .set(ArgSettings::TakesValue)
                                 .help("the case to test")))
}

fn build_app_with_underscore() -> App<'static, 'static> {
    build_app_with_name("my_app").subcommand(App::new("some_cmd")
                                                 .about("tests other things")
                                                 .arg(Arg::new("config")
                                                          .long("--config")
                                                          .set(ArgSettings::TakesValue)
                                                          .help("the other case to test")))
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

// Disabled until I figure out this windows line ending and AppVeyor issues
//#[test]
// fn powershell() {
//     let mut app = build_app();
//     let mut buf = vec![];
//     app.gen_completions_to("myapp", Shell::PowerShell, &mut buf);
//     let string = String::from_utf8(buf).unwrap();
//
//     assert!(compare(&*string, POWERSHELL));
// }

// Disabled until I figure out this windows line ending and AppVeyor issues
//#[test]
// fn powershell_with_underscore() {
//     let mut app = build_app_with_underscore();
//     let mut buf = vec![];
//     app.gen_completions_to("my_app", Shell::PowerShell, &mut buf);
//     let string = String::from_utf8(buf).unwrap();
//
//     assert!(compare(&*string, POWERSHELL_WUS));
// }

#[test]
fn bash_with_underscore() {
    let mut app = build_app_with_underscore();
    let mut buf = vec![];
    app.gen_completions_to("my_app", Shell::Bash, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, BASH_WUS));
}

#[test]
fn fish_with_underscore() {
    let mut app = build_app_with_underscore();
    let mut buf = vec![];
    app.gen_completions_to("my_app", Shell::Fish, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, FISH_WUS));
}

#[test]
fn zsh_with_underscore() {
    let mut app = build_app_with_underscore();
    let mut buf = vec![];
    app.gen_completions_to("my_app", Shell::Zsh, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, ZSH_WUS));
}
