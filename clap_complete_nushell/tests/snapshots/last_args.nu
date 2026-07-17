module completions {

  export extern my-app [
    --help(-h)                # Print help
  ]

  export extern "my-app single" [
    --help(-h)                # Print help
    arg: string
  ]

  export extern "my-app multiple" [
    --help(-h)                # Print help
    ...args: string
  ]

  def "nu-complete my-app choice arg" [] {
    [ "bash" "zsh" "fish" ]
  }

  export extern "my-app choice" [
    --help(-h)                # Print help
    arg: string@"nu-complete my-app choice arg"
  ]

  def "nu-complete my-app multiple-choice args" [] {
    [ "bash" "zsh" "fish" ]
  }

  export extern "my-app multiple-choice" [
    --help(-h)                # Print help
    ...args: string@"nu-complete my-app multiple-choice args"
  ]

  export extern "my-app any-path" [
    --help(-h)                # Print help
    arg: path
  ]

  export extern "my-app multiple-any-path" [
    --help(-h)                # Print help
    ...args: path
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "my-app help" [
  ]

  export extern "my-app help single" [
  ]

  export extern "my-app help multiple" [
  ]

  export extern "my-app help choice" [
  ]

  export extern "my-app help multiple-choice" [
  ]

  export extern "my-app help any-path" [
  ]

  export extern "my-app help multiple-any-path" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "my-app help help" [
  ]

}

export use completions *
