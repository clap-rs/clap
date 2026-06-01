module completions {

  export extern my-app [
    --help(-h)                # Print help
    ...args: string
    ...last_args: string
  ]

}

export use completions *
