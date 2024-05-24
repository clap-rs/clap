use crate::common;
use snapbox::assert_data_eq;

#[test]
fn basic() {
    let name = "my-app";
    let cmd = common::basic_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/basic.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn feature_sample() {
    let name = "my-app";
    let cmd = common::feature_sample_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/feature_sample.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn special_commands() {
    let name = "my-app";
    let cmd = common::special_commands_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/special_commands.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn quoting() {
    let name = "my-app";
    let cmd = common::quoting_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/quoting.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn aliases() {
    let name = "my-app";
    let cmd = common::aliases_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/aliases.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn sub_subcommands() {
    let name = "my-app";
    let cmd = common::sub_subcommands_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/sub_subcommands.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn custom_bin_name() {
    let name = "my-app";
    let bin_name = "bin-name";
    let cmd = common::basic_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/custom_bin_name.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        bin_name,
    );
}

#[test]
fn value_hint() {
    let name = "my-app";
    let cmd = common::value_hint_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/value_hint.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn value_terminator() {
    let name = "my-app";
    let cmd = common::value_terminator_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/value_terminator.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn two_multi_valued_arguments() {
    let name = "my-app";
    let cmd = common::two_multi_valued_arguments_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/two_multi_valued_arguments.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn subcommand_last() {
    let name = "my-app";
    let cmd = common::subcommand_last(name);
    common::assert_matches(
        snapbox::file!["../snapshots/subcommand_last.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
#[cfg(unix)]
fn register_completion() {
    common::register_example::<completest_pty::ElvishRuntimeBuilder>("static", "exhaustive");
}

#[test]
#[cfg(unix)]
fn complete() {
    if !common::has_command("elvish") {
        return;
    }

    let term = completest::Term::new();
    let mut runtime =
        common::load_runtime::<completest_pty::ElvishRuntimeBuilder>("static", "exhaustive");

    let input = "exhaustive \t";
    let expected = snapbox::str![
        r#"% exhaustive --generate
 COMPLETING argument  
--generate     generate                                                 
--global       everywhere                                               
--help         Print help                                               
--version      Print version                                            
-V             Print version                                            
-h             Print help                                               
action         action                                                   
alias          alias                                                    
complete       Register shell completions for this program              
help           Print this message or the help of the given subcommand(s)
hint           hint                                                     
last           last                                                     
pacman         pacman                                                   
quote          quote                                                    
value          value                                                    "#
    ];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);
}
