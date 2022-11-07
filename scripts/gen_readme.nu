#!/usr/bin/env nu

let example_file = "examples/sub_subcommands.rs"

def generate_url [baseurl: string, ...path: string] {
  [$baseurl]
  |append $path
  |str join '/'
}

def generate_preamble [user: string, repo: string] {
  let baseurl = "https://img.shields.io"
  let crates = "https://crates.io"
  let docs = "https://docs.rs"
  let github = "https://github.com"
  let branch = "main"

  $"# ($repo)\n
Generates [Nushell]\((generate_url $github nushell/nushell)\) completions for [`clap`]\((generate_url $github clap-rs/clap)\) based CLIs\n
[![Crates.io]\((generate_url $baseurl crates/v $repo)\)]\((generate_url $crates crates $repo)\)
[![Crates.io]\((generate_url $baseurl crates/d $repo)\)]\((generate_url $crates crates $repo)\)
[![License]\((generate_url $baseurl github/license $user)/(generate_url $repo)\)]\(LICENSE\)
[![docs.rs]\((generate_url $baseurl docsrs $repo)\)]\((generate_url $docs $repo)\)
[![Build Status]\((generate_url $baseurl github/workflow/status $user $repo CI $branch)\)]\((generate_url $github $user $repo actions/workflows/ci.yaml?query=branch%3A($branch))\)
[![GitHub last commit]\((generate_url $baseurl github/last-commit $user $repo)\)]\((generate_url $github $user $repo commits/($branch))\)\n"
}

def code_to_md [title: string, lang: string, code: string] {
  $"### ($title)\n\n```($lang)\n($code)```\n"
}

def generate_md [file: path] {
  let package = (open Cargo.toml | get package)
  let user = ($package.authors | first | str replace ' <.*@.*>$' '')
  let repo = $package.name
  let stem = ($file | path parse | get stem)
  let rust_code = (open -r $file)
  let nu_code = (cargo run --quiet --example $stem)
  let rust_example = (code_to_md myapp.rs rust $rust_code)
  let nu_example = (code_to_md myapp.nu nu $nu_code)

  |generate_preamble $user $repo
  |[$in]
  |append "## Examples\n"
  |append $rust_example
  |append $nu_example
  |str join "\n"
  |save -r README.md
}

generate_md $example_file
