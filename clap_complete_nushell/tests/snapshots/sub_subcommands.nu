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

  # top level subcommand
  export extern "my-app some_cmd" [
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  def "nu-complete my-app some_cmd sub_cmd config" [] {
    [ "\"Lest quotes, aren't escaped.\"" "\"Second to trigger display of options\"" ]
  }

  # sub-subcommand
  export extern "my-app some_cmd sub_cmd" [
    --config: string@"nu-complete my-app some_cmd sub_cmd config" # the other case to test
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "my-app some_cmd help" [
  ]

  # sub-subcommand
  export extern "my-app some_cmd help sub_cmd" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "my-app some_cmd help help" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "my-app help" [
  ]

  # tests things
  export extern "my-app help test" [
  ]

  # top level subcommand
  export extern "my-app help some_cmd" [
  ]

  # sub-subcommand
  export extern "my-app help some_cmd sub_cmd" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "my-app help help" [
  ]

}

export use completions *
