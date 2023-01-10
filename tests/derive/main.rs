#![cfg(feature = "derive")]
#![cfg(feature = "help")]
#![cfg(feature = "usage")]

mod app_name;
mod arguments;
mod author_version_about;
mod basic;
mod boxed;
mod custom_string_parsers;
mod default_value;
mod deny_warnings;
mod doc_comments_help;
mod explicit_name_no_renaming;
mod flags;
mod flatten;
mod generic;
mod groups;
mod help;
mod issues;
mod macros;
mod naming;
mod nested_subcommands;
mod non_literal_attributes;
mod occurrences;
mod options;
mod privacy;
mod raw_bool_literal;
mod raw_idents;
mod rename_all_env;
mod skip;
mod subcommands;
mod type_alias_regressions;
mod utf8;
mod utils;
mod value_enum;
