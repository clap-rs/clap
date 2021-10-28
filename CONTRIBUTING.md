# How to Contribute

Contributions are always welcome! And there is a multitude of ways in which you can help depending on what you like to do, or are good at. Anything from documentation, code cleanup, issue completion, new features, you name it, even filing issues is contributing and greatly appreciated!

Another really great way to help is if you find an interesting, or helpful way in which to use `clap`. You can either add it to the [examples/](examples) directory, or file an issue and tell me. I'm all about giving credit where credit is due :)

### Testing Code

To test with all features enabled, you can run this command:

```sh
$ make tests
```

Sometimes it's helpful to only run a subset of the tests, which can be done via:

```sh
$ make test <test_group> [test_name]
```

### Linting Code

During the CI process `clap` runs against many different lints using [`clippy`](https://github.com/rust-lang/rust-clippy).

In order to check the code for lints and to format it run either:

```sh
$ make lint
```

### Debugging Code

Another helpful technique is to see the `clap` debug output while developing features. In order to see the debug output while running an individual test, run:

```sh
$ make debug <test_group> [test_name]
```

### Tests and Documentation

1. Create tests for your changes
2. **Ensure the tests are passing.** Run the tests as specified above.
3. **Ensure linting is passing.** Run the lints as specified above.
4. Ensure your changes contain documentation if adding new APIs or features.

### Preparing the PR

1. `git rebase` into concise commits and remove `--fixup`s or `wip` commits (`git rebase -i HEAD~NUM` where `NUM` is number of commits back to start the rebase)
2. Push your changes back to your fork (`git push origin $your-branch`)
3. Create a pull request against `master`! (You can also create the pull request first, and we'll merge when ready. This a good way to discuss proposed changes.)

### Goals

There are a few goals of `clap` that I'd like to maintain throughout contributions. If your proposed changes break, or go against any of these goals we'll discuss the changes further before merging (but will *not* be ignored, all contributes are welcome!). These are by no means hard-and-fast rules, as I'm no expert and break them myself from time to time (even if by mistake or ignorance :P).

* Remain backwards compatible when possible
  - If backwards compatibility *must* be broken, use deprecation warnings if at all possible before removing legacy code
  - This does not apply for security concerns
* Parse arguments quickly
  - Parsing of arguments shouldn't slow down usage of the main program
  - This is also true of generating help and usage information (although *slightly* less stringent, as the program is about to exit)
* Try to be cognizant of memory usage
  - Once parsing is complete, the memory footprint of `clap` should be low since the main program is the star of the show
* `panic!` on *developer* error
  (e.g. [apps](https://github.com/clap-rs/clap/blob/62eff1f8d3394cef819b4aa7b23a1032fc584f03/src/build/app/debug_asserts.rs) and [args](https://github.com/clap-rs/clap/blob/62eff1f8d3394cef819b4aa7b23a1032fc584f03/src/build/arg/debug_asserts.rs)),
  exit gracefully on *end-user* error
