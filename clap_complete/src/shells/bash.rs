use std::{fmt::Write as _, io::Write};

use clap::*;

use crate::generator::{utils, Generator};

/// Generate bash completion file
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Bash;

impl Generator for Bash {
    fn file_name(&self, name: &str) -> String {
        format!("{}.bash", name)
    }

    fn generate(&self, cmd: &Command, buf: &mut dyn Write) {
        let bin_name = cmd
            .get_bin_name()
            .expect("crate::generate should have set the bin_name");

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
            \"$1\")
                cmd=\"{cmd}\"
                ;;{subcmds}
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
            case \"${{prev}}\" in{name_opts_details}
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W \"${{opts}}\" -- \"${{cur}}\") )
            return 0
            ;;{subcmd_details}
    esac
}}

complete -F _{name} -o bashdefault -o default {name}
",
                name = bin_name,
                cmd = bin_name.replace('-', "__"),
                name_opts = all_options_for_path(cmd, bin_name),
                name_opts_details = option_details_for_path(cmd, bin_name),
                subcmds = all_subcommands(cmd),
                subcmd_details = subcommand_details(cmd)
            )
            .as_bytes()
        );
    }
}

fn all_subcommands(cmd: &Command) -> String {
    debug!("all_subcommands");

    let mut subcmds = vec![String::new()];
    let mut scs = utils::all_subcommands(cmd)
        .iter()
        .map(|x| x.0.clone())
        .collect::<Vec<_>>();

    scs.sort();
    scs.dedup();

    subcmds.extend(scs.iter().map(|sc| {
        format!(
            "{name})
                cmd+=\"__{fn_name}\"
                ;;",
            name = sc,
            fn_name = sc.replace('-', "__")
        )
    }));

    subcmds.join("\n            ")
}

fn subcommand_details(cmd: &Command) -> String {
    debug!("subcommand_details");

    let mut subcmd_dets = vec![String::new()];
    let mut scs = utils::all_subcommands(cmd)
        .iter()
        .map(|x| x.1.replace(' ', "__"))
        .collect::<Vec<_>>();

    scs.sort();

    subcmd_dets.extend(scs.iter().map(|sc| {
        format!(
            "{subcmd})
            opts=\"{sc_opts}\"
            if [[ ${{cur}} == -* || ${{COMP_CWORD}} -eq {level} ]] ; then
                COMPREPLY=( $(compgen -W \"${{opts}}\" -- \"${{cur}}\") )
                return 0
            fi
            case \"${{prev}}\" in{opts_details}
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W \"${{opts}}\" -- \"${{cur}}\") )
            return 0
            ;;",
            subcmd = sc.replace('-', "__"),
            sc_opts = all_options_for_path(cmd, &*sc),
            level = sc.split("__").map(|_| 1).sum::<u64>(),
            opts_details = option_details_for_path(cmd, &*sc)
        )
    }));

    subcmd_dets.join("\n        ")
}

fn option_details_for_path(cmd: &Command, path: &str) -> String {
    debug!("option_details_for_path: path={}", path);

    let p = utils::find_subcommand_with_path(cmd, path.split("__").skip(1).collect());
    let mut opts = vec![String::new()];

    for o in p.get_opts() {
        if let Some(longs) = o.get_long_and_visible_aliases() {
            opts.extend(longs.iter().map(|long| {
                format!(
                    "--{})
                    COMPREPLY=({})
                    return 0
                    ;;",
                    long,
                    vals_for(o)
                )
            }));
        }

        if let Some(shorts) = o.get_short_and_visible_aliases() {
            opts.extend(shorts.iter().map(|short| {
                format!(
                    "-{})
                    COMPREPLY=({})
                    return 0
                    ;;",
                    short,
                    vals_for(o)
                )
            }));
        }
    }

    opts.join("\n                ")
}

fn vals_for(o: &Arg) -> String {
    debug!("vals_for: o={}", o.get_id());

    if let Some(vals) = o.get_possible_values() {
        format!(
            "$(compgen -W \"{}\" -- \"${{cur}}\")",
            vals.iter()
                .filter(|pv| pv.is_hide_set())
                .map(PossibleValue::get_name)
                .collect::<Vec<_>>()
                .join(" ")
        )
    } else {
        String::from("$(compgen -f \"${cur}\")")
    }
}

fn all_options_for_path(cmd: &Command, path: &str) -> String {
    debug!("all_options_for_path: path={}", path);

    let p = utils::find_subcommand_with_path(cmd, path.split("__").skip(1).collect());

    let mut opts = String::new();
    for short in utils::shorts_and_visible_aliases(p) {
        write!(&mut opts, "-{} ", short).unwrap();
    }
    for long in utils::longs_and_visible_aliases(p) {
        write!(&mut opts, "--{} ", long).unwrap();
    }
    for pos in p.get_positionals() {
        if let Some(vals) = pos.get_possible_values() {
            for value in vals {
                write!(&mut opts, "{} ", value.get_name()).unwrap();
            }
        } else {
            write!(&mut opts, "{} ", pos).unwrap();
        }
    }
    for (sc, _) in utils::subcommands(p) {
        write!(&mut opts, "{} ", sc).unwrap();
    }
    opts.pop();

    opts
}
