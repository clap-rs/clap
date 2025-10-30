module completions {

  # testing nushell completions
  export extern my-app [
    --flag(-f)                # cmd flag
    --flg                     # cmd flag
    -F                        # cmd flag
    --option(-o): string      # cmd option
    --opt: string             # cmd option
    -O: string                # cmd option
    --help(-h)                # Print help
    --version(-V)             # Print version
    positional?: string
  ]

}

export use completions *
