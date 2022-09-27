`singleton` is a simple example of using [`once_cell`]. Use a global state for command line option is a common pattern for small application. You can use either derive or builder API.

Clap doesn't recommend to use singleton, this example is only given for a Quick and Dirty (Q&D) bootstrap of small binary. Singleton pattern have caveats its make your code is less robust. A function that use global state is more difficult to test, you can't easily override options values for a specific test, you will have difficulty with cargo test ecosystem.

```console
$ singleton
The answer is 42!

```

[`once_cell`]: https://docs.rs/once_cell/latest/once_cell/