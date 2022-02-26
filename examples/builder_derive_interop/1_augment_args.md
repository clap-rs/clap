*[Jump to source](1_augment_args.rs)*

# 1: Augment built `Command` with derived `Args`

When using the derive API, you can `#[clap(flatten)]` a struct deriving `Args` into a struct deriving `Args` or `Parser`. This example shows how you can augment a `Command` instance created using the builder API with `Args` created using the derive API.

It uses the `Args::augment_args` method to add the arguments to the `Command` instance.
