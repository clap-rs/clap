module completions {

  def "nu-complete my-app choice" [] {
    [ "bash" "fish" "zsh" ]
  }

  export extern my-app [
    --choice: string@"nu-complete my-app choice"
    --unknown: string
    --other: string
    --path(-p): path
    --file(-f): path
    --dir(-d): path
    --exe(-e): path
    --cmd-name: string
    --cmd(-c): string
    command_with_args?: string
    --user(-u): string
    --host(-H): string
    --url: string
    --email: string
    --help(-h)                # Print help
  ]

}

export use completions *
