module completions {

  export extern my-app [
    --help(-h)                # Print help
  ]

  export extern "my-app single" [
    --help(-h)                # Print help
    arg: string
    last_arg: string
  ]

  export extern "my-app multiple" [
    --help(-h)                # Print help
    ...args: string
    ...last_args: string
  ]

  def "nu-complete my-app choice arg" [] {
    [ "bash" "zsh" "fish" ]
  }

  def "nu-complete my-app choice last_arg" [] {
    [ "nushell" "powershell" ]
  }

  export extern "my-app choice" [
    --help(-h)                # Print help
    arg: string@"nu-complete my-app choice arg"
    last_arg: string@"nu-complete my-app choice last_arg"
  ]

  def "nu-complete my-app multipleChoice args" [] {
    [ "bash" "zsh" "fish" ]
  }

  def "nu-complete my-app multipleChoice last_args" [] {
    [ "nushell" "powershell" ]
  }

  export extern "my-app multipleChoice" [
    --help(-h)                # Print help
    ...args: string@"nu-complete my-app multipleChoice args"
    ...last_args: string@"nu-complete my-app multipleChoice last_args"
  ]

  export extern "my-app anyPath" [
    --help(-h)                # Print help
    arg: path
    last_arg: path
  ]

  export extern "my-app multipleAnyPath" [
    --help(-h)                # Print help
    ...args: path
    ...last_args: path
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

  export extern "my-app help multipleChoice" [
  ]

  export extern "my-app help anyPath" [
  ]

  export extern "my-app help multipleAnyPath" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "my-app help help" [
  ]

}

export use completions *
