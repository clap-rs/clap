# How to Contribute

Contributions are always welcome! And there is a multitude of ways in which you can help depending on what you like to do, or are good at. Anything from documentation, code cleanup, issue completion, new features, you name it, even filing issues is contributing and greatly appreciated!

## Goals

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

## General Overview

### Where to Start

- [Discussions](https://github.com/clap-rs/clap/discussions) can be useful for getting help and brainstorming
- [Issues](https://github.com/clap-rs/clap/issues) work well discussing a need and how to solve it
  - Focus: requirements gathering and design discussions
  - Sometimes a branch or Draft PR might be used to demonstrate an idea
- [PRs](https://github.com/clap-rs/clap/pulls) work well for when the solution has already been discussed as an Issue or there is little to no discussion (obvious bug or documentation fixes)
  - Focus: implementation discussions

### Compatibility Expectations

Our releases fall into one of:
- Major releases which are reserved for breaking changes
  - Aspire to at least 6-9 months between releases
  - Remove all deprecated functionality
  - Changes in help/error output that could cause glaring inconsistencies in end-user applications
  - Try to minimize new breaking changes to ease user transition and reduce time "we go dark" (unreleased feature-branch)
  - Upon release, a minor release will be made for the previous major that enables `deprecated` feature by default
- Minor releases which are for minor compatibility changes
  - Aspire to at least 2 months between releases
  - Changes to MSRV
  - Wide-spread help/error output changes that would cause minor inconsistencies in end-user applications
  - Deprecating existing functionality (behind the `deprecated` feature flag)
  - Making the `deprecated` feature flag enabled-by-default (only on last planned minor release)
  - `#[doc(hidden)]` all deprecated items in the prior minor release
- Patch releases
  - One for every user-facing, user-contributed PR (i.e. release early, release often)
  - Changes in help/error output that are one-off or improving consistency so as to not increase inconsistency with end-user applications

If your change does not fit within a "patch" release, please coordinate with the clap maintainers for how to handle the situation.

Some practices to avoid breaking changes
- Duplicate functionality, with old functionality marked as "deprecated"
  - Common documentation pattern: `/// Deprecated in [Issue #XXX](https://github.com/clap-rs/clap/issues/XXX), replaced with [intra-doc-link]`
  - Common deprecation pattern: `#[cfg_attr(feature = "deprecated", deprecated(since = "X.Y.Z", note = "Replaced with `ITEM` in Issue #XXX"))]`
    - `deprecated` feature flag is to allow people to process them on their
      time table and allow us to process feedback from early adopters before
      requiring everyone to process them on the next major version.
  - Please keep API addition and deprecation in separate commits in a PR to make it easier to review
- Develop the feature behind an `unstable-<name>` feature flag with a stablization tracking issue (e.g. [Multicall Tracking issue](https://github.com/clap-rs/clap/issues/2861))

### Version Support Policy

As we work towards [a more flexible architecture](https://github.com/clap-rs/clap/discussions/3476), we hope to support multiple major versions to help ease users through the upgrade churn.

| Version                                              | Status        | Support | End-of-Life |
|------------------------------------------------------|---------------|---------|-------------|
| [v4](https://github.com/clap-rs/clap/tree/master)    | active        | Features and bug fixes target `master` by default | TBD |
| [v3](https://github.com/clap-rs/clap/tree/v3-master) | maintenance   | Accepting trivial cherry-picks from `master` (i.e. minimal conflict resolution) by contributors and fixes for ecosystem-wide showstoppers | TBD |
| [v2](https://github.com/clap-rs/clap/tree/v2-master) | deprecated    | Only accepting fixes for ecosystem-wide showstoppers | TBD |
| v1                                                   | unsupported   | \- | \- |

Note: We have not yet determined the End-of-Life schedule for previous major versions.  We will give at least a 2 month warning before changing the support status.

### Verifying Changes

A common (sub)set of commands for verifying your change:
```console
$ make test-full
$ make clippy-full
$ make doc
```
*(If `make` is not available on your system, you can look up what these expand to in the [Makefile](./Makefile))*

Check out the [Makefile](./Makefile) for more commands run by CI.

### Debugging Clap

A helpful technique is to see the `clap` debug output while developing features. In order to see the debug output while running the full test suite or individual tests, run:

```console
$ cargo test --features debug

# Or for individual tests
$ cargo test --test <test_name> --features debug
```

### Preparing the PR

1. `git rebase` into concise commits and remove `--fixup`s or `wip` commits (`git rebase -i HEAD~NUM` where `NUM` is number of commits back to start the rebase)
2. Push your changes back to your fork (`git push origin $your-branch`)
3. Create a pull request against `master`! (You can also create the pull request first, and we'll merge when ready. This a good way to discuss proposed changes.)

PR expectations:
- Changes are tested and, if needed, documented
- PRs remain small and focused
 - If needed, we can put changes behind feature flags as they evolve
- Commits are atomic (i.e. do a single thing)
- Commits are in [Conventional Commit](https://www.conventionalcommits.org/) style

We recognize that these are ideals and we don't want lack of comfort with git
to get in the way of contributing.  If you didn't do these, bring it up with
the maintainers and we can help work around this.

## Conditions for fulfilling a bounty:

1. You should make a pull request which fixes the issue the bounty was promised for
2. The pull request should be merged by one of the maintainers

### Below are the steps to redeem a bounty:

1. Go to https://opencollective.com/clap/expenses/new.
2. Select **Invoice**.
3. Enter **Expense Title** as "Issue Bounty".
4. In **Description**, link the issue you are redeeming _(Ex: `https://github.com/clap-rs/clap/issues/1464`)_
5. In **Amount**, write the amount that the issue promised _(Ex: 10)_
6. Fill payment information and submit
7. Wait for us to approve it

### Can I forgo the bounty?

Yes, you can. In that case, you don't have to do anything except writing a
comment on the issue saying that I do. The bounty will be reassigned to another
issue.

## Specific Tasks

### Section-specific CONTRIBUTING

- [Example CONTRIBUTING](./examples/README.md#contributing)
- [Tutorial (builder) CONTRIBUTING](./examples/tutorial_builder/README.md#contributing)
- [Tutorial (derive) CONTRIBUTING](./examples/tutorial_derive/README.md#contributing)
- [clap_derive CONTRIBUTING](./clap_derive/CONTRIBUTING.md)

### Updating MSRV

Search for `MSRV`, for example
```bash
$ rg --hidden MSRV
```
And update all of the references
