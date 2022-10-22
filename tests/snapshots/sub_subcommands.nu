module completions {

  # Tests completions
  export extern my-app [
    file?: string	# some input file
    --config(-c)	# some config file
    choice?: string
    --version(-V)	# Print version information
  ]

  # tests things
  export extern "my-app test" [
    --case: string	# the case to test
    --version(-V)	# Print version information
  ]

  # top level subcommand
  export extern "my-app some_cmd" [
    --version(-V)	# Print version information
  ]

  # sub-subcommand
  export extern "my-app some_cmd sub_cmd" [
    --config: string	# the other case to test
    --version(-V)	# Print version information
  ]

}

use completions *
