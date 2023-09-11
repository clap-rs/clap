module completions {

  def "nu-complete my-app choice" [] {
    [ "first" "second" ]
  }

  # Tests completions
  export extern my-app [
    file?: string             # some input file
    --config(-c)              # some config file
    --conf                    # some config file
    -C                        # some config file
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

  # Print this message or the help of the given subcommand(s)
  export extern "my-app help" [
  ]

  # tests things
  export extern "my-app help test" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "my-app help help" [
  ]

}

export use completions *
