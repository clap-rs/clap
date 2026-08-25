// This example shows what a `#[derive(Parser)]` struct expands to in
// equivalent builder code.  It is referenced from the Derive Reference
// under "What Does a Derive Expand To?".
//
// Each derive struct is followed by a `fn` showing the builder equivalent.
// Comments link each derive attribute to its builder method.

#![allow(dead_code)]

use clap::{Arg, ArgAction, ArgGroup, Args, Command, Parser, Subcommand, ValueEnum, value_parser};

// ── 1. Command-level attributes ────────────────────────────────────────

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = Format::Text)]
    format: Format,

    /// Optional config file to read from
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,

    #[command(flatten)]
    verbosity: Verbosity,

    #[command(subcommand)]
    command: Option<Commands>,
}

// The derive above generates roughly this builder:
fn cli_as_builder() -> Command {
    // `#[command(version, about, long_about = None)]`
    let m = Command::new("cli")
        .version("0.1.0")
        // `/// Simple program to greet a person` → `.about(...)`
        .about("Simple program to greet a person")
        .long_about(None);

    // `#[command(flatten)] verbosity: Verbosity`
    // → args from the `Verbosity` struct are inlined here
    let m = m
        // `#[arg(short, long, conflicts_with = "verbose")]`
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .conflicts_with("verbose"),
        )
        // `#[arg(short, long, action = Count)] verbose: u8`
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::Count),
        )
        // `#[arg(long, value_name = "LEVEL", requires = "verbose")] level: Option<u8>`
        .arg(
            Arg::new("level")
                .long("level")
                .value_name("LEVEL")
                .requires("verbose"),
        )
        // `#[arg(long, num_args = 0..=1, require_equals = true, ...)]`
        .arg(
            Arg::new("log_mode")
                .long("log-mode")
                .num_args(0..=1)
                .require_equals(true)
                .value_name("MODE")
                .requires("verbose"),
        )
        // `conflicts_with` creates an implicit ArgGroup
        .group(ArgGroup::new("verbosity").args(["quiet", "verbose", "level", "log_mode"]));

    // `#[arg(short, long)] name: String` → `.short('n').long("name").required(true)`
    let m = m.arg(
        Arg::new("name")
            .short('n')
            .long("name")
            .required(true),
    );

    // `#[arg(short, long, default_value_t = 1)] count: u8`
    // → `.short('c').long("count").default_value("1")`
    let m = m.arg(
        Arg::new("count")
            .short('c')
            .long("count")
            .default_value("1"),
    );

    // `#[arg(short, long, value_enum, default_value_t = Format::Text)]`
    // → `.short('f').long("format").value_parser(...).default_value("text")`
    let m = m.arg(
        Arg::new("format")
            .short('f')
            .long("format")
            .value_parser(value_parser!(Format))
            .default_value("text"),
    );

    // `#[command(subcommand)] command: Option<Commands>`
    // → `.subcommand(...)` for each variant
    let config_cmd = Command::new("config")
        .about("Print detailed configuration")
        .arg(
            Arg::new("show_defaults")
                .long("show-defaults")
                .action(ArgAction::SetTrue),
        );

    let greet_cmd = Command::new("greet")
        .about("Run the greeting service")
        .arg(
            Arg::new("name")
                .required(true),
        );

    m.subcommand(config_cmd).subcommand(greet_cmd)
}

// ── 2. Args struct (flattened) ───────────────────────────────────────────

#[derive(Args, Debug)]
struct Verbosity {
    /// Silence all output
    #[arg(short, long, conflicts_with = "verbose")]
    quiet: bool,

    /// Increase verbosity (can be specified multiple times)
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,

    /// Force a specific verbosity level
    #[arg(long, value_name = "LEVEL", requires = "verbose")]
    level: Option<u8>,

    #[arg(long, requires = "verbose", num_args = 0..=1, require_equals = true, value_name = "MODE")]
    log_mode: Option<String>,
}

// ── 3. ValueEnum (possible values) ──────────────────────────────────────

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
enum Format {
    /// Human-readable text output
    Text,
    /// Machine-readable JSON output
    Json,
    /// Compact single-line output
    Compact,
}

// The derive above generates PossibleValue entries for each variant.
// In the builder API that's equivalent to:
fn format_value_parser() {
    let _ = value_parser!(Format);

    // Under the hood, `ValueEnum::value_variants()` exposes the variants,
    // and each variant's doc comment becomes the help text for a
    // `PossibleValue`.  The builder API mirrors this with:
    //
    //   Arg::new("format")
    //       .value_parser(["text", "json", "compact"])
    //       .value_parser("text", "Human-readable text output")
    //       .value_parser("json", "Machine-readable JSON output")
    //       .value_parser("compact", "Compact single-line output")
}

// ── 4. Subcommand enum ──────────────────────────────────────────────────

#[derive(Subcommand, Debug)]
enum Commands {
    /// Print detailed configuration
    Config {
        /// Show defaults
        #[arg(long)]
        show_defaults: bool,
    },

    /// Run the greeting service
    Greet {
        /// Name of the person to greet
        name: String,
    },
}

// Each variant becomes a subcommand:
//   `config` → `Command::new("config").about(...).arg(...)`
//   `greet`   → `Command::new("greet").about(...).arg(...)`
// The doc comment becomes `.about(...)`.

fn main() {
    let _ = cli_as_builder();
    println!("Run with --help to see the generated CLI");
}
