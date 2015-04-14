<a name="v0.6.2"></a>
## v0.6.2 (2015-04-14)


#### Features

* **macros**
  *  add ability to get mutliple typed values or exit ([0b87251f](https://github.com/kbknapp/clap-rs/commit/0b87251fc088234bee51c323c2b652d7254f7a59))
  *  add ability to get a typed multiple values ([e243fe38](https://github.com/kbknapp/clap-rs/commit/e243fe38ddbbf845a46c0b9baebaac3778c80927))
  *  add convenience macro to get a typed value or exit ([4b7cd3ea](https://github.com/kbknapp/clap-rs/commit/4b7cd3ea4947780d9daa39f3e1ddab53ad4c7fef))
  *  add convenience macro to get a typed value ([8752700f](https://github.com/kbknapp/clap-rs/commit/8752700fbb30e89ee68adbce24489ae9a24d33a9))



<a name="v0.6.1"></a>
## v0.6.1 (2015-04-13)


#### Bug Fixes

* **from_usage**  trim all whitespace before parsing ([91d29045](https://github.com/kbknapp/clap-rs/commit/91d2904599bd602deef2e515dfc65dc2863bdea0))



<a name="v0.6.0"></a>
## v0.6.0 (2015-04-13)


#### Bug Fixes

* **tests**  fix failing doc tests ([3710cd69](https://github.com/kbknapp/clap-rs/commit/3710cd69162f87221a62464f63437c1ce843ad3c))

#### Features

* **app**  add support for building args from usage strings ([d5d48bcf](https://github.com/kbknapp/clap-rs/commit/d5d48bcf463a4e494ef758836bd69a4c220bbbb5))
* **args**  add ability to create basic arguments from a usage string ([ab409a8f](https://github.com/kbknapp/clap-rs/commit/ab409a8f1db9e37cc70200f6f4a84a162692e618))



<a name="v0.5.14"></a>
## v0.5.14 (2015-04-10)


#### Bug Fixes

* **usage**
  *  remove unneeded space ([51372789](https://github.com/kbknapp/clap-rs/commit/5137278942121bc2593ce6e5dc224ec2682549e6))
  *  remove warning about unused variables ([ba817b9d](https://github.com/kbknapp/clap-rs/commit/ba817b9d815e37320650973f1bea0e7af3030fd7))

#### Features

* **usage**  add ability to get usage string for subcommands too ([3636afc4](https://github.com/kbknapp/clap-rs/commit/3636afc401c2caa966efb5b1869ef4f1ed3384aa))



<a name="v0.5.13"></a>
## v0.5.13 (2015-04-09)


#### Features

* **SubCommands**  add method to get name and subcommand matches together ([64e53928](https://github.com/kbknapp/clap-rs/commit/64e539280e23e567cf5de393b346eb0ca20e7eb5))
* **ArgMatches**  add method to get default usage string ([02462150](https://github.com/kbknapp/clap-rs/commit/02462150ca750bdc7012627d7e8d96379d494d7f))



<a name="v0.5.12"></a>
## v0.5.12 (2015-04-08)


#### Features

* **help**  sort arguments by name so as to not display a random order ([f4b2bf57](https://github.com/kbknapp/clap-rs/commit/f4b2bf5767386013069fb74862e6e938dacf44d2))



<a name="v0.5.11"></a>
## v0.5.11 (2015-04-08)


#### Bug Fixes

* **flags**  fix bug not allowing users to specify -v or -h ([90e72cff](https://github.com/kbknapp/clap-rs/commit/90e72cffdee321b79eea7a2207119533540062b4))



<a name="v0.5.10"></a>
## v0.5.10 (2015-04-08)


#### Bug Fixes

* **help**  fix spacing when option argument has not long version ([ca17fa49](https://github.com/kbknapp/clap-rs/commit/ca17fa494b68e92da83ee364bf64b0687006824b))



<a name="v0.5.9"></a>
## v0.5.9 (2015-04-08)


#### Bug Fixes

* **positional args**  all previous positional args become required when a latter one is required ([c14c3f31](https://github.com/kbknapp/clap-rs/commit/c14c3f31fd557c165570b60911d8ee483d89d6eb), closes [#50](https://github.com/kbknapp/clap-rs/issues/50))
* **clap**  remove unstable features for Rust 1.0 ([9abdb438](https://github.com/kbknapp/clap-rs/commit/9abdb438e36e364d41550e7f5d44ebcaa8ee6b10))
* **args**  improve error messages for arguments with mutual exclusions ([18dbcf37](https://github.com/kbknapp/clap-rs/commit/18dbcf37024daf2b76ca099a6f118b53827aa339), closes [#51](https://github.com/kbknapp/clap-rs/issues/51))



<a name="v0.5.8"></a>
## v0.5.8 (2015-04-08)


#### Bug Fixes

* **option args**  fix bug in getting the wrong number of occurrences for options ([82ad6ad7](https://github.com/kbknapp/clap-rs/commit/82ad6ad77539cf9f9a03b78db466f575ebd972cc))
* **help**  fix formatting for option arguments with no long ([e8691004](https://github.com/kbknapp/clap-rs/commit/e869100423d93fa3acff03c4620cbcc0d0e790a1))
* **flags**  add assertion to catch flags with specific value sets ([a0a2a40f](https://github.com/kbknapp/clap-rs/commit/a0a2a40fed57f7c5ad9d68970d090e9856306c7d), closes [#52](https://github.com/kbknapp/clap-rs/issues/52))
* **args**  improve error messages for arguments with mutual exclusions ([bff945fc](https://github.com/kbknapp/clap-rs/commit/bff945fc5d03bba4266533340adcffb002508d1b), closes [#51](https://github.com/kbknapp/clap-rs/issues/51))
* **tests**  add missing .takes_value(true) to option2 ([bdb0e88f](https://github.com/kbknapp/clap-rs/commit/bdb0e88f696c8595c3def3bfb0e52d538c7be085))
* **positional args**  all previous positional args become required when a latter one is required ([343d47dc](https://github.com/kbknapp/clap-rs/commit/343d47dcbf83786a45c0d0f01b27fd9dd76725de), closes [#50](https://github.com/kbknapp/clap-rs/issues/50))



<a name="v0.5.7"></a>
## v0.5.7 (2015-04-08)


#### Bug Fixes

* **args**  fix bug in arguments who are required and mutually exclusive ([6ceb88a5](https://github.com/kbknapp/clap-rs/commit/6ceb88a594caae825605abc1cdad95204996bf29))



<a name="v0.5.6"></a>
## v0.5.6 (2015-04-08)


#### Bug Fixes

* **help**  fix formatting of help and usage ([28691b52](https://github.com/kbknapp/clap-rs/commit/28691b52f67e65c599e10e4ea2a0f6f9765a06b8))



<a name="v0.5.5"></a>
## v0.5.5 (2015-04-08)


#### Bug Fixes

* **help**  fix formatting of help for flags and options ([6ec10115](https://github.com/kbknapp/clap-rs/commit/6ec1011563a746f0578a93b76d45e63878e0f9a8))



<a name="v0.5.4"></a>
## v0.5.4 (2015-04-08)


#### Features

* **help**  add '...' to indicate multiple values supported ([297ddba7](https://github.com/kbknapp/clap-rs/commit/297ddba77000e2228762ab0eca50b480f7467386))



<a name="v0.5.3"></a>
## v0.5.3 (2015-04-08)


#### Features

* **positionals**
  *  add assertions for positional args with multiple vals ([b7fa72d4](https://github.com/kbknapp/clap-rs/commit/b7fa72d40f18806ec2042dd67a518401c2cf5681))
  *  add support for multiple values ([80784009](https://github.com/kbknapp/clap-rs/commit/807840094109fbf90b348039ae22669ef27889ba))



<a name="v0.5.2"></a>
## v0.5.2 (2015-04-08)


#### Bug Fixes

* **apps**  allow use of hyphens in application and subcommand names ([da549dcb](https://github.com/kbknapp/clap-rs/commit/da549dcb6c7e0d773044ab17829744483a8b0f7f))



<a name="v0.5.1"></a>
## v0.5.1 (2015-04-08)


#### Bug Fixes

* **args**  determine if the only arguments allowed are also required ([0a09eb36](https://github.com/kbknapp/clap-rs/commit/0a09eb365ced9a03faf8ed24f083ef730acc90e8))



<a name="v0.5.0"></a>
## v0.5.0 (2015-04-08)


#### Features

* **args**  add support for a specific set of allowed values on options or positional arguments ([270eb889](https://github.com/kbknapp/clap-rs/commit/270eb88925b6dc2881bff1f31ee344f085d31809))



<a name="v0.4.18"></a>
## v0.4.18 (2015-04-08)


#### Bug Fixes

* **usage**  display required args in usage, even if only required by others ([1b7316d4](https://github.com/kbknapp/clap-rs/commit/1b7316d4a8df70b0aa584ccbfd33f68966ad2a54))

#### Features

* **subcommands**  properly list subcommands in help and usage ([4ee02344](https://github.com/kbknapp/clap-rs/commit/4ee023442abc3dba54b68138006a52b714adf331))



<a name="v0.4.17"></a>
## v0.4.17 (2015-04-08)


#### Bug Fixes

* **tests**  remove cargo test from claptests makefile ([1cf73817](https://github.com/kbknapp/clap-rs/commit/1cf73817d6fb1dccb5b6a23b46c2efa8b567ad62))



<a name="v0.4.16"></a>
## v0.4.16 (2015-04-08)


#### Bug Fixes

* **option**  fix bug with option occurrence values ([9af52e93](https://github.com/kbknapp/clap-rs/commit/9af52e93cef9e17ac9974963f132013d0b97b946))
* **tests**  fix testing script bug and formatting ([d8f03a55](https://github.com/kbknapp/clap-rs/commit/d8f03a55c4f74d126710ee06aad5a667246a8001))

#### Features

* **arg**  allow lifetimes other than 'static in arguments ([9e8c1fb9](https://github.com/kbknapp/clap-rs/commit/9e8c1fb9406f8448873ca58bab07fe905f1551e5))



