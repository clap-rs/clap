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

  # tests other things
  export extern "my-app some_cmd" [
    --config: string          # the other case to test
    ...path: string
    --version(-V)             # Print version information
  ]

  export extern "my-app some-cmd-with-hyphens" [
    --version(-V)             # Print version information
  ]

  export extern "my-app some-hidden-cmd" [
    --version(-V)             # Print version information
  ]

}

use completions *
