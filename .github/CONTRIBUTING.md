# How to Contribute

Contributions are always welcome! And there is a multitude of ways in which you can help depending on what you like to do, or are good at. Anything from documentation, code cleanup, issue completion, new features, you name it, even filing issues is contributing and greatly appreciated!

Another really great way to help is if you find an interesting, or helpful way in which to use `clap_derive`. You can either add it to the [examples/](examples) directory, or file an issue and tell me. I'm all about giving credit where credit is due :)

### Testing Code

To test with all features both enabled and disabled, you can run these commands:

```sh
$ cargo test 
```

Alternatively, if you have [`just`](https://github.com/casey/just) installed you can run the prebuilt recipes. *Not* using `just` is perfectly fine as well, it simply bundles commands automatically.

For example, to test the code, as above simply run:

```sh
$ just run-tests
```

From here on, I will list the appropriate `cargo` command as well as the `just` command.

Sometimes it's helpful to only run a subset of the tests, which can be done via:

```sh
$ cargo test --test <test_name>

# Or

$ just run-test <test_name>
```

### Linting Code

During the CI process `clap_derive` runs against many different lints using [`clippy`](https://github.com/Manishearth/rust-clippy). In order to check if these lints pass on your own computer prior to submitting a PR you'll need a nightly compiler.

In order to check the code for lints run either:

```sh
$ cargo +nightly build --features lints

# Or

$ just lint
```

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
2. **Ensure the tests are passing.** Run the tests (`cargo test`), alternatively `just run-tests` if you have `just` installed.
3. **Optional** Run the lints (`cargo +nightly build --features lints`) (requires a nightly compiler), alternatively `just lint`
4. Ensure your changes contain documentation if adding new APIs or features.

### Preparing the PR

1. `git rebase` into concise commits and remove `--fixup`s or `wip` commits (`git rebase -i HEAD~NUM` where `NUM` is number of commits back to start the rebase)
2. Push your changes back to your fork (`git push origin $your-branch`)
3. Create a pull request against `master`! (You can also create the pull request first, and we'll merge when ready. This a good way to discuss proposed changes.)

### Other ways to contribute

Another really great way to help is if you find an interesting, or helpful way in which to use `clap_derive`. You can either add it to the [examples/](../examples) directory, or file an issue and tell me. I'm all about giving credit where credit is due :)
