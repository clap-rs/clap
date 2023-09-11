module completions {

  export extern my-app [
    --single-quotes           # Can be 'always', 'auto', or 'never'
    --double-quotes           # Can be "always", "auto", or "never"
    --backticks               # For more information see `echo test`
    --backslash               # Avoid '\n'
    --brackets                # List packages [filter]
    --expansions              # Execute the shell command with $SHELL
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # Can be 'always', 'auto', or 'never'
  export extern "my-app cmd-single-quotes" [
    --help(-h)                # Print help
  ]

  # Can be "always", "auto", or "never"
  export extern "my-app cmd-double-quotes" [
    --help(-h)                # Print help
  ]

  # For more information see `echo test`
  export extern "my-app cmd-backticks" [
    --help(-h)                # Print help
  ]

  # Avoid '\n'
  export extern "my-app cmd-backslash" [
    --help(-h)                # Print help
  ]

  # List packages [filter]
  export extern "my-app cmd-brackets" [
    --help(-h)                # Print help
  ]

  # Execute the shell command with $SHELL
  export extern "my-app cmd-expansions" [
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "my-app help" [
  ]

  # Can be 'always', 'auto', or 'never'
  export extern "my-app help cmd-single-quotes" [
  ]

  # Can be "always", "auto", or "never"
  export extern "my-app help cmd-double-quotes" [
  ]

  # For more information see `echo test`
  export extern "my-app help cmd-backticks" [
  ]

  # Avoid '\n'
  export extern "my-app help cmd-backslash" [
  ]

  # List packages [filter]
  export extern "my-app help cmd-brackets" [
  ]

  # Execute the shell command with $SHELL
  export extern "my-app help cmd-expansions" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "my-app help help" [
  ]

}

export use completions *
