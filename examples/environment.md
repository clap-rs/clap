```console
$ environment --help
A CLI that reads configuration from arguments, environment variables, or defaults (in that priority order).

Usage: environment[EXE] [OPTIONS]

Options:
      --host <HOST>  Address to bind to [env: APP_HOST=] [default: 127.0.0.1]
  -p, --port <PORT>  Port to listen on [env: APP_PORT=] [default: 3000]
  -v, --verbose      Enable verbose output [env: APP_VERBOSE=]
  -h, --help         Print help
  -V, --version      Print version

$ environment
Listening on 127.0.0.1:3000

$ environment --host 0.0.0.0 --port 8080
Listening on 0.0.0.0:8080

```
*(version number and `.exe` extension on windows replaced by placeholders)*
