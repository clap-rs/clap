module completions {

  # Tests positional argument index ordering
  export extern my-app [
    --flag(-f)                # some flag
    third?: string            # third positional
    first?: string            # first positional
    --option(-o): string      # some option
    second?: string           # second positional
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

}

export use completions *
