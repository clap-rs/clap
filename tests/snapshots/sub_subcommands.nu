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
    --version(-V)             # Print version information
  ]

  # tests things
  export extern "my-app test" [
    --case: string            # the case to test
    --version(-V)             # Print version information
  ]

  # top level subcommand
  export extern "my-app some_cmd" [
    --version(-V)             # Print version information
  ]

  def "nu-complete my-app some_cmd sub_cmd config" [] {
    [ "\"Lest quotes, aren't escaped.\"" "\"Second to trigger display of options\"" ]
  }

  # sub-subcommand
  export extern "my-app some_cmd sub_cmd" [
    --config: string@"nu-complete my-app some_cmd sub_cmd config" # the other case to test
    --version(-V)             # Print version information
  ]

}

use completions *
