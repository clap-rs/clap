// Used to simulate a fairly large number of subcommands
//
// CLI used is from rustup 408ed84f0e50511ed44a405dd91365e5da588790

use clap::{App, AppSettings, Arg, ArgGroup, ArgSettings};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn build_rustup(c: &mut Criterion) {
    c.bench_function("build_rustup", |b| b.iter(build_cli));
}

pub fn parse_rustup(c: &mut Criterion) {
    c.bench_function("parse_rustup", |b| {
        b.iter(|| build_cli().get_matches_from(vec![""]))
    });
}

pub fn parse_rustup_with_sc(c: &mut Criterion) {
    c.bench_function("parse_rustup_with_sc", |b| {
        b.iter(|| build_cli().get_matches_from(vec!["rustup override add stable"]))
    });
}

fn build_cli() -> App<'static> {
    App::new("rustup")
        .version("0.9.0") // Simulating
        .about("The Rust toolchain installer")
        .after_help(RUSTUP_HELP)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::DeriveDisplayOrder)
        // .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("verbose")
                .about("Enable verbose output")
                .short('v')
                .long("verbose"),
        )
        .subcommand(
            App::new("show")
                .about("Show the active and installed toolchains")
                .after_help(SHOW_HELP),
        )
        .subcommand(
            App::new("install")
                .about("Update Rust toolchains")
                .after_help(TOOLCHAIN_INSTALL_HELP)
                .setting(AppSettings::Hidden) // synonym for 'toolchain install'
                .arg(Arg::with_name("toolchain").setting(ArgSettings::Required)),
        )
        .subcommand(
            App::new("update")
                .about("Update Rust toolchains")
                .after_help(UPDATE_HELP)
                .arg(Arg::with_name("toolchain").setting(ArgSettings::Required))
                .arg(
                    Arg::with_name("no-self-update")
                        .about("Don't perform self update when running the `rustup` command")
                        .long("no-self-update")
                        .setting(ArgSettings::Hidden),
                ),
        )
        .subcommand(
            App::new("default")
                .about("Set the default toolchain")
                .after_help(DEFAULT_HELP)
                .arg(Arg::with_name("toolchain").setting(ArgSettings::Required)),
        )
        .subcommand(
            App::new("toolchain")
                .about("Modify or query the installed toolchains")
                .after_help(TOOLCHAIN_HELP)
                .setting(AppSettings::DeriveDisplayOrder)
                // .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(App::new("list").about("List installed toolchains"))
                .subcommand(
                    App::new("install")
                        .about("Install or update a given toolchain")
                        .arg(Arg::with_name("toolchain").setting(ArgSettings::Required)),
                )
                .subcommand(
                    App::new("uninstall")
                        .about("Uninstall a toolchain")
                        .arg(Arg::with_name("toolchain").setting(ArgSettings::Required)),
                )
                .subcommand(
                    App::new("link")
                        .about("Create a custom toolchain by symlinking to a directory")
                        .arg(Arg::with_name("toolchain").setting(ArgSettings::Required))
                        .arg(Arg::with_name("path").setting(ArgSettings::Required)),
                )
                .subcommand(
                    App::new("update")
                        .setting(AppSettings::Hidden) // synonym for 'install'
                        .arg(Arg::with_name("toolchain").setting(ArgSettings::Required)),
                )
                .subcommand(
                    App::new("add")
                        .setting(AppSettings::Hidden) // synonym for 'install'
                        .arg(Arg::with_name("toolchain").setting(ArgSettings::Required)),
                )
                .subcommand(
                    App::new("remove")
                        .setting(AppSettings::Hidden) // synonym for 'uninstall'
                        .arg(Arg::with_name("toolchain").setting(ArgSettings::Required)),
                ),
        )
        .subcommand(
            App::new("target")
                .about("Modify a toolchain's supported targets")
                .setting(AppSettings::VersionlessSubcommands)
                .setting(AppSettings::DeriveDisplayOrder)
                // .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("list")
                        .about("List installed and available targets")
                        .arg(
                            Arg::with_name("toolchain")
                                .long("toolchain")
                                .setting(ArgSettings::TakesValue),
                        ),
                )
                .subcommand(
                    App::new("add")
                        .about("Add a target to a Rust toolchain")
                        .arg(Arg::with_name("target").setting(ArgSettings::Required))
                        .arg(
                            Arg::with_name("toolchain")
                                .long("toolchain")
                                .setting(ArgSettings::TakesValue),
                        ),
                )
                .subcommand(
                    App::new("remove")
                        .about("Remove a target  from a Rust toolchain")
                        .arg(Arg::with_name("target").setting(ArgSettings::Required))
                        .arg(
                            Arg::with_name("toolchain")
                                .long("toolchain")
                                .setting(ArgSettings::TakesValue),
                        ),
                )
                .subcommand(
                    App::new("install")
                        .setting(AppSettings::Hidden) // synonym for 'add'
                        .arg(Arg::with_name("target").setting(ArgSettings::Required))
                        .arg(
                            Arg::with_name("toolchain")
                                .long("toolchain")
                                .setting(ArgSettings::TakesValue),
                        ),
                )
                .subcommand(
                    App::new("uninstall")
                        .setting(AppSettings::Hidden) // synonym for 'remove'
                        .arg(Arg::with_name("target").setting(ArgSettings::Required))
                        .arg(
                            Arg::with_name("toolchain")
                                .long("toolchain")
                                .setting(ArgSettings::TakesValue),
                        ),
                ),
        )
        .subcommand(
            App::new("component")
                .about("Modify a toolchain's installed components")
                .setting(AppSettings::VersionlessSubcommands)
                .setting(AppSettings::DeriveDisplayOrder)
                // .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("list")
                        .about("List installed and available components")
                        .arg(
                            Arg::with_name("toolchain")
                                .long("toolchain")
                                .setting(ArgSettings::TakesValue),
                        ),
                )
                .subcommand(
                    App::new("add")
                        .about("Add a component to a Rust toolchain")
                        .arg(Arg::with_name("component").setting(ArgSettings::Required))
                        .arg(
                            Arg::with_name("toolchain")
                                .long("toolchain")
                                .setting(ArgSettings::TakesValue),
                        )
                        .arg(
                            Arg::with_name("target")
                                .long("target")
                                .setting(ArgSettings::TakesValue),
                        ),
                )
                .subcommand(
                    App::new("remove")
                        .about("Remove a component from a Rust toolchain")
                        .arg(Arg::with_name("component").setting(ArgSettings::Required))
                        .arg(
                            Arg::with_name("toolchain")
                                .long("toolchain")
                                .setting(ArgSettings::TakesValue),
                        )
                        .arg(
                            Arg::with_name("target")
                                .long("target")
                                .setting(ArgSettings::TakesValue),
                        ),
                ),
        )
        .subcommand(
            App::new("override")
                .about("Modify directory toolchain overrides")
                .after_help(OVERRIDE_HELP)
                .setting(AppSettings::VersionlessSubcommands)
                .setting(AppSettings::DeriveDisplayOrder)
                // .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(App::new("list").about("List directory toolchain overrides"))
                .subcommand(
                    App::new("set")
                        .about("Set the override toolchain for a directory")
                        .arg(Arg::with_name("toolchain").setting(ArgSettings::Required)),
                )
                .subcommand(
                    App::new("unset")
                        .about("Remove the override toolchain for a directory")
                        .after_help(OVERRIDE_UNSET_HELP)
                        .arg(
                            Arg::with_name("path")
                                .long("path")
                                .setting(ArgSettings::TakesValue)
                                .about("Path to the directory"),
                        )
                        .arg(
                            Arg::with_name("nonexistent")
                                .long("nonexistent")
                                .about("Remove override toolchain for all nonexistent directories"),
                        ),
                )
                .subcommand(
                    App::new("add")
                        .setting(AppSettings::Hidden) // synonym for 'set'
                        .arg(Arg::with_name("toolchain").setting(ArgSettings::Required)),
                )
                .subcommand(
                    App::new("remove")
                        .setting(AppSettings::Hidden) // synonym for 'unset'
                        .about("Remove the override toolchain for a directory")
                        .arg(
                            Arg::with_name("path")
                                .long("path")
                                .setting(ArgSettings::TakesValue),
                        )
                        .arg(
                            Arg::with_name("nonexistent")
                                .long("nonexistent")
                                .about("Remove override toolchain for all nonexistent directories"),
                        ),
                ),
        )
        .subcommand(
            App::new("run")
                .about("Run a command with an environment configured for a given toolchain")
                .after_help(RUN_HELP)
                .setting(AppSettings::TrailingVarArg)
                .arg(Arg::with_name("toolchain").setting(ArgSettings::Required))
                .arg(Arg::with_name("command").settings(&[
                    ArgSettings::Required,
                    ArgSettings::MultipleValues,
                    ArgSettings::MultipleOccurrences,
                ])),
        )
        .subcommand(
            App::new("which")
                .about("Display which binary will be run for a given command")
                .arg(Arg::with_name("command").setting(ArgSettings::Required)),
        )
        .subcommand(
            App::new("doc")
                .about("Open the documentation for the current toolchain")
                .after_help(DOC_HELP)
                .arg(
                    Arg::with_name("book")
                        .long("book")
                        .about("The Rust Programming Language book"),
                )
                .arg(
                    Arg::with_name("std")
                        .long("std")
                        .about("Standard library API documentation"),
                )
                .group(ArgGroup::with_name("page").args(&["book", "std"])),
        )
        .subcommand(
            App::new("man")
                .about("View the man page for a given command")
                .arg(Arg::with_name("command").setting(ArgSettings::Required))
                .arg(
                    Arg::with_name("toolchain")
                        .long("toolchain")
                        .setting(ArgSettings::TakesValue),
                ),
        )
        .subcommand(
            App::new("self")
                .about("Modify the rustup installation")
                .setting(AppSettings::VersionlessSubcommands)
                .setting(AppSettings::DeriveDisplayOrder)
                .subcommand(App::new("update").about("Download and install updates to rustup"))
                .subcommand(
                    App::new("uninstall")
                        .about("Uninstall rustup.")
                        .arg(Arg::with_name("no-prompt").short('y')),
                )
                .subcommand(App::new("upgrade-data").about("Upgrade the internal data format.")),
        )
        .subcommand(
            App::new("telemetry")
                .about("rustup telemetry commands")
                .setting(AppSettings::Hidden)
                .setting(AppSettings::VersionlessSubcommands)
                .setting(AppSettings::DeriveDisplayOrder)
                .subcommand(App::new("enable").about("Enable rustup telemetry"))
                .subcommand(App::new("disable").about("Disable rustup telemetry"))
                .subcommand(App::new("analyze").about("Analyze stored telemetry")),
        )
        .subcommand(
            App::new("set").about("Alter rustup settings").subcommand(
                App::new("default-host")
                    .about("The triple used to identify toolchains when not specified")
                    .arg(Arg::with_name("host_triple").setting(ArgSettings::Required)),
            ),
        )
}

static RUSTUP_HELP: &str = r"
rustup installs The Rust Programming Language from the official
release channels, enabling you to easily switch between stable, beta,
and nightly compilers and keep them updated. It makes cross-compiling
simpler with binary builds of the standard library for common platforms.

If you are new to Rust consider running `rustup doc --book`
to learn Rust.";

static SHOW_HELP: &str = r"
Shows the name of the active toolchain and the version of `rustc`.

If the active toolchain has installed support for additional
compilation targets, then they are listed as well.

If there are multiple toolchains installed then all installed
toolchains are listed as well.";

static UPDATE_HELP: &str = r"
With no toolchain specified, the `update` command updates each of the
installed toolchains from the official release channels, then updates
rustup itself.

If given a toolchain argument then `update` updates that toolchain,
the same as `rustup toolchain install`.

'toolchain' specifies a toolchain name, such as 'stable', 'nightly',
or '1.8.0'. For more information see `rustup help toolchain`.";

static TOOLCHAIN_INSTALL_HELP: &str = r"
Installs a specific rust toolchain.

The 'install' command is an alias for 'rustup update <toolchain>'.

'toolchain' specifies a toolchain name, such as 'stable', 'nightly',
or '1.8.0'. For more information see `rustup help toolchain`.";

static DEFAULT_HELP: &str = r"
Sets the default toolchain to the one specified. If the toolchain is
not already installed then it is installed first.";

static TOOLCHAIN_HELP: &str = r"
Many `rustup` commands deal with *toolchains*, a single installation
of the Rust compiler. `rustup` supports multiple types of
toolchains. The most basic track the official release channels:
'stable', 'beta' and 'nightly'; but `rustup` can also install
toolchains from the official archives, for alternate host platforms,
and from local builds.

Standard release channel toolchain names have the following form:

    <channel>[-<date>][-<host>]

    <channel>       = stable|beta|nightly|<version>
    <date>          = YYYY-MM-DD
    <host>          = <target-triple>

'channel' is either a named release channel or an explicit version
number, such as '1.8.0'. Channel names can be optionally appended with
an archive date, as in 'nightly-2014-12-18', in which case the
toolchain is downloaded from the archive for that date.

Finally, the host may be specified as a target triple. This is most
useful for installing a 32-bit compiler on a 64-bit platform, or for
installing the [MSVC-based toolchain] on Windows. For example:

    rustup toolchain install stable-x86_64-pc-windows-msvc

For convenience, elements of the target triple that are omitted will be
inferred, so the above could be written:

    $ rustup default stable-msvc

Toolchain names that don't name a channel instead can be used to name
custom toolchains with the `rustup toolchain link` command.";

static OVERRIDE_HELP: &str = r"
Overrides configure rustup to use a specific toolchain when
running in a specific directory.

Directories can be assigned their own Rust toolchain with
`rustup override`. When a directory has an override then
any time `rustc` or `cargo` is run inside that directory,
or one of its child directories, the override toolchain
will be invoked.

To pin to a specific nightly:

    rustup override set nightly-2014-12-18

Or a specific stable release:

    rustup override set 1.0.0

To see the active toolchain use `rustup show`. To remove the override
and use the default toolchain again, `rustup override unset`.";

static OVERRIDE_UNSET_HELP: &str = r"
If `--path` argument is present, removes the override toolchain for
the specified directory. If `--nonexistent` argument is present, removes
the override toolchain for all nonexistent directories. Otherwise,
removes the override toolchain for the current directory.";

static RUN_HELP: &str = r"
Configures an environment to use the given toolchain and then runs
the specified program. The command may be any program, not just
rustc or cargo. This can be used for testing arbitrary toolchains
without setting an override.

Commands explicitly proxied by `rustup` (such as `rustc` and `cargo`)
also have a shorthand for this available. The toolchain can be set by
using `+toolchain` as the first argument. These are equivalent:

    cargo +nightly build

    rustup run nightly cargo build";

static DOC_HELP: &str = r"
Opens the documentation for the currently active toolchain with the
default browser.

By default, it opens the documentation index. Use the various flags to
open specific pieces of documentation.";

criterion_group!(benches, build_rustup, parse_rustup, parse_rustup_with_sc);

criterion_main!(benches);
