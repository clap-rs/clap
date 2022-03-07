*[Jump to source](1_augment_args.rs)*

# 1: Using derived arguments in a builder application

When using the derive API, you can `#[clap(flatten)]` a struct deriving `Args` into a struct deriving `Args` or `Parser`. This example shows how you can augment a `Command` instance created using the builder API with `Args` created using the derive API.

It uses the `Args::augment_args` method to add the arguments to the `Command` instance.

Crates such as [clap-verbosity-flag](https://github.com/rust-cli/clap-verbosity-flag) provide structs that implement `Args` or `Parser`. Without the technique shown in this example, it would not be possible to use such crates with the builder API. `augment_args` to the rescue!
