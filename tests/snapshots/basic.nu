module completions {

  export extern my-app [
    -c
    -v
  ]

  # Subcommand
  export extern "my-app test" [
    -d
    -c
  ]

}

use completions *
