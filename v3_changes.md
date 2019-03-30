# Highlights

* `App` vs `Cmd` (No more `SubCommand`)
* `AppSettings` vs `CmdSettings`
* Lazy propagation
* Lazy requirement validation
* `Cmd::write_help` takes `&mut self` now
* `Cmd::override_usage` No longer implies `\t` which allows multi lined usages
* In usage parser, for options `[name]... --option [val]` results in `ArgSettings::MultipleOccurrences` but `--option [val]...` results in `ArgSettings::MultipleValues` *and* `ArgSettings::MultipleOccurrences`. Before both resulted in the same thing
* Allow empty values no longer default
* UseValueDelimiter no longer the default
* Multiple delima fixed (vals vs occurrences)
* Ability to mutate args once they've been added to an `Cmd`
* `Cmd::args` and `Cmd::arg` are more generic
* Can unset global settings
* Instead of adding arg with long `--help` or `--version` you can use `Cmd::mut_arg` to override things
  * Caution, must fully override
  * No longer forces auto-handle of help/ver however if still desired `CmdSettings::NoAuto{Help,Version}`

# How to Upgrade

### If you use `Arg::multiple(true)`

# Deprecations

## Simple Renames

### App

- `App::get_matches` -> `App::parse` 
- `App::get_matches_safe` -> `App::try_parse` 
- `App::get_matches_from_safe` -> `App::try_parse_from` 
- `App::get_matches_safe_borrow` -> `App::try_parse_from_mut` 
- `App::usage` -> `Cmd::override_usage` 
- `App::help` -> `Cmd::override_help`
- `App::template` -> `Cmd::help_template`

### Arg

- `Arg::unset` -> `Arg::unset_setting` 
- `Arg::set` -> `Arg::setting` 
- `Arg::from_yaml` -> `Arg::from`
- `Arg::with_name` -> `Arg::new`
- `Arg::group` -> Use Cmd::group
- `Arg::groups` -> Use Cmd::group

### ArgGroup

- `ArgGroup::with_name` -> `ArgGroup::new`

## Structural Changes

### App

- `App::version_message` -> `Cmd::mut_arg`
- `App::version_short` -> `Cmd::mut_arg`
- `App::help_message` -> `Cmd::mut_arg`
- `App::help_short` -> `Cmd::mut_arg`
- `App::args_from_usage` -> `Cmd::args(&str)`
- `App::arg_from_usage` -> `Cmd::arg(&str)`
- `Cmd::write_help` -> `&self` -> `&mut self` (#808)
- `App::gen_completions` -> `clap_completions::generate`
- `App::gen_completions_to` -> `clap_completions::generate_to`

#### Not Done Yet

- `App::settings` -> `Cmd::setting(Setting1 | Setting2)`
- `App::unset_settings` -> `Cmd::unset_setting(Setting1 | Setting2)`
- `App::global_settings` -> `Cmd::global_setting(Setting1 | Setting2)`

### Arg

- `Arg::from_usage` -> `Arg::from(&str)` 

# Additional APIs

## Cmd (former App)

* `Cmd::mut_arg`
* `Cmd::unset_global_setting`

## Arg

