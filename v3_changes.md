# Highlights

* Lazy propagation
* Lazy requirement validation
* `App::write_help` takes `&mut self` now
* `App::override_usage` No longer implies `\t` which allows multi lined usages
* `Arg::setting` instead of `bool` methods
* In usage parser, for options `[name]... --option [val]` results in `ArgSettings::MultipleOccurrences` but `--option [val]...` results in `ArgSettings::MultipleValues` *and* `ArgSettings::MultipleOccurrences`. Before both resulted in the same thing
* Allow empty values no longer default
* UseValueDelimiter no longer the default
* Multpiple delima fixed (vals vs occurrences)

# Deprecations

## Simple Renames

- `App::usage` -> `App::override_usage` 
- `App::help` -> `App::override_help`
- `App::template` -> `App::help_template`
- `App::get_matches_safe` -> `App::try_get_matches` 
- `App::get_matches_from_safe` -> `App::try_get_matches_from` 
- `App::get_matches_safe_borrow` -> `App::try_get_matches_from_mut` 
- `Arg::unset` -> `Arg::unset_setting` 
- `Arg::set` -> `Arg::setting` 


## Arg Bool Methods

- `Arg::last` -> `ArgSettings::Last`
- `Arg::required` -> `ArgSettings::Required`
- `Arg::require_equals` -> `ArgSettings::RequireEquals`
- `Arg::allow_hyphen_values` -> `ArgSettings::AllowHyphenValues`
- `Arg::takes_value` -> `ArgSettings::TakesValue`
- `Arg::hide_possible_values` -> `ArgSettings::HidePossibleValues`
- `Arg::hide_default_value` -> `ArgSettings::HideDefaultValue`
- `Arg::multiple` -> `ArgSettings::MultipleValues` 
- `Arg::multiple` -> `ArgSettings::MultipleOccurrences` 
- `Arg::multiple` -> `ArgSettings::Multiple` 
- `Arg::global` -> `ArgSettings::Global`
- `Arg::empty_values` -> `ArgSettings::AllowEmptyValues`
- `Arg::hidden` -> `ArgSettings::Hidden`
- `Arg::case_insensitive` -> `ArgSettings::IgnoreCase`
- `Arg::use_delimiter` -> `ArgSettings::UseDelimiter`
- `Arg::require_delimiter` -> `ArgSettings::RequireDelimiter`
- `Arg::hide_env_values` -> `ArgSettings::HideEnvValues`
- `Arg::next_line_help` -> `ArgSettings::NextLineHelp`

- `App::version_message` -> `App::mut_arg`
- `App::version_short` -> `App::mut_arg`
- `App::help_message` -> `App::mut_arg`
- `App::help_short` -> `App::mut_arg`
- `App::args_from_usage` -> `App::args(&str)`
- `App::arg_from_usage` -> `App::arg(Arg::from)`
- `App::write_help` -> `&self` -> `&mut self` (#808)
- `App::gen_completions` -> `clap_completions::generate`
- `App::gen_completions_to` -> `clap_completions::generate_to`
- `Arg::from_usage` -> `Arg::from(&str)` 

# Additions

