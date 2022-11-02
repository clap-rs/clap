module completions {

  export extern my-app [
    --single-quotes           # Can be 'always', 'auto', or 'never'
    --double-quotes           # Can be "always", "auto", or "never"
    --backticks               # For more information see `echo test`
    --backslash               # Avoid '\n'
    --brackets                # List packages [filter]
    --expansions              # Execute the shell command with $SHELL
    --version(-V)             # Print version information
  ]

  # Can be 'always', 'auto', or 'never'
  export extern "my-app cmd-single-quotes" [
  ]

  # Can be "always", "auto", or "never"
  export extern "my-app cmd-double-quotes" [
  ]

  # For more information see `echo test`
  export extern "my-app cmd-backticks" [
  ]

  # Avoid '\n'
  export extern "my-app cmd-backslash" [
  ]

  # List packages [filter]
  export extern "my-app cmd-brackets" [
  ]

  # Execute the shell command with $SHELL
  export extern "my-app cmd-expansions" [
  ]

}

use completions *
