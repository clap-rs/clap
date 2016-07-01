use std::path::PathBuf;
use std::fs::File;
use std::ffi::OsString;
use std::io::Write;

use app::parser::Parser;
use shell::Shell;
use args::{ArgSettings, OptBuilder};

macro_rules! w {
    ($_self:ident, $f:ident, $to_w:expr) => {
        match $f.write_all($to_w) {
            Ok(..) => (),
            Err(..) => panic!(format!("Failed to write to file completions file")),
        }
    };
}

pub struct ComplGen<'a, 'b> where 'a: 'b {
    p: &'b Parser<'a, 'b>,
    out_dir: OsString,
}

impl<'a, 'b> ComplGen<'a, 'b> {
    pub fn new(p: &'b Parser<'a, 'b>, od: OsString) -> Self {
        ComplGen {
            p: p,
            out_dir: od,
        }
    }

    pub fn generate(&self, for_shell: Shell) {
        match for_shell {
            Shell::Bash => self.gen_bash(),
        }
    }

    fn gen_bash(&self) {
        use std::error::Error;
        let out_dir = PathBuf::from(&self.out_dir);

        let mut file = match File::create(out_dir.join("bash.sh")) {
            Err(why) => panic!("couldn't create bash completion file: {}",
                why.description()),
            Ok(file) => file,
        };
        w!(self, file, format!(
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

complete -F _{name} {name}
",
            name=self.p.meta.bin_name.as_ref().unwrap(),
            name_opts=self.all_options_for_path(self.p.meta.bin_name.as_ref().unwrap()),
            name_opts_details=self.option_details_for_path(self.p.meta.bin_name.as_ref().unwrap()),
            subcmds=self.all_subcommands(),
            subcmd_details=self.subcommand_details()
        ).as_bytes());
    }

    fn all_subcommands(&self) -> String {
        let mut subcmds = String::new();
        let mut scs = get_all_subcommands(self.p);
        scs.sort();
        scs.dedup();

        for sc in &scs {
            subcmds = format!(
                "{}
            {name})
                cmd+=\"_{name}\"
                ;;",
                subcmds,
                name=sc.replace("-", "_"));
        }

        subcmds
    }

    fn subcommand_details(&self) -> String {
        let mut subcmd_dets = String::new();
        let mut scs = get_all_subcommand_paths(self.p, true);
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
                subcmd=sc.replace("-", "_"),
                sc_opts=self.all_options_for_path(&*sc),
                level=sc.split("_").map(|_|1).fold(0, |acc, n| acc + n),
                opts_details=self.option_details_for_path(&*sc)
            );
        }

        subcmd_dets
    }

    fn all_options_for_path(&self, path: &str) -> String {
        let mut p = self.p;
        for sc in path.split("_").skip(1) {
            debugln!("iter;sc={}", sc);
            p = &p.subcommands.iter().filter(|s| s.p.meta.name == sc).next().unwrap().p;
        }
        let mut opts = p.short_list.iter().fold(String::new(), |acc, s| format!("{} -{}", acc, s));
        opts = format!("{} {}", opts, p.long_list.iter()
                                                 .fold(String::new(), |acc, l| format!("{} --{}", acc, l)));
        opts = format!("{} {}", opts, p.positionals.values()
                                                   .fold(String::new(), |acc, p| format!("{} {}", acc, p)));
        opts = format!("{} {}", opts, p.subcommands.iter()
                                                   .fold(String::new(), |acc, s| format!("{} {}", acc, s.p.meta.name)));
        opts
    }

    fn option_details_for_path(&self, path: &str) -> String {
        let mut p = self.p;
        for sc in path.split("_").skip(1) {
            debugln!("iter;sc={}", sc);
            p = &p.subcommands.iter().filter(|s| s.p.meta.name == sc).next().unwrap().p;
        }
        let mut opts = String::new();
        for o in &p.opts {
            if let Some(l) = o.long {
                opts = format!("{}
                --{})
                    COMPREPLY=({})
                    return 0
                    ;;", opts, l, vals_for(o));
            }
            if let Some(s) = o.short {
                opts = format!("{}
                    -{})
                    COMPREPLY=({})
                    return 0
                    ;;", opts, s, vals_for(o));
            }
        }
        opts
    }
}

pub fn get_all_subcommands(p: &Parser) -> Vec<String> {
    let mut subcmds = vec![];
    if !p.has_subcommands() {
        return vec![p.meta.name.clone()]
    }
    for sc in p.subcommands.iter().map(|ref s| s.p.meta.name.clone()) {
        subcmds.push(sc);
    }
    for sc_v in p.subcommands.iter().map(|ref s| get_all_subcommands(&s.p)) {
        subcmds.extend(sc_v);
    }
    subcmds
}

pub fn get_all_subcommand_paths(p: &Parser, first: bool) -> Vec<String> {
    let mut subcmds = vec![];
    if !p.has_subcommands() {
        if !first {
            return vec![p.meta.bin_name.as_ref().unwrap().clone().replace(" ", "_")]
        }
        return vec![];
    }
    for sc in p.subcommands.iter()
                           .map(|ref s| s.p.meta.bin_name.as_ref()
                                                         .unwrap()
                                                         .clone()
                                                         .replace(" ", "_")) {
        subcmds.push(sc);
    }
    for sc_v in p.subcommands.iter().map(|ref s| get_all_subcommand_paths(&s.p, false)) {
        subcmds.extend(sc_v);
    }
    subcmds
}

fn vals_for(o: &OptBuilder) -> String {
    use args::AnyArg;
    let mut ret = String::new();
    let mut needs_quotes = true;
    if let Some(ref vals) = o.possible_vals() {
        needs_quotes = false;
        ret = format!("$(compgen -W \"{}\" -- ${{cur}})", vals.join(" "));
    } else if let Some(ref vec) = o.val_names() {
        let mut it = vec.iter().peekable();
        while let Some((_, val)) = it.next() {
            ret = format!("{}<{}>{}", ret, val,
                if it.peek().is_some() {
                    " "
                } else {
                    ""
                });
        }
        let num = vec.len();
        if o.is_set(ArgSettings::Multiple) && num == 1 {
            ret = format!("{}...", ret);
        }
    } else if let Some(num) = o.num_vals() {
        let mut it = (0..num).peekable();
        while let Some(_) = it.next() {
            ret = format!("{}<{}>{}", ret, o.name(),
            if it.peek().is_some() {
                " "
            } else {
                ""
            });
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
