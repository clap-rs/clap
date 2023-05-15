```console
$ 03_05_default_values_derive --help
A simple to use, efficient, and full-featured Command Line Argument Parser

Usage: 03_05_default_values_derive[EXE] [OPTIONS]

Options:
      --port <PORT>      [default: 2020]
      --host <HOST>      [default: example.com]
      --config <CONFIG>  [default: config.json]
      --seed <SEED>      [default: 1 2 3]
      --source <SOURCE>  [default: a b]
  -h, --help             Print help
  -V, --version          Print version

$ 03_05_default_values_derive
Cli {
    port: 2020,
    host: "example.com",
    config: "config.json",
    seed: [
        1,
        2,
        3,
    ],
    source: [
        "a",
        "b",
    ],
}

```
