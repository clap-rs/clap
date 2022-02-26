*[Jump to source](3_hand_subcommand.rs)*

# 3: Add hand-implemented subcommands to derived CLI

When using the derive API, you can use `#[clap(subcommand)]` inside the struct to add subcommands. The type of the field is usually an enum that derived `Parser`. However, you can also implement the `Subcommand` trait manually on this enum (or any other type) and it can still be used inside the struct created with the derive API. The implementation of the `Subcommand` trait will use the builder API to add the subcommands to the `Command` instance created behind the scenes for you by the derive API.

Notice how in the previous example we used `augment_subcommands` on an enum that derived `Parser`, whereas now we implement `augment_subcommands` ourselves, but the derive API calls it automatically since we used the `#[clap(subcommand)]` attribute.
