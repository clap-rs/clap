#![allow(clippy::bool_assert_comparison)]
#![allow(clippy::redundant_clone)]
#![cfg(feature = "help")]
#![cfg(feature = "usage")]

mod action;
mod app_settings;
mod arg_aliases;
mod arg_aliases_short;
mod arg_matches;
mod borrowed;
mod cargo;
mod command;
mod conflicts;
mod default_missing_vals;
mod default_vals;
mod delimiters;
mod derive_order;
mod display_order;
mod double_require;
mod empty_values;
mod env;
mod error;
mod flag_subcommands;
mod flags;
mod global_args;
mod groups;
mod help;
mod help_env;
mod hidden_args;
mod ignore_errors;
mod indices;
mod multiple_occurrences;
mod multiple_values;
mod occurrences;
mod opts;
mod positionals;
mod posix_compatible;
mod possible_values;
mod propagate_globals;
mod require;
mod subcommands;
mod template_help;
mod tests;
mod unicode;
mod unique_args;
mod utf16;
mod utf8;
mod utils;
mod version;
