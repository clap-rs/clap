module completions {

  # Tests positional argument index ordering
  export extern my-app [
    --flag(-f)                # some flag
    --option(-o): string      # some option
    --help(-h)                # Print help
    --version(-V)             # Print version
    first?: string            # first positional
    second?: string           # second positional
    third?: string            # third positional
  ]

}

export use completions *
