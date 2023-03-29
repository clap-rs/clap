# Contributing to PROJECT

Thanks for wanting to contribute! There are many ways to contribute and we
appreciate any level you're willing to do.

## Feature Requests

Need some new functionality to help?  You can let us know by opening an
[issue][new issue]. It's helpful to look through [all issues][all issues] in
case its already being talked about.

## Bug Reports

Please let us know about what problems you run into, whether in behavior or
ergonomics of API.  You can do this by opening an [issue][new issue]. It's
helpful to look through [all issues][all issues] in case its already being
talked about.

## Pull Requests

Looking for an idea? Check our [issues][issues]. If it's look more open ended,
it is probably best to post on the issue how you are thinking of resolving the
issue so you can get feedback early in the process. We want you to be
successful and it can be discouraging to find out a lot of re-work is needed.

Already have an idea?  It might be good to first [create an issue][new issue]
to propose it so we can make sure we are aligned and lower the risk of having
to re-work some of it and the discouragement that goes along with that.

### Process

Before posting a PR, we request that the commit history get cleaned up.
However, we recommend avoiding this during the review to make it easier to
check how feedback was handled. Once the PR is ready, we'll ask you to clean up
the commit history from the review.  Once you let us know this is done, we can
move forward with merging!  If you are uncomfortable with these parts of git,
let us know and we can help.

For commit messages, we use [Conventional](https://www.conventionalcommits.org)
style.  If you already wrote your commits and don't feel comfortable changing
them, don't worry and go ahead and create your PR.  We'll work with you on the
best route forward. You can check your branch locally with
[`committed`](https://github.com/crate-ci/committed).

As a heads up, we'll be running your PR through the following gauntlet:
- warnings turned to compile errors
- `cargo test`
- `rustfmt`
- `clippy`
- `rustdoc`
- [`committed`](https://github.com/crate-ci/committed)
- [`typos`](https://github.com/crate-ci/typos)

## Releasing

Pre-requisites
- Running `cargo login`
- A member of `ORG:Maintainers`
- Push permission to the repo
- [`cargo-release`](https://github.com/crate-ci/cargo-release/)

When we're ready to release, a project owner should do the following
1. Update the changelog (see `cargo release changes` for ideas)
2. Determine what the next version is, according to semver
3. Run [`cargo release -x <level>`](https://github.com/crate-ci/cargo-release)

[issues]: https://github.com/ORG/PROJECT/issues
[new issue]: https://github.com/ORG/PROJECT/issues/new
[all issues]: https://github.com/ORG/PROJECT/issues?utf8=%E2%9C%93&q=is%3Aissue
