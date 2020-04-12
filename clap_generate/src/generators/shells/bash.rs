// Std
use std::io::Write;

// Internal
use crate::Generator;
use clap::*;

/// Generate bash completion file
pub struct Bash;

impl Generator for Bash {
    fn file_name(name: &str) -> String {
        format!("{}.bash", name)
    }

    fn generate(app: &App, buf: &mut dyn Write) {
        let bin_name = app.get_bin_name().unwrap();

        w!(
            buf,
            format!(
                "_{name}() {{
    local i cur prev opts cmds
    COMPREPLY=()
    cur=\"${{COMP_WORDS[COMP_CWORD]}}\"
    prev=\"${{COMP_WORDS[COMP_CWORD-1]}}\"
    cmd=\"\"
    opts=\"\"

    for i in ${{COMP_WORDS[@]}}
    do
        case \"${{i}}\" in
            {name})
                cmd=\"{cmd}\"
                ;;
            {subcmds}
            *)
                ;;
        esac
    done

    case \"${{cmd}}\" in
        {cmd})
            opts=\"{name_opts}\"
            if [[ ${{cur}} == -* || ${{COMP_CWORD}} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W \"${{opts}}\" -- ${{cur}}) )
                return 0
            fi
            case \"${{prev}}\" in
                {name_opts_details}
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W \"${{opts}}\" -- ${{cur}}) )
            return 0
            ;;
        {subcmd_details}
    esac
}}

complete -F _{name} -o bashdefault -o default {name}
",
                name = bin_name,
                cmd = bin_name.replace("-", "__"),
                name_opts = all_options_for_path(app, bin_name),
                name_opts_details = option_details_for_path(app, bin_name),
                subcmds = all_subcommands(app),
                subcmd_details = subcommand_details(app)
            )
            .as_bytes()
        );
    }
}

fn all_subcommands(app: &App) -> String {
    debugln!("Bash::all_subcommands;");

    let mut subcmds = String::new();
    let mut scs = Bash::all_subcommands(app)
        .iter()
        .map(|x| x.0.clone())
        .collect::<Vec<_>>();

    scs.sort();
    scs.dedup();

    for sc in &scs {
        subcmds = format!(
            "{}
            {name})
                cmd+=\"__{fn_name}\"
                ;;",
            subcmds,
            name = sc,
            fn_name = sc.replace("-", "__")
        );
    }

    subcmds
}

fn subcommand_details(app: &App) -> String {
    debugln!("Bash::subcommand_details;");

    let mut subcmd_dets = String::new();
    let mut scs = Bash::all_subcommands(app)
        .iter()
        .map(|x| x.1.replace(" ", "__"))
        .collect::<Vec<_>>();

    scs.sort();

    for sc in &scs {
        subcmd_dets = format!(
            "{}
        {subcmd})
            opts=\"{sc_opts}\"
            if [[ ${{cur}} == -* || ${{COMP_CWORD}} -eq {level} ]] ; then
                COMPREPLY=( $(compgen -W \"${{opts}}\" -- ${{cur}}) )
                return 0
            fi
            case \"${{prev}}\" in
                {opts_details}
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W \"${{opts}}\" -- ${{cur}}) )
            return 0
            ;;",
            subcmd_dets,
            subcmd = sc.replace("-", "__"),
            sc_opts = all_options_for_path(app, &*sc),
            level = sc.split("__").map(|_| 1).sum::<u64>(),
            opts_details = option_details_for_path(app, &*sc)
        );
    }

    subcmd_dets
}

fn option_details_for_path(app: &App, path: &str) -> String {
    debugln!("Bash::option_details_for_path: path={}", path);

    let p = Bash::find_subcommand_with_path(app, path.split("__").skip(1).collect());
    let mut opts = String::new();

    for o in opts!(p) {
        if let Some(l) = o.get_long() {
            opts = format!(
                "{}
                --{})
                    COMPREPLY=({})
                    return 0
                    ;;",
                opts,
                l,
                vals_for(o)
            );
        }

        if let Some(s) = o.get_short() {
            opts = format!(
                "{}
                    -{})
                    COMPREPLY=({})
                    return 0
                    ;;",
                opts,
                s,
                vals_for(o)
            );
        }
    }

    opts
}

fn vals_for(o: &Arg) -> String {
    debugln!("Bash::vals_for: o={}", o.get_name());

    if let Some(ref vals) = o.get_possible_values() {
        format!("$(compgen -W \"{}\" -- ${{cur}})", vals.join(" "))
    } else {
        String::from("$(compgen -f ${cur})")
    }
}

fn all_options_for_path(app: &App, path: &str) -> String {
    debugln!("Bash::all_options_for_path: path={}", path);

    let p = Bash::find_subcommand_with_path(app, path.split("__").skip(1).collect());
    let scs: Vec<_> = Bash::subcommands(p).iter().map(|x| x.0.clone()).collect();

    let opts = format!(
        "{shorts} {longs} {pos} {subcmds}",
        shorts = Bash::shorts(p)
            .iter()
            .fold(String::new(), |acc, s| format!("{} -{}", acc, s)),
        longs = Bash::longs(p)
            .iter()
            .fold(String::new(), |acc, l| format!("{} --{}", acc, l)),
        pos = positionals!(p).fold(String::new(), |acc, p| format!("{} {}", acc, p)),
        subcmds = scs.join(" "),
    );

    opts
}
