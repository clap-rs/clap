// Std
use std::fmt::Write as _;
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
                COMPREPLY=( $(compgen -W \"${{opts}}\" -- \"${{cur}}\") )
                return 0
            fi
            case \"${{prev}}\" in
                {name_opts_details}
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W \"${{opts}}\" -- \"${{cur}}\") )
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
    debug!("all_subcommands");

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
    debug!("subcommand_details");

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
                COMPREPLY=( $(compgen -W \"${{opts}}\" -- \"${{cur}}\") )
                return 0
            fi
            case \"${{prev}}\" in
                {opts_details}
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W \"${{opts}}\" -- \"${{cur}}\") )
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
    debug!("option_details_for_path: path={}", path);

    let p = Bash::find_subcommand_with_path(app, path.split("__").skip(1).collect());
    let mut opts = String::new();

    for o in p.get_opts() {
        if let Some(longs) = o.get_long_and_visible_aliases() {
            for long in longs {
                opts = format!(
                    "{}
                --{})
                    COMPREPLY=({})
                    return 0
                    ;;",
                    opts,
                    long,
                    vals_for(o)
                );
            }
        }

        if let Some(shorts) = o.get_short_and_visible_aliases() {
            for short in shorts {
                opts = format!(
                    "{}
                -{})
                    COMPREPLY=({})
                    return 0
                    ;;",
                    opts,
                    short,
                    vals_for(o)
                );
            }
        }
    }

    opts
}

fn vals_for(o: &Arg) -> String {
    debug!("vals_for: o={}", o.get_name());

    if let Some(ref vals) = o.get_possible_values() {
        format!("$(compgen -W \"{}\" -- \"${{cur}}\")", vals.join(" "))
    } else {
        String::from("$(compgen -f \"${cur}\")")
    }
}

fn all_options_for_path(app: &App, path: &str) -> String {
    debug!("all_options_for_path: path={}", path);

    let p = Bash::find_subcommand_with_path(app, path.split("__").skip(1).collect());

    let mut opts = String::new();
    for short in Bash::shorts_and_visible_aliases(p) {
        write!(&mut opts, "-{} ", short).unwrap();
    }
    for long in Bash::longs_and_visible_aliases(p) {
        write!(&mut opts, "--{} ", long).unwrap();
    }
    for pos in p.get_positionals() {
        for value in pos.get_possible_values().unwrap_or_default() {
            write!(&mut opts, "{} ", value).unwrap();
        }
    }
    for (sc, _) in Bash::subcommands(p) {
        write!(&mut opts, "{} ", sc).unwrap();
    }
    opts.pop(); // Remove the space at the end.

    opts
}
