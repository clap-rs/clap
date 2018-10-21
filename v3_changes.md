# Highlights

* Lazy propagation
* Lazy requirement validation
* `App::write_help` takes `&mut self` now
* `App::override_usage` No longer implies `\t` which allows multi lined usages
* In usage parser, for options `[name]... --option [val]` results in `ArgSettings::MultipleOccurrences` but `--option [val]...` results in `ArgSettings::MultipleValues` *and* `ArgSettings::MultipleOccurrences`. Before both resulted in the same thing
* Allow empty values no longer default
* UseValueDelimiter no longer the default
* Multiple delima fixed (vals vs occurrences)
* Ability to mutate args once they've been added to an `App`
* `App::args` and `App::arg` are more generic
* Can unset global settings
* Instead of adding arg with long `--help` or `--version` you can use `App::mut_arg` to override things
  * Caution, must fully override
  * No longer forces auto-handle of help/ver however if still desired `AppSettings::NoAuto{Help,Version}`

# How to Upgrade

### If you use `Arg::multiple(true)`


# Deprecations

## Simple Renames

### App

- `App::get_matches_safe` -> `App::try_get_matches` 
- `App::get_matches_from_safe` -> `App::try_get_matches_from` 
- `App::get_matches_safe_borrow` -> `App::try_get_matches_from_mut` 
- `App::usage` -> `App::override_usage` 
- `App::help` -> `App::override_help`
- `App::template` -> `App::help_template`

### Arg

- `Arg::unset` -> `Arg::unset_setting` 
- `Arg::set` -> `Arg::setting` 


## Structural Changes

### App

- `App::version_message` -> `App::mut_arg`
- `App::version_short` -> `App::mut_arg`
- `App::help_message` -> `App::mut_arg`
- `App::help_short` -> `App::mut_arg`
- `App::args_from_usage` -> `App::args(&str)`
- `App::arg_from_usage` -> `App::arg(&str)`
- `App::write_help` -> `&self` -> `&mut self` (#808)
- `App::gen_completions` -> `clap_completions::generate`
- `App::gen_completions_to` -> `clap_completions::generate_to`
- `App::settings` -> `App::setting(Setting1 | Setting2)`
- `App::unset_settings` -> `App::unset_setting(Setting1 | Setting2)`
- `App::global_settings` -> `App::global_setting(Setting1 | Setting2)`

### Arg

- `Arg::from_usage` -> `Arg::from(&str)` 

# Additional APIs

## App

* `App::mut_arg`
* `App::unset_global_setting`

## Arg

