#[allow(unused_imports)]
use clap_complete::env::EnvCompleter;
#[allow(unused_imports)]
use clap_complete::env::Powershell;
#[allow(unused_imports)]
use snapbox::IntoData as _;
#[allow(unused_imports)]
use snapbox::assert_data_eq;

use crate::common;

#[test]
fn basic() {
    let name = "my-app";
    let cmd = common::basic_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/basic.ps1"],
        clap_complete::shells::PowerShell,
        cmd,
        name,
    );
}

#[test]
fn feature_sample() {
    let name = "my-app";
    let cmd = common::feature_sample_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/feature_sample.ps1"],
        clap_complete::shells::PowerShell,
        cmd,
        name,
    );
}

#[test]
fn special_commands() {
    let name = "my-app";
    let cmd = common::special_commands_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/special_commands.ps1"],
        clap_complete::shells::PowerShell,
        cmd,
        name,
    );
}

#[test]
fn quoting() {
    let name = "my-app";
    let cmd = common::quoting_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/quoting.ps1"],
        clap_complete::shells::PowerShell,
        cmd,
        name,
    );
}

#[test]
fn aliases() {
    let name = "my-app";
    let cmd = common::aliases_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/aliases.ps1"],
        clap_complete::shells::PowerShell,
        cmd,
        name,
    );
}

#[test]
fn sub_subcommands() {
    let name = "my-app";
    let cmd = common::sub_subcommands_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/sub_subcommands.ps1"],
        clap_complete::shells::PowerShell,
        cmd,
        name,
    );
}

#[test]
fn external_subcommands() {
    let name = "my-app";
    let cmd = common::external_subcommand(name);
    common::assert_matches(
        snapbox::file!["../snapshots/external_subcommands.ps1"],
        clap_complete::shells::PowerShell,
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
        snapbox::file!["../snapshots/custom_bin_name.ps1"],
        clap_complete::shells::PowerShell,
        cmd,
        bin_name,
    );
}

#[test]
fn value_hint() {
    let name = "my-app";
    let cmd = common::value_hint_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/value_hint.ps1"],
        clap_complete::shells::PowerShell,
        cmd,
        name,
    );
}

#[test]
fn value_terminator() {
    let name = "my-app";
    let cmd = common::value_terminator_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/value_terminator.ps1"],
        clap_complete::shells::PowerShell,
        cmd,
        name,
    );
}

#[test]
fn multi_value_option() {
    let name = "my-app";
    let cmd = common::multi_value_option_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/multi_value_option.ps1"],
        clap_complete::shells::PowerShell,
        cmd,
        name,
    );
}

#[test]
fn optional_value_option() {
    let name = "my-app";
    let cmd = common::optional_value_option_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/optional_value_option.ps1"],
        clap_complete::shells::PowerShell,
        cmd,
        name,
    );
}

#[test]
fn optional_multi_value_option() {
    let name = "my-app";
    let cmd = common::optional_multi_value_option_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/optional_multi_value_option.ps1"],
        clap_complete::shells::PowerShell,
        cmd,
        name,
    );
}

#[test]
fn two_multi_valued_arguments() {
    let name = "my-app";
    let cmd = common::two_multi_valued_arguments_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/two_multi_valued_arguments.ps1"],
        clap_complete::shells::PowerShell,
        cmd,
        name,
    );
}

#[test]
fn subcommand_last() {
    let name = "my-app";
    let cmd = common::subcommand_last(name);
    common::assert_matches(
        snapbox::file!["../snapshots/subcommand_last.ps1"],
        clap_complete::shells::PowerShell,
        cmd,
        name,
    );
}

// Snapshot of the generated PowerShell registration script, not a test
// of completion behaviour: it pins the `""` empty-word handling under
// legacy native argument passing (see #6548).
    #[test]
    #[cfg(feature = "unstable-dynamic")]
    fn powershell_env_registration_snapshot() {
        let mut buf = Vec::new();
        Powershell
            .write_registration("V", "n", "/ignored/bin", "/p/completer", &mut buf)
            .expect("write_registration failed");
        let script = String::from_utf8(buf).expect("Invalid UTF-8");
        assert_data_eq!(
            script.trim(),
            snapbox::str![[r#"
Register-ArgumentCompleter -Native -CommandName /ignored/bin -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $prev = $env:V;
    $env:V = "powershell";

    $args = $commandAst.Extent.Text
    $args = $args.Substring(0, [math]::Min($cursorPosition, $args.Length));
    if ($wordToComplete -eq "") {
        if ($null -eq $PSNativeCommandArgumentPassing -or $PSNativeCommandArgumentPassing -eq 'Legacy') {
            $args += ' ''""''';
        } else {
            $args += " ''";
        }
    }

    $results = Invoke-Expression @"
& /p/completer -- $args
"@;
    if ($null -eq $prev) {
        Remove-Item Env:\V;
    } else {
        $env:V = $prev;
    }
    $results | ForEach-Object {
        $split = $_.Split("`t");
        $cmd = $split[0];

        if ($split.Length -eq 2) {
            $help = $split[1];
        }
        else {
            $help = $split[0];
        }

        [System.Management.Automation.CompletionResult]::new($cmd, $cmd, 'ParameterValue', $help)
    }
};
"#]]
            .raw()
        );
    }
