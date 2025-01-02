module completions {

  export extern test [
    --global                  # everywhere
    --generate: string        # generate
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  def "nu-complete test action choice" [] {
    [ "first" "second" ]
  }

  export extern "test action" [
    --set-true                # bool
    --set: string             # value
    --count                   # number
    --choice: string@"nu-complete test action choice" # enum
    --global                  # everywhere
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  export extern "test quote" [
    --single-quotes           # Can be 'always', 'auto', or 'never'
    --double-quotes           # Can be "always", "auto", or "never"
    --backticks               # For more information see `echo test`
    --backslash               # Avoid '\n'
    --brackets                # List packages [filter]
    --expansions              # Execute the shell command with $SHELL
    --global                  # everywhere
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # Can be 'always', 'auto', or 'never'
  export extern "test quote cmd-single-quotes" [
    --global                  # everywhere
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # Can be "always", "auto", or "never"
  export extern "test quote cmd-double-quotes" [
    --global                  # everywhere
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # For more information see `echo test`
  export extern "test quote cmd-backticks" [
    --global                  # everywhere
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # Avoid '\n'
  export extern "test quote cmd-backslash" [
    --global                  # everywhere
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # List packages [filter]
  export extern "test quote cmd-brackets" [
    --global                  # everywhere
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # Execute the shell command with $SHELL
  export extern "test quote cmd-expansions" [
    --global                  # everywhere
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "test quote help" [
  ]

  # Can be 'always', 'auto', or 'never'
  export extern "test quote help cmd-single-quotes" [
  ]

  # Can be "always", "auto", or "never"
  export extern "test quote help cmd-double-quotes" [
  ]

  # For more information see `echo test`
  export extern "test quote help cmd-backticks" [
  ]

  # Avoid '\n'
  export extern "test quote help cmd-backslash" [
  ]

  # List packages [filter]
  export extern "test quote help cmd-brackets" [
  ]

  # Execute the shell command with $SHELL
  export extern "test quote help cmd-expansions" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "test quote help help" [
  ]

  export extern "test value" [
    --delim: string
    --tuple: string
    --require-eq: string
    ...term: string
    --global                  # everywhere
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  export extern "test pacman" [
    --global                  # everywhere
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  export extern "test pacman one" [
    --global                  # everywhere
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  export extern "test pacman two" [
    --global                  # everywhere
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "test pacman help" [
  ]

  export extern "test pacman help one" [
  ]

  export extern "test pacman help two" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "test pacman help help" [
  ]

  export extern "test last" [
    first?: string
    free?: string
    --global                  # everywhere
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  export extern "test alias" [
    --flag(-f)                # cmd flag
    --flg                     # cmd flag
    -F                        # cmd flag
    --option(-o): string      # cmd option
    --opt: string             # cmd option
    -O: string                # cmd option
    positional?: string
    --global                  # everywhere
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  def "nu-complete test hint choice" [] {
    [ "bash" "fish" "zsh" ]
  }

  export extern "test hint" [
    --choice: string@"nu-complete test hint choice"
    --unknown: string
    --other: string
    --path(-p): path
    --file(-f): path
    --dir(-d): path
    --exe(-e): path
    --cmd-name: string
    --cmd(-c): string
    command_with_args?: string
    --user(-u): string
    --host(-H): string
    --url: string
    --email: string
    --global                  # everywhere
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "test help" [
  ]

  export extern "test help action" [
  ]

  export extern "test help quote" [
  ]

  # Can be 'always', 'auto', or 'never'
  export extern "test help quote cmd-single-quotes" [
  ]

  # Can be "always", "auto", or "never"
  export extern "test help quote cmd-double-quotes" [
  ]

  # For more information see `echo test`
  export extern "test help quote cmd-backticks" [
  ]

  # Avoid '\n'
  export extern "test help quote cmd-backslash" [
  ]

  # List packages [filter]
  export extern "test help quote cmd-brackets" [
  ]

  # Execute the shell command with $SHELL
  export extern "test help quote cmd-expansions" [
  ]

  export extern "test help value" [
  ]

  export extern "test help pacman" [
  ]

  export extern "test help pacman one" [
  ]

  export extern "test help pacman two" [
  ]

  export extern "test help last" [
  ]

  export extern "test help alias" [
  ]

  export extern "test help hint" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "test help help" [
  ]

}

export use completions *
