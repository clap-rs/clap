**This requires enabling the [`derive` feature flag][crate::_features].**

Help:
```console
$ echo --help
Usage: echo --text <TEXT>

Options:
  -h, --help  Print help

Echo:
  -t, --text <TEXT>  The text to be echoed [aliases: text]
$ ping --help
Usage: ping

Options:
  -h, --help  Print help
$ exit --help
Usage: exit

Options:
  -h, --help  Print help
```

Echo:
```console
$ echo -t 'Hello, world!'
Hello, world!
```

Ping:
```console
$ ping
pong
```

Exit:
```console
$ exit
Exiting ...
```