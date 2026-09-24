/// Tests for GitHub issue #4707: `requires` validation should not be bypassed when 
/// arguments are in a mutually exclusive group.
/// 
/// This issue appears to have been resolved in the current version of clap.
/// These tests verify that the requires validation works correctly.
use clap::{Arg, ArgAction, ArgGroup, Command, error::ErrorKind};

#[cfg(feature = "derive")]
mod derive_tests {
    use clap::{ArgGroup, Parser};

    #[derive(Parser, Debug)]
    #[clap(group = ArgGroup::new("command").multiple(false))]
    struct Args {
        #[clap(long, group = "command")]
        read: bool,

        #[clap(long, group = "command")]
        write: bool,

        #[clap(long, requires = "read")]
        show_hex: bool,
    }

    #[test]
    fn issue_4707_original_derive_example() {
        // This is the exact example from the GitHub issue
        // It should fail when --show-hex is used without --read
        let result = Args::try_parse_from(["test", "--show-hex"]);
        
        assert!(result.is_err(), "Should fail because --show-hex requires --read");
        assert_eq!(result.unwrap_err().kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn issue_4707_derive_example_with_write() {
        // Test the problematic case: using --write and --show-hex together
        // This should fail because --show-hex requires --read, but --write and --read are mutually exclusive
        let result = Args::try_parse_from(["test", "--write", "--show-hex"]);
        
        assert!(result.is_err(), "Should fail because --show-hex requires --read but --write and --read are mutually exclusive");
        assert_eq!(result.unwrap_err().kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn issue_4707_derive_example_valid_case() {
        // This should succeed when --read and --show-hex are used together
        let result = Args::try_parse_from(["test", "--read", "--show-hex"]);
        
        assert!(result.is_ok(), "Should succeed when --read and --show-hex are used together");
        let args = result.unwrap();
        assert!(args.read);
        assert!(args.show_hex);
        assert!(!args.write);
    }
}

#[test]
fn issue_4707_requires_should_be_validated_when_args_are_in_group() {
    // This test ensures that `requires` validation is NOT bypassed 
    // when arguments are in a mutually exclusive group
    let cmd = Command::new("test")
        .arg(Arg::new("one").short('1').action(ArgAction::SetTrue).requires("foo"))
        .arg(Arg::new("two").short('2').action(ArgAction::SetTrue).requires("foo"))
        .arg(Arg::new("foo").short('f').action(ArgAction::SetTrue))
        .group(ArgGroup::new("group").args(["one", "two"]));

    // This should fail because --foo is required when either -1 or -2 is present
    let result = cmd.try_get_matches_from(vec!["test", "-1"]);
    
    // Verify the validation works correctly (issue is fixed)
    assert!(result.is_err(), "Should fail because -1 requires foo but foo is missing");
    assert_eq!(result.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn issue_4707_mutually_exclusive_group_bypasses_requires() {
    // Test with explicitly mutually exclusive group
    let cmd = Command::new("test")
        .arg(Arg::new("one").short('1').action(ArgAction::SetTrue).requires("foo"))
        .arg(Arg::new("two").short('2').action(ArgAction::SetTrue).requires("foo"))
        .arg(Arg::new("foo").short('f').action(ArgAction::SetTrue))
        .group(ArgGroup::new("group").args(["one", "two"]).multiple(false)); // explicit mutually exclusive

    // This should fail because --foo is required when either -1 or -2 is present
    let result = cmd.try_get_matches_from(vec!["test", "-1"]);
    
    assert!(result.is_err(), "Should fail because -1 requires foo but foo is missing");
    assert_eq!(result.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn issue_4707_requires_should_work_when_required_arg_provided() {
    let cmd = Command::new("test")
        .arg(Arg::new("one").short('1').action(ArgAction::SetTrue).requires("foo"))
        .arg(Arg::new("two").short('2').action(ArgAction::SetTrue).requires("foo"))
        .arg(Arg::new("foo").short('f').action(ArgAction::SetTrue))
        .group(ArgGroup::new("group").args(["one", "two"]));

    // This should succeed because --foo is provided
    let result = cmd.try_get_matches_from(vec!["test", "-1", "-f"]);
    
    assert!(result.is_ok(), "Should have succeeded when required argument is provided");
}

#[test]
fn issue_4707_group_requires_validation() {
    // Test with group that has 'requires' on the group itself
    let cmd = Command::new("test")
        .arg(Arg::new("one").short('1').action(ArgAction::SetTrue))
        .arg(Arg::new("two").short('2').action(ArgAction::SetTrue)) 
        .arg(Arg::new("foo").short('f').action(ArgAction::SetTrue))
        .group(ArgGroup::new("group").args(["one", "two"]).requires("foo"));

    // This should fail because group requires 'foo'
    let result = cmd.try_get_matches_from(vec!["test", "-1"]);
    
    assert!(result.is_err(), "Should fail because group requires foo");
    assert_eq!(result.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn issue_4707_complex_interaction_test() {
    // Test complex interactions between mutually exclusive groups and requires
    let cmd = Command::new("test")
        .arg(Arg::new("verbose").short('v').action(ArgAction::SetTrue).requires("output"))
        .arg(Arg::new("quiet").short('q').action(ArgAction::SetTrue).requires("output"))
        .arg(Arg::new("output").short('o').action(ArgAction::SetTrue))
        .group(ArgGroup::new("verbosity").args(["verbose", "quiet"]).multiple(false));
    
    // Test case 1: One argument from group without its required dependency
    let result1 = cmd.clone().try_get_matches_from(vec!["test", "-v"]);
    assert!(result1.is_err(), "Should fail because -v requires output");
    assert_eq!(result1.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
    
    // Test case 2: Other argument from group without its required dependency  
    let result2 = cmd.clone().try_get_matches_from(vec!["test", "-q"]);
    assert!(result2.is_err(), "Should fail because -q requires output");
    assert_eq!(result2.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
    
    // Test case 3: Valid usage with dependency
    let result3 = cmd.clone().try_get_matches_from(vec!["test", "-v", "-o"]);
    assert!(result3.is_ok(), "Should succeed when dependency is provided");
}

#[test]
fn issue_4707_exact_github_example_builder() {
    // This reproduces the exact case from the GitHub issue using clap builder instead of derive
    let cmd = Command::new("test")
        .arg(Arg::new("read").long("read").action(ArgAction::SetTrue).group("command"))
        .arg(Arg::new("write").long("write").action(ArgAction::SetTrue).group("command"))
        .arg(Arg::new("show_hex").long("show-hex").action(ArgAction::SetTrue).requires("read"))
        .group(ArgGroup::new("command").multiple(false));
    
    // This exact case should fail: --write --show-hex
    // Because --show-hex requires --read, but --read and --write are mutually exclusive
    let result = cmd.try_get_matches_from(["test", "--write", "--show-hex"]);
    
    assert!(result.is_err(), "Should fail because --show-hex requires --read but --write and --read are mutually exclusive");
    assert_eq!(result.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}
