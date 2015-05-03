<a name="v0.7.3"></a>
## v0.7.3 (2015-05-03)


#### Bug Fixes

* **RequiredValues**  fixes a bug where missing values are parsed as missing arguments ([93c4a723](https://github.com/kbknapp/clap-rs/commit/93c4a7231ba1a08152648598f7aa4503ea82e4de))

#### Improvements

* **ErrorMessages**  improves error messages and corrections ([a29c3983](https://github.com/kbknapp/clap-rs/commit/a29c3983c4229906655a29146ec15a0e46dd942d))
* **ArgGroups**  improves requirment and confliction support for groups ([c236dc5f](https://github.com/kbknapp/clap-rs/commit/c236dc5ff475110d2a1b80e62903f80296163ad3))



<a name="v0.7.2"></a>
## v0.7.2 (2015-05-03)


#### Bug Fixes

* **RequiredArgs**  fixes bug where required-by-default arguments are not listed in usage ([12aea961](https://github.com/kbknapp/clap-rs/commit/12aea9612d290845ba86515c240aeeb0a21198db), closes [#96](https://github.com/kbknapp/clap-rs/issues/96))



<a name="v0.7.1"></a>
## v0.7.1 (2015-05-01)


#### Bug Fixes

* **MultipleValues**  stops evaluating values if the max or exact number of values was reached ([86d92c9f](https://github.com/kbknapp/clap-rs/commit/86d92c9fdbf9f422442e9562977bbaf268dbbae1))



<a name="v0.7.0"></a>
## v0.7.0 (2015-04-30)


#### Bug Fixes

* **from_usage**  removes bug where usage strings have no help text ([ad4e5451](https://github.com/kbknapp/clap-rs/commit/ad4e54510739aeabf75f0da3278fb0952db531b3), closes [#83](https://github.com/kbknapp/clap-rs/issues/83))

#### Features

* **MultipleValues**
  *  add support for minimum and maximum number of values ([53f6b8c9](https://github.com/kbknapp/clap-rs/commit/53f6b8c9d8dc408b4fa9f833fc3a63683873c42f))
  *  adds support limited number and named values ([ae09f05e](https://github.com/kbknapp/clap-rs/commit/ae09f05e92251c1b39a83d372736fcc7b504e432))
  *  implement shorthand for options with multiple values ([6669f0a9](https://github.com/kbknapp/clap-rs/commit/6669f0a9687d4f668523145d7bd5c007d1eb59a8))
* **arg**  allow other types besides Vec for multiple value settings ([0cc2f698](https://github.com/kbknapp/clap-rs/commit/0cc2f69839b9b1db5d06330771b494783049a88e), closes [#87](https://github.com/kbknapp/clap-rs/issues/87))
* **usage**  implement smart usage strings on errors ([d77048ef](https://github.com/kbknapp/clap-rs/commit/d77048efb1e595ffe831f1a2bea2f2700db53b9f), closes [#88](https://github.com/kbknapp/clap-rs/issues/88))



<a name="v0.6.9"></a>
## v0.6.9 (2015-04-29)


#### Bug Fixes

* **from_usage**  removes bug where usage strings have no help text ([ad4e5451](https://github.com/kbknapp/clap-rs/commit/ad4e54510739aeabf75f0da3278fb0952db531b3), closes [#83](https://github.com/kbknapp/clap-rs/issues/83))



<a name="0.6.8"></a>
## 0.6.8 (2015-04-27)


#### Bug Fixes

* **help**  change long help --long=long -> --long <long> ([1e25abfc](https://github.com/kbknapp/clap-rs/commit/1e25abfc36679ab89eae71bf98ced4de81992d00))
* **RequiredArgs**  required by default args should no longer be required when their exclusions are present ([4bb4c3cc](https://github.com/kbknapp/clap-rs/commit/4bb4c3cc076b49e86720e882bf8c489877199f2d))

#### Features

* **ArgGroups**  add ability to create arg groups ([09eb4d98](https://github.com/kbknapp/clap-rs/commit/09eb4d9893af40c347e50e2b717e1adef552357d))



<a name="v0.6.7"></a>
## v0.6.7 (2015-04-22)


#### Bug Fixes

* **from_usage**  fix bug causing args to not be required ([b76129e9](https://github.com/kbknapp/clap-rs/commit/b76129e9b71a63365d5c77a7f57b58dbd1e94d49))

#### Features

* **apps**  add ability to display additional help info after auto-gen'ed help msg ([65cc259e](https://github.com/kbknapp/clap-rs/commit/65cc259e4559cbe3653c865ec0c4b1e42a389b07))



<a name="v0.6.6"></a>
## v0.6.6 (2015-04-19)


#### Bug Fixes

* **from_usage**  tabs and spaces should be treated equally ([4fd44181](https://github.com/kbknapp/clap-rs/commit/4fd44181d55d8eb88caab1e625231cfa3129e347))

#### Features

* **macros.rs**  add macro to get version from Cargo.toml ([c630969a](https://github.com/kbknapp/clap-rs/commit/c630969aa3bbd386379219cae27ba1305b117f3e))



<a name="v0.6.5"></a>
## v0.6.5 (2015-04-19)


#### Bug Fixes

* **macros.rs**  fix use statements for trait impls ([86e4075e](https://github.com/kbknapp/clap-rs/commit/86e4075eb111937c8a7bdb344e866e350429f042))



<a name="v0.6.4"></a>
## v0.6.4 (2015-04-17)


#### Features

* **macros**  add ability to create enums pub or priv with derives ([2c499f80](https://github.com/kbknapp/clap-rs/commit/2c499f8015a199827cdf1fa3ec4f6f171722f8c7))



<a name="v0.6.3"></a>
## v0.6.3 (2015-04-16)


#### Features

* **macros**  add macro to create custom enums to use as types ([fb672aff](https://github.com/kbknapp/clap-rs/commit/fb672aff561c29db2e343d6c607138f141aca8b6))



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



