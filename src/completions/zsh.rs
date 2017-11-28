// Std
use std::io::Write;
#[allow(unused_imports)]
use std::ascii::AsciiExt;

// Internal
use app::App;
use app::parser::Parser;
use args::{AnyArg, ArgSettings};
use completions;
use INTERNAL_ERROR_MSG;

pub struct ZshGen<'a, 'b>
where
    'a: 'b,
{
    p: &'b Parser<'a, 'b>,
}

impl<'a, 'b> ZshGen<'a, 'b> {
    pub fn new(p: &'b Parser<'a, 'b>) -> Self {
        debugln!("ZshGen::new;");
        ZshGen { p: p }
    }

    pub fn generate_to<W: Write>(&self, buf: &mut W) {
        debugln!("ZshGen::generate_to;");
        w!(
            buf,
            format!(
                "\
#compdef {name}

_{name}() {{
    typeset -A opt_args
    local ret=1

    local context curcontext=\"$curcontext\" state line
    {initial_args}
    {subcommands}
}}

{subcommand_details}

_{name} \"$@\"",
                name = self.p.meta.bin_name.as_ref().unwrap(),
                initial_args = get_args_of(self.p),
                subcommands = get_subcommands_of(self.p),
                subcommand_details = subcommand_details(self.p)
            ).as_bytes()
        );
    }
}

// Displays the positional args and commands of a subcommand
// (( $+functions[_[bin_name_underscore]_commands] )) ||
// _[bin_name_underscore]_commands() {
// 	local commands; commands=(
// 		'[arg_name]:[arg_help]'
// 	)
// 	_describe -t commands '[bin_name] commands' commands "$@"
//
// Where the following variables are present:
//    [bin_name_underscore]: The full space deliniated bin_name, where spaces have been replaced by
//                           underscore characters
//    [arg_name]: The name of the positional arg or subcommand
//    [arg_help]: The help message of the arg or subcommand
//    [bin_name]: The full space deliniated bin_name
//
// Here's a snippet from rustup:
//
// (( $+functions[_rustup_commands] )) ||
// _rustup_commands() {
// 	local commands; commands=(
// 		'show:Show the active and installed toolchains'
//      'update:Update Rust toolchains'
//      # ... snip for brevity
//      'help:Prints this message or the help of the given subcommand(s)'
// 	)
// 	_describe -t commands 'rustup commands' commands "$@"
//
fn subcommand_details(p: &Parser) -> String {
    debugln!("ZshGen::subcommand_details;");
    // First we do ourself
    let mut ret = vec![
        format!(
            "\
(( $+functions[_{bin_name_underscore}_commands] )) ||
_{bin_name_underscore}_commands() {{
    local commands; commands=(
        {subcommands_and_args}
    )
    _describe -t commands '{bin_name} commands' commands \"$@\"
}}",
            bin_name_underscore = p.meta.bin_name.as_ref().unwrap().replace(" ", "__"),
            bin_name = p.meta.bin_name.as_ref().unwrap(),
            subcommands_and_args = subcommands_and_args_of(p)
        ),
    ];

    // Next we start looping through all the children, grandchildren, etc.
    let mut all_subcommands = completions::all_subcommands(p);
    all_subcommands.sort();
    all_subcommands.dedup();
    for &(_, ref bin_name) in &all_subcommands {
        debugln!("ZshGen::subcommand_details:iter: bin_name={}", bin_name);
        ret.push(format!(
            "\
(( $+functions[_{bin_name_underscore}_commands] )) ||
_{bin_name_underscore}_commands() {{
    local commands; commands=(
        {subcommands_and_args}
    )
    _describe -t commands '{bin_name} commands' commands \"$@\"
}}",
            bin_name_underscore = bin_name.replace(" ", "__"),
            bin_name = bin_name,
            subcommands_and_args = subcommands_and_args_of(parser_of(p, bin_name))
        ));
    }

    ret.join("\n")
}

// Generates subcommand and positional argument completions in form of
//
// 		'[arg_name]:[arg_help]'
//
// Where:
//    [arg_name]: the argument or subcommand's name
//    [arg_help]: the help message of the argument or subcommand
//
// A snippet from rustup:
// 		'show:Show the active and installed toolchains'
//      'update:Update Rust toolchains'
fn subcommands_and_args_of(p: &Parser) -> String {
    debugln!("ZshGen::subcommands_and_args_of;");
    let mut ret = vec![];
    fn add_sc(sc: &App, n: &str, ret: &mut Vec<String>) {
        debugln!("ZshGen::add_sc;");
        let s = format!(
            "\"{name}:{help}\" \\",
            name = n,
            help = sc.p
                .meta
                .about
                .unwrap_or("")
                .replace("[", "\\[")
                .replace("]", "\\]")
        );
        if !s.is_empty() {
            ret.push(s);
        }
    }

    // First the subcommands
    for sc in p.subcommands() {
        debugln!(
            "ZshGen::subcommands_and_args_of:iter: subcommand={}",
            sc.p.meta.name
        );
        add_sc(sc, &sc.p.meta.name, &mut ret);
        if let Some(ref v) = sc.p.meta.aliases {
            for alias in v.iter().filter(|&&(_, vis)| vis).map(|&(n, _)| n) {
                add_sc(sc, alias, &mut ret);
            }
        }
    }

    // Then the positional args
    for arg in p.positionals() {
        debugln!("ZshGen::subcommands_and_args_of:iter: arg={}", arg.b.name);
        let a = format!(
            "\"{name}:{help}\" \\",
            name = arg.b.name.to_ascii_uppercase(),
            help = arg.b
                .help
                .unwrap_or("")
                .replace("[", "\\[")
                .replace("]", "\\]")
        );

        if !a.is_empty() {
            ret.push(a);
        }
    }

    ret.join("\n")
}

// Get's the subcommand section of a completion file
// This looks roughly like:
//
// case $state in
// ([bin_name]_args)
//     curcontext=\"${curcontext%:*:*}:[name_hyphen]-command-$words[1]:\"
//     case $line[1] in
//
//         ([name])
//         _arguments -C -s -S \
//             [subcommand_args]
//         && ret=0
//
//         [RECURSIVE_CALLS]
//
//         ;;",
//
//         [repeat]
//
//     esac
// ;;
// esac",
//
// Where the following variables are present:
//    [name] = The subcommand name in the form of "install" for "rustup toolchain install"
//    [bin_name] = The full space deliniated bin_name such as "rustup toolchain install"
//    [name_hyphen] = The full space deliniated bin_name, but replace spaces with hyphens
//    [repeat] = From the same recursive calls, but for all subcommands
//    [subcommand_args] = The same as zsh::get_args_of
fn get_subcommands_of(p: &Parser) -> String {
    debugln!("get_subcommands_of;");

    debugln!(
        "get_subcommands_of: Has subcommands...{:?}",
        p.has_subcommands()
    );
    if !p.has_subcommands() {
        return String::new();
    }

    let sc_names = completions::subcommands_of(p);

    let mut subcmds = vec![];
    for &(ref name, ref bin_name) in &sc_names {
        let mut v = vec![format!("({})", name)];
        let subcommand_args = get_args_of(parser_of(p, &*bin_name));
        if !subcommand_args.is_empty() {
            v.push(subcommand_args);
        }
        let subcommands = get_subcommands_of(parser_of(p, &*bin_name));
        if !subcommands.is_empty() {
            v.push(subcommands);
        }
        v.push(String::from(";;"));
        subcmds.push(v.join("\n"));
    }

    format!(
        "case $state in
    ({name})
        curcontext=\"${{curcontext%:*:*}}:{name_hyphen}-command-$words[1]:\"
        case $line[1] in
            {subcommands}
        esac
    ;;
esac",
        name = p.meta.name,
        name_hyphen = p.meta.bin_name.as_ref().unwrap().replace(" ", "-"),
        subcommands = subcmds.join("\n")
    )
}

fn parser_of<'a, 'b>(p: &'b Parser<'a, 'b>, sc: &str) -> &'b Parser<'a, 'b> {
    debugln!("parser_of: sc={}", sc);
    if sc == p.meta.bin_name.as_ref().unwrap_or(&String::new()) {
        return p;
    }
    &p.find_subcommand(sc).expect(INTERNAL_ERROR_MSG).p
}

// Writes out the args section, which ends up being the flags and opts, and a jump to
// another ZSH function if there are positional args or subcommands.
// The structer works like this:
//    ([conflicting_args]) [multiple] arg [takes_value] [[help]] [: :(possible_values)]
//       ^-- list '-v -h'    ^--'*'          ^--'+'                   ^-- list 'one two three'
//
// An example from the rustup command:
//
// _arguments -C -s -S \
// 		'(-h --help --verbose)-v[Enable verbose output]' \
// 		'(-V -v --version --verbose --help)-h[Prints help information]' \
//      # ... snip for brevity
// 		'1:: :_rustup_commands' \   # <-- displays positional args and subcommands
// 		'*:: :->rustup' \           # <-- displays subcommand args and child subcommands
// 	&& ret=0
//
// The args used for _arguments are as follows:
//    -C: modify the $context internal variable
//    -s: Allow stacking of short args (i.e. -a -b -c => -abc)
//    -S: Do not complete anything after '--' and treat those as argument values
fn get_args_of(p: &Parser) -> String {
    debugln!("get_args_of;");
    let mut ret = vec![String::from("_arguments -s -S -C \\")];
    let opts = write_opts_of(p);
    let flags = write_flags_of(p);
    let sc_or_a = if p.has_subcommands() || p.has_positionals() {
        format!(
            "\"1:: :_{name}_commands\" \\",
            name = p.meta.bin_name.as_ref().unwrap().replace(" ", "__")
        )
    } else {
        String::new()
    };
    let sc = if p.has_subcommands() {
        format!("\"*:: :->{name}\" \\", name = p.meta.name)
    } else {
        String::new()
    };

    if !opts.is_empty() {
        ret.push(opts);
    }
    if !flags.is_empty() {
        ret.push(flags);
    }
    if !sc_or_a.is_empty() {
        ret.push(sc_or_a);
    }
    if !sc.is_empty() {
        ret.push(sc);
    }
    ret.push(String::from("&& ret=0"));

    ret.join("\n")
}

// Escape string inside single quotes and brackets
fn escape_string(string: &str) -> String {
    string
        .replace("\\", "\\\\")
        .replace("'", "'\\''")
        .replace("[", "\\[")
        .replace("]", "\\]")
}

fn write_opts_of(p: &Parser) -> String {
    debugln!("write_opts_of;");
    let mut ret = vec![];
    for o in p.opts() {
        debugln!("write_opts_of:iter: o={}", o.name());
        let help = o.help().map_or(String::new(), escape_string);
        let mut conflicts = get_zsh_arg_conflicts!(p, o, INTERNAL_ERROR_MSG);
        conflicts = if conflicts.is_empty() {
            String::new()
        } else {
            format!("({})", conflicts)
        };

        let multiple = if o.is_set(ArgSettings::Multiple) {
            "*"
        } else {
            ""
        };
        let pv = if let Some(pv_vec) = o.possible_vals() {
            format!(": :({})", pv_vec.join(" "))
        } else {
            String::new()
        };
        if let Some(short) = o.short() {
            let s = format!(
                "'{conflicts}{multiple}-{arg}+[{help}]{possible_values}' \\",
                conflicts = conflicts,
                multiple = multiple,
                arg = short,
                possible_values = pv,
                help = help
            );

            debugln!("write_opts_of:iter: Wrote...{}", &*s);
            ret.push(s);
        }
        if let Some(long) = o.long() {
            let l = format!(
                "'{conflicts}{multiple}--{arg}+[{help}]{possible_values}' \\",
                conflicts = conflicts,
                multiple = multiple,
                arg = long,
                possible_values = pv,
                help = help
            );

            debugln!("write_opts_of:iter: Wrote...{}", &*l);
            ret.push(l);
        }
    }

    ret.join("\n")
}

fn write_flags_of(p: &Parser) -> String {
    debugln!("write_flags_of;");
    let mut ret = vec![];
    for f in p.flags() {
        debugln!("write_flags_of:iter: f={}", f.name());
        let help = f.help().map_or(String::new(), escape_string);
        let mut conflicts = get_zsh_arg_conflicts!(p, f, INTERNAL_ERROR_MSG);
        conflicts = if conflicts.is_empty() {
            String::new()
        } else {
            format!("({})", conflicts)
        };

        let multiple = if f.is_set(ArgSettings::Multiple) {
            "*"
        } else {
            ""
        };
        if let Some(short) = f.short() {
            let s = format!(
                "'{conflicts}{multiple}-{arg}[{help}]' \\",
                multiple = multiple,
                conflicts = conflicts,
                arg = short,
                help = help
            );

            debugln!("write_flags_of:iter: Wrote...{}", &*s);
            ret.push(s);
        }

        if let Some(long) = f.long() {
            let l = format!(
                "'{conflicts}{multiple}--{arg}[{help}]' \\",
                conflicts = conflicts,
                multiple = multiple,
                arg = long,
                help = help
            );

            debugln!("write_flags_of:iter: Wrote...{}", &*l);
            ret.push(l);
        }
    }

    ret.join("\n")
}
