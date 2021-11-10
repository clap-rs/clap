Example of a `hostname-style` multicall program

See the documentation for clap::AppSettings::Multicall for rationale.

This example omits the implementation of displaying address config

```bash
$ hostname hostname
www
$ hostname dnsdomainname
example.com
```
*Note: without the links setup, we can't demonostrate the multicall behavior*

Though users must pass something:
```bash
$ hostname
? failed
hostname 

USAGE:
    hostname [SUBCOMMAND]

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    dnsdomainname    show domain name part of FQDN
    help             Print this message or the help of the given subcommand(s)
    hostname         show hostname part of FQDN
```
