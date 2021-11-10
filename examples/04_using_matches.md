Once all App settings (including all arguments) have been set, you call get_matches() which
parses the string provided by the user, and returns all the valid matches to the ones you
specified.

For example:
```bash
$ 04_using_matches input
Doing real work with file: input
$ 04_using_matches input --debug
Debugging is turned on
Doing real work with file: input
$ 04_using_matches input --config path
Using config file: path
Doing real work with file: input
```
