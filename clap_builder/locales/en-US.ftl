# Core clap localization strings - English

# Help and version
print-help = Print help
print-version = Print version
clap-print-help = Print this message or the help of the given subcommand(s)

# Usage formatting
usage-header = Usage
usage-options = OPTIONS
usage-default-subcommand = COMMAND

# Error messages
clap-error-arg-undefined = Argument `{$id}` is undefined
clap-error-group-undefined = Group `{$id}` is undefined
clap-error-command-undefined = Command `{$name}` is undefined

# Help messages for built-in functionality
help-short-help = Print help (see more with '--help')
help-long-help = Print help (see a summary with '-h')

# Help formatting
help-commands = Commands

# Error messages
error-unrecognized-subcommand = unrecognized subcommand '{ $subcommand }'

# Core error system
error-unknown-cause = unknown cause
error-label = error
error-tip = tip

# Argument conflict errors
error-argument-cannot-be-used-multiple-times = the argument '{ $argument }' cannot be used multiple times
error-argument-cannot-be-used-with = the argument '{ $argument }' cannot be used with
error-subcommand-cannot-be-used-with = the subcommand '{ $subcommand }' cannot be used with
error-one-or-more-other-arguments = one or more of the other specified arguments

# Value and assignment errors
error-equal-sign-needed = equal sign is needed when assigning values to '{ $argument }'
error-value-required-but-none-supplied = a value is required for '{ $argument }' but none was supplied
error-invalid-value-for-argument = invalid value '{ $value }' for '{ $argument }'
error-possible-values = possible values

# Subcommand errors
error-requires-subcommand = '{ $command }' requires a subcommand but one was not provided
error-subcommands = subcommands

# Missing arguments
error-missing-required-arguments = the following required arguments were not provided:

# Value count errors
error-unexpected-value-no-more-expected = unexpected value '{ $value }' for '{ $argument }' found; no more were expected
error-values-required-only-provided = { $min_values } values required by '{ $argument }'; only { $actual_values } { $were_provided }
error-wrong-number-of-values = { $expected_values } values required for '{ $argument }' but { $actual_values } { $were_provided }
error-were-provided = were provided
error-was-provided = was provided

# Unknown argument errors
error-unexpected-argument = unexpected argument '{ $argument }' found

# Help messages
error-for-more-information-try = For more information, try '{ $help }'.

# Context types for suggestions
error-context-subcommand = subcommand
error-context-argument = argument
error-context-value = value
error-context-subcommands = subcommands
error-context-arguments = arguments
error-context-values = values

# Suggestion messages
error-similar-exists-singular = a similar { $context } exists: '{ $suggestion }'
error-similar-exists-plural = some similar { $context } exist: '{ $suggestion }'