module completions {

  export extern my-app [
    -c
    -v
    --help(-h)                # Print help
  ]

  # Subcommand with a second line
  export extern "my-app test" [
    -d
    -c
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "my-app help" [
  ]

  # Subcommand with a second line
  export extern "my-app help test" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "my-app help help" [
  ]

}

export use completions *
