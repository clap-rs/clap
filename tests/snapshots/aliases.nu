module completions {

  # testing nushell completions
  export extern my-app [
    --flag(-f)	# cmd flag
    --flg	# cmd flag
    -F	# cmd flag
    --option(-o): string	# cmd option
    --opt: string	# cmd option
    -O: string	# cmd option
    positional?: string
    --version(-V)	# Print version information
  ]

}

use completions *
