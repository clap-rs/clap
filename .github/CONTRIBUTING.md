# How to Contribute

Contributions are always welcome! Please use the following guidelines when contributing to `clap`

## Goals

There are a few goals of `clap` that I'd like to maintain throughout contributions.

* Remain backwards compatible when possible
  - If backwards compatibility *must* be broken, use deprecation warnings if at all possible before removing legacy code
  - This does not apply for security concerns
  - `clap` officially supports the current stable version of Rust, minus two releases (i.e. if 1.13.0 is current, `clap` must support 1.11.0 and beyond)
* Parse arguments quickly
  - Parsing of arguments shouldn't slow down usage of the main program
  - This is also true of generating help and usage information (although *slightly* less stringent, as the program is about to exit)
* Try to be cognizant of memory usage
  - Once parsing is complete, the memory footprint of `clap` should be low since the  main program is the star of the show
* `panic!` on *developer* error, exit gracefully on *end-user* error

### Commit Messages

I use a [conventional](https://github.com/ajoslin/conventional-changelog/blob/a5505865ff3dd710cf757f50530e73ef0ca641da/conventions/angular.md) changelog format so I can update my changelog automatically using [clog](https://github.com/clog-tool/clog-cli)

 * Please format your commit subject line using the following format: `TYPE(COMPONENT): MESSAGE` where `TYPE` is one of the following:
    - `api`  - An addition to the API
    - `setting` - A new `AppSettings` variant
    - `feat` - A new feature of an existing API
    - `imp`  - An improvement to an existing feature/API
    - `perf` - A performance improvement
    - `docs` - Changes to documentation only
    - `tests` - Changes to the testing framework or tests only
    - `fix` - A bug fix
    - `refactor` - Code functionality doesn't change, but underlying structure may
    - `style` - Stylistic changes only, no functionality changes
    - `wip` - A work in progress commit (Should typically be `git rebase`'ed away)
    - `chore` - Catch all or things that have to do with the build system, etc
    - `examples` - Changes to existing example, or a new example
 * The `COMPONENT` is optional, and may be a single file, directory, or logical component. Parenthesis can be omitted if you are opting not to use the `COMPONENT`. 

### Tests and Documentation

1. Create tests for your changes
2. **Ensure the tests are passing.** Run the tests (`cargo test --features "yaml unstable"`), alternatively `just run-tests` if you have `just` installed.
3. **Optional** Run the lints (`cargo build --features lints`) (requires a nightly compiler), alternatively `just lint`
4. Ensure your changes contain documentation if adding new APIs or features.

### Preparing the PR

1. `git rebase` into concise commits and remove `--fixup`s or `wip` commits (`git rebase -i HEAD~NUM` where `NUM` is number of commits back to start the rebase)
2. Push your changes back to your fork (`git push origin $your-branch`)
3. Create a pull request against `master`! (You can also create the pull request first, and we'll merge when ready. This a good way to discuss proposed changes.)

### Other ways to contribute

Another really great way to help is if you find an interesting, or helpful way in which to use `clap`. You can either add it to the [examples/](../examples) directory, or file an issue and tell me. I'm all about giving credit where credit is due :)

