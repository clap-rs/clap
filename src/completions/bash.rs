// Std
use std::io::Write;

// Internal
use app::parser::Parser;
use args::{ArgSettings, OptBuilder};
use completions;

pub struct BashGen<'a, 'b>
    where 'a: 'b
{
    p: &'b Parser<'a, 'b>,
}

impl<'a, 'b> BashGen<'a, 'b> {
    pub fn new(p: &'b Parser<'a, 'b>) -> Self { BashGen { p: p } }

    pub fn generate_to<W: Write>(&self, buf: &mut W) {

        w!(buf,
           format!("_{name}() {{
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
                   name = self.p.meta.bin_name.as_ref().unwrap(),
                   name_opts = self.all_options_for_path(self.p.meta.bin_name.as_ref().unwrap()),
                   name_opts_details =
                       self.option_details_for_path(self.p.meta.bin_name.as_ref().unwrap()),
                   subcmds = self.all_subcommands(),
                   subcmd_details = self.subcommand_details())
               .as_bytes());
    }

    fn all_subcommands(&self) -> String {
        debugln!("BashGen::all_subcommands;");
        let mut subcmds = String::new();
        let scs = completions::all_subcommand_names(self.p);

        for sc in &scs {
            subcmds = format!("{}
            {name})
                cmd+=\"__{name}\"
                ;;",
                              subcmds,
                              name = sc.replace("-", "__"));
        }

        subcmds
    }

    fn subcommand_details(&self) -> String {
        debugln!("BashGen::subcommand_details;");
        let mut subcmd_dets = String::new();
        let mut scs = completions::get_all_subcommand_paths(self.p, true);
        scs.sort();
        scs.dedup();

        for sc in &scs {
            subcmd_dets = format!("{}
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
                                  opts_details = self.option_details_for_path(&*sc));
        }

        subcmd_dets
    }

    fn option_details_for_path(&self, path: &str) -> String {
        debugln!("BashGen::option_details_for_path: path={}", path);
        let mut p = self.p;
        for sc in path.split("__").skip(1) {
            debugln!("BashGen::option_details_for_path:iter: sc={}", sc);
            p = &p.subcommands
                .iter()
                .find(|s| {
                    s.p.meta.name == sc ||
                    (s.p.meta.aliases.is_some() &&
                     s.p
                        .meta
                        .aliases
                        .as_ref()
                        .unwrap()
                        .iter()
                        .any(|&(n, _)| n == sc))
                })
                .unwrap()
                .p;
        }
        let mut opts = String::new();
        for o in p.opts() {
            if let Some(l) = o.s.long {
                opts = format!("{}
                --{})
                    COMPREPLY=({})
                    return 0
                    ;;",
                               opts,
                               l,
                               self.vals_for(o));
            }
            if let Some(s) = o.s.short {
                opts = format!("{}
                    -{})
                    COMPREPLY=({})
                    return 0
                    ;;",
                               opts,
                               s,
                               self.vals_for(o));
            }
        }
        opts
    }

    fn vals_for(&self, o: &OptBuilder) -> String {
        debugln!("BashGen::vals_for: o={}", o.b.name);
        use args::AnyArg;
        let mut ret = String::new();
        let mut needs_quotes = true;
        if let Some(vals) = o.possible_vals() {
            needs_quotes = false;
            ret = format!("$(compgen -W \"{}\" -- ${{cur}})", vals.join(" "));
        } else if let Some(vec) = o.val_names() {
            let mut it = vec.iter().peekable();
            while let Some((_, val)) = it.next() {
                ret = format!("{}<{}>{}",
                              ret,
                              val,
                              if it.peek().is_some() { " " } else { "" });
            }
            let num = vec.len();
            if o.is_set(ArgSettings::Multiple) && num == 1 {
                ret = format!("{}...", ret);
            }
        } else if let Some(num) = o.num_vals() {
            let mut it = (0..num).peekable();
            while let Some(_) = it.next() {
                ret = format!("{}<{}>{}",
                              ret,
                              o.name(),
                              if it.peek().is_some() { " " } else { "" });
            }
            if o.is_set(ArgSettings::Multiple) && num == 1 {
                ret = format!("{}...", ret);
            }
        } else {
            ret = format!("<{}>", o.name());
            if o.is_set(ArgSettings::Multiple) {
                ret = format!("{}...", ret);
            }
        }
        if needs_quotes {
            ret = format!("\"{}\"", ret);
        }
        ret
    }
    fn all_options_for_path(&self, path: &str) -> String {
        debugln!("BashGen::all_options_for_path: path={}", path);
        let mut p = self.p;
        for sc in path.split("__").skip(1) {
            debugln!("BashGen::all_options_for_path:iter: sc={}", sc);
            p = &p.subcommands
                .iter()
                .find(|s| {
                    s.p.meta.name == sc ||
                    (s.p.meta.aliases.is_some() &&
                     s.p
                        .meta
                        .aliases
                        .as_ref()
                        .unwrap()
                        .iter()
                        .any(|&(n, _)| n == sc))
                })
                .unwrap()
                .p;
        }
        let mut opts = p.short_list.iter().fold(String::new(), |acc, s| format!("{} -{}", acc, s));
        opts = format!("{} {}",
                       opts,
                       p.long_list
                           .iter()
                           .fold(String::new(), |acc, l| format!("{} --{}", acc, l)));
        opts = format!("{} {}",
                       opts,
                       p.positionals
                           .values()
                           .fold(String::new(), |acc, p| format!("{} {}", acc, p)));
        opts = format!("{} {}",
                       opts,
                       p.subcommands
                           .iter()
                           .fold(String::new(), |acc, s| format!("{} {}", acc, s.p.meta.name)));
        for sc in &p.subcommands {
            if let Some(ref aliases) = sc.p.meta.aliases {
                opts = format!("{} {}",
                               opts,
                               aliases.iter()
                                   .map(|&(n, _)| n)
                                   .fold(String::new(), |acc, a| format!("{} {}", acc, a)));
            }
        }
        opts
    }
}
