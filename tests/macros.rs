#[macro_use]
extern crate clap;

#[test]
fn basic() {
    clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
        (@arg opt: -o --option +takes_value ... "tests options")
        (@arg positional: index(1) "tests positionals")
        (@arg flag: -f --flag ... +global "tests flags")
        (@arg flag2: -F conflicts_with[flag] requires[option2]
            "tests flags with exclusions")
        (@arg option2: --long_option_2 conflicts_with[option] requires[positional2]
            "tests long options with exclusions")
        (@arg positional2: index(2) "tests positionals with exclusions")
        (@arg option3: -O --Option +takes_value possible_value[fast slow]
            "tests options with specific value sets")
        (@arg positional3: index(3) ... possible_value[vi emacs]
            "tests positionals with specific values")
        (@arg multvals: --multvals +takes_value value_name[one two]
            "Tests mutliple values, not mult occs")
        (@arg multvalsmo: --multvalsmo ... +takes_value value_name[one two]
            "Tests mutliple values, not mult occs")
        (@arg minvals: --minvals2 min_values(1) ... +takes_value "Tests 2 min vals")
        (@arg maxvals: --maxvals3 ... +takes_value max_values(3) "Tests 3 max vals")
        (@subcommand subcmd =>
            (about: "tests subcommands")
            (version: "0.1")
            (author: "Kevin K. <kbknapp@gmail.com>")
            (@arg scoption: -o --option ... +takes_value "tests options")
            (@arg scpositional: index(1) "tests positionals"))
    );
}
