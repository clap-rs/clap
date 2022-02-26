*[Jump to source](2_augment_subcommands.rs)*

# 2: Augment built `Command` with derived subcommands

When using the derive API, you can use `#[clap(subcommand)]` inside the struct to add subcommands. The type of the field is usually an enum that derived `Parser`. However, you can also add the subcommands in that enum to a `Command` instance created with the builder API.

It uses the `Subcommand::augment_subcommands` method to add the subcommands to the `Command` instance.
