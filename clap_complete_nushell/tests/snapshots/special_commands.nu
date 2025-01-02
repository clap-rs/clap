module completions {

  def "nu-complete my-app choice" [] {
    [ "first" "second" ]
  }

  # Tests completions
  export extern my-app [
    file?: path               # some input file
    --config(-c)              # some config file with another line
    --conf                    # some config file with another line
    -C                        # some config file with another line
    choice?: string@"nu-complete my-app choice"
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # tests things
  export extern "my-app test" [
    --case: string            # the case to test
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # tests other things
  export extern "my-app some_cmd" [
    --config: string          # the other case to test
    ...path: string
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  export extern "my-app some-cmd-with-hyphens" [
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  export extern "my-app some-hidden-cmd" [
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "my-app help" [
  ]

  # tests things
  export extern "my-app help test" [
  ]

  # tests other things
  export extern "my-app help some_cmd" [
  ]

  export extern "my-app help some-cmd-with-hyphens" [
  ]

  export extern "my-app help some-hidden-cmd" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "my-app help help" [
  ]

}

export use completions *
