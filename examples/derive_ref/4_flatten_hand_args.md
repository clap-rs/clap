*[Jump to source](4_flatten_hand_args.rs)*

# 4: Flattening hand-implemented args into a derived application

When using the derive API, you can use `#[clap(flatten)]` inside the struct to add arguments as if they were added directly to the containing struct. The type of the field is usually an struct that derived `Args`. However, you can also implement the `Args` trait manually on this struct (or any other type) and it can still be used inside the struct created with the derive API. The implementation of the `Args` trait will use the builder API to add the arguments to the `Command` instance created behind the scenes for you by the derive API.

Notice how in the example 1 we used `augment_args` on the struct that derived `Parser`, whereas now we implement `augment_args` ourselves, but the derive API calls it automatically since we used the `#[clap(flatten)]` attribute.
