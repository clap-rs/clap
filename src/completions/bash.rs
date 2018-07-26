// Std
use std::io::Write;
use std::ffi::OsStr;

// Internal
use build::{App, Arg};
use completions;

pub struct BashGen<'a, 'b>(&'b App<'a, 'b>)
where
    'a: 'b;

impl<'a, 'b> BashGen<'a, 'b> {
    pub fn new(app: &'b App<'a, 'b>) -> Self { BashGen(app) }

    pub fn generate_to<W: Write>(&self, buf: &mut W) {
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
                cmd=\"{name}\"
                ;;
            {subcmds}
            *)
                ;;
        esac
    done

    case \"${{cmd}}\" in
        {name})
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
                name = self.0.bin_name.as_ref().unwrap(),
                name_opts = self.all_options_for_path(self.0.bin_name.as_ref().unwrap()),
                name_opts_details = self.option_details_for_path(self.0.bin_name.as_ref().unwrap()),
                subcmds = self.all_subcommands(),
                subcmd_details = self.subcommand_details()
            ).as_bytes()
        );
    }

    fn all_subcommands(&self) -> String {
        debugln!("BashGen::all_subcommands;");
        let mut subcmds = String::new();
        let scs = completions::all_subcommand_names(self.0);

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

    fn subcommand_details(&self) -> String {
        debugln!("BashGen::subcommand_details;");
        let mut subcmd_dets = String::new();
        let mut scs = completions::get_all_subcommand_paths(self.0, true);
        scs.sort();
        scs.dedup();

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
                sc_opts = self.all_options_for_path(&*sc),
                level = sc.split("__").map(|_| 1).fold(0, |acc, n| acc + n),
                opts_details = self.option_details_for_path(&*sc)
            );
        }

        subcmd_dets
    }

    fn option_details_for_path(&self, path: &str) -> String {
        debugln!("BashGen::option_details_for_path: path={}", path);
        let mut p = self.0;
        for sc in path.split("__").skip(1) {
            debugln!("BashGen::option_details_for_path:iter: sc={}", sc);
            p = &find_subcmd!(p, sc).unwrap();
        }
        let mut opts = String::new();
        for (_, o) in opts!(p) {
            if let Some(l) = o.long {
                opts = format!(
                    "{}
                --{})
                    COMPREPLY=({})
                    return 0
                    ;;",
                    opts,
                    l,
                    self.vals_for(o)
                );
            }
            if let Some(s) = o.short {
                opts = format!(
                    "{}
                    -{})
                    COMPREPLY=({})
                    return 0
                    ;;",
                    opts,
                    s,
                    self.vals_for(o)
                );
            }
        }
        opts
    }

    fn vals_for(&self, o: &Arg) -> String {
        debugln!("BashGen::vals_for: o={}", o.name);
        if let Some(ref vals) = o.possible_vals {
            format!("$(compgen -W \"{}\" -- ${{cur}})", vals.join(" "))
        } else {
            String::from("$(compgen -f ${cur})")
        }
    }

    fn all_options_for_path(&self, path: &str) -> String {
        debugln!("BashGen::all_options_for_path: path={}", path);
        let mut p = self.0;
        for sc in path.split("__").skip(1) {
            debugln!("BashGen::all_options_for_path:iter: sc={}", sc);
            p = &find_subcmd!(p, sc).unwrap();
        }
        let opts = format!(
            "{shorts} {longs} {pos} {subcmds}",
            shorts = shorts!(p).fold(String::new(), |acc, s| format!("{} -{}", acc, s)),
            // Handles aliases too
            // error-handling?
            longs = longs!(p).fold(String::new(), |acc, l| format!("{} --{}", acc, l.to_str().unwrap()),
            pos = positionals!(p).fold(String::new(), |acc, p| format!("{} {}", acc, p)),
            // Handles aliases too
            subcmds = sc_names!(p).fold(String::new(), |acc, s| format!("{} {}", acc, s))
        );
        opts
    }
}
