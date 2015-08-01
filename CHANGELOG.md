<a name="v1.1.6"></a>
### v1.1.6 (2015-08-01)


#### Bug Fixes

*   fixes two bugs in App when printing newlines in help and subcommands required error ([d63c0136](https://github.com/kbknapp/clap-rs/commit/d63c0136310db9dd2b1c7b4745938311601d8938))



<a name="v1.1.5"></a>
### v1.1.5 (2015-07-29)

#### Performance

*   removes some unneeded allocations ([93e915df](https://github.com/kbknapp/clap-rs/commit/93e915dfe300f7b7d6209ca93323c6a46f89a8c1))

<a name="v1.1.4"></a>
### v1.1.4 (2015-07-20)


#### Improvements

* **Usage Strings**  displays a [--] when it may be helpful ([86c3be85](https://github.com/kbknapp/clap-rs/commit/86c3be85fb6f77f83b5a6d2df40ae60937486984))

#### Bug Fixes

* **Macros**  fixes a typo in a macro generated error message ([c9195c5f](https://github.com/kbknapp/clap-rs/commit/c9195c5f92abb8cd6a37b4f4fbb2f1fee2a8e368))
* **Type Errors**  fixes formatting of error output when failed type parsing ([fe5d95c6](https://github.com/kbknapp/clap-rs/commit/fe5d95c64f3296e6eddcbec0cb8b86659800145f))



<a name="v1.1.3"></a>
### v1.1.3 (2015-07-18)


#### Documentation

*   updates README.md to include lack of color support on Windows ([52f81e17](https://github.com/kbknapp/clap-rs/commit/52f81e17377b18d2bd0f34693b642b7f358998ee))

#### Bug Fixes

*   fixes formatting bug which prevented compiling on windows ([9cb5dceb](https://github.com/kbknapp/clap-rs/commit/9cb5dceb3e5fe5e0e7b24619ff77e5040672b723), closes [#163](https://github.com/kbknapp/clap-rs/issues/163))



<a name="v1.1.2"></a>
### v1.1.2 (2015-07-17)


#### Bug Fixes

*   fixes a bug when parsing multiple {n} newlines inside help strings ([6d214b54](https://github.com/kbknapp/clap-rs/commit/6d214b549a9b7e189a94e5fa2b7c92cc333ca637))



<a name="v1.1.1"></a>
## v1.1.1 (2015-07-17)


#### Bug Fixes

*   fixes a logic bug and allows setting Arg::number_of_values() < 2 ([42b6d1fc](https://github.com/kbknapp/clap-rs/commit/42b6d1fc3c519c92dfb3af15276e7d3b635e6cfe), closes [#161](https://github.com/kbknapp/clap-rs/issues/161))



<a name="v1.1.0"></a>
## v1.1.0 (2015-07-16)


#### Features

*   allows creating unified help messages, a la docopt or getopts ([52bcd892](https://github.com/kbknapp/clap-rs/commit/52bcd892ea51564ce463bc5865acd64f8fe91cb1), closes [#158](https://github.com/kbknapp/clap-rs/issues/158))
*   allows stating all subcommands should *not* have --version flags ([336c476f](https://github.com/kbknapp/clap-rs/commit/336c476f631d512b54ac56fdca6f29ebdc2c00c5), closes [#156](https://github.com/kbknapp/clap-rs/issues/156))
*   allows setting version number to auto-propagate through subcommands ([bc66d3c6](https://github.com/kbknapp/clap-rs/commit/bc66d3c6deedeca62463fff95369ab1cfcdd366b), closes [#157](https://github.com/kbknapp/clap-rs/issues/157))

#### Improvements

* **Help Strings**  properly aligns and handles newlines in long help strings ([f9800a29](https://github.com/kbknapp/clap-rs/commit/f9800a29696dd2cc0b0284bf693b3011831e556f), closes [#145](https://github.com/kbknapp/clap-rs/issues/145))


#### Performance

* **Help Messages**  big performance improvements when printing help messages ([52bcd892](https://github.com/kbknapp/clap-rs/commit/52bcd892ea51564ce463bc5865acd64f8fe91cb1))

#### Documentation

*   updates readme with new features ([8232f7bb](https://github.com/kbknapp/clap-rs/commit/8232f7bb52e88862bc13c3d4f99ee4f56cfe4bc0))
*   fix incorrect code example for `App::subcommand_required` ([8889689d](https://github.com/kbknapp/clap-rs/commit/8889689dc6336ccc45b2c9f2cf8e2e483a639e93))


<a name="v1.0.3"></a>
### v1.0.3 (2015-07-11)


#### Improvements

* **Errors**  writes errors to stderr ([cc76ab8c](https://github.com/kbknapp/clap-rs/commit/cc76ab8c2b77c67b42f4717ded530df7806142cf), closes [#154](https://github.com/kbknapp/clap-rs/issues/154))

#### Documentation

* **README.md**  updates example help message to new format ([0aca29bd](https://github.com/kbknapp/clap-rs/commit/0aca29bd5d6d1a4e9971bdc88d946ffa58606efa))



<a name="v1.0.2"></a>
### v1.0.2 (2015-07-09)


#### Improvements

* **Usage**  re-orders optional arguments and required to natural standard ([dc7e1fce](https://github.com/kbknapp/clap-rs/commit/dc7e1fcea5c85d317018fb201d2a9262249131b4), closes [#147](https://github.com/kbknapp/clap-rs/issues/147))



<a name="v1.0.1"></a>
### v1.0.1 (2015-07-08)


#### Bug Fixes

*   allows empty values when using --long='' syntax ([083f82d3](https://github.com/kbknapp/clap-rs/commit/083f82d333b69720a6ef30074875310921d964d1), closes [#151](https://github.com/kbknapp/clap-rs/issues/151))



<a name="v1.0.0"></a>
## v1.0.0 (2015-07-08)


#### Documentation

* **README.md**  adds new features to what's new list ([938f7f01](https://github.com/kbknapp/clap-rs/commit/938f7f01340f521969376cf4e2e3d9436bca21f7))
* **README.md**  use with_name for subcommands ([28b7e316](https://github.com/kbknapp/clap-rs/commit/28b7e3161fb772e5309042648fe8c3a420645bac))

#### Features

*   args can now be parsed from arbitrary locations, not just std::env::args() ([75312528](https://github.com/kbknapp/clap-rs/commit/753125282b1b9bfff875f1557ce27610edcc59e1))



<a name="v1.0.0"></a>
## v1.0.0-beta (2015-06-30)


#### Features

*   allows waiting for user input on error ([d0da3bdd](https://github.com/kbknapp/clap-rs/commit/d0da3bdd9d1871541907ea9c645322a74d260e07), closes [#140](https://github.com/kbknapp/clap-rs/issues/140))
* **Help**  allows one to fully override the auto-generated help message ([26d5ae3e](https://github.com/kbknapp/clap-rs/commit/26d5ae3e330d1e150811d5b60b2b01a8f8df854e), closes [#141](https://github.com/kbknapp/clap-rs/issues/141))

#### Documentation

*   adds "whats new" section to readme ([ff149a29](https://github.com/kbknapp/clap-rs/commit/ff149a29dd9e179865e6d577cd7dc87c54f8f95c))

#### Improvements

*   removes deprecated functions in prep for 1.0 ([274484df](https://github.com/kbknapp/clap-rs/commit/274484dfd08fff4859cefd7e9bef3b73d3a9cb5f))



<a name="v0.11.0"></a>
## v0.11.0 (2015-06-17) - BREAKING CHANGE


#### Documentation

*   updates docs to new version flag defaults ([ebf442eb](https://github.com/kbknapp/clap-rs/commit/ebf442ebebbcd2ec6bfe2c06566c9d362bccb112))

#### Features

* **Help and Version**  default short for version is now `-V` but can be overridden (only breaks manual documentation) (**BREAKING CHANGE** [eb1d9320](https://github.com/kbknapp/clap-rs/commit/eb1d9320c509c1e4e57d7c7959da82bcfe06ada0))



<a name="v0.10.5"></a>
### v0.10.5 (2015-06-06)


#### Bug Fixes

* **Global Args**  global arguments propogate fully now ([1f377960](https://github.com/kbknapp/clap-rs/commit/1f377960a48c82f54ca5f39eb56bcb393140b046), closes [#137](https://github.com/kbknapp/clap-rs/issues/137))



<a name="v0.10.4"></a>
### v0.10.4 (2015-06-06)


#### Bug Fixes

* **Global Args**  global arguments propogate fully now ([8f2c0160](https://github.com/kbknapp/clap-rs/commit/8f2c0160c8d844daef375a33dbaec7d89de00a00), closes [#137](https://github.com/kbknapp/clap-rs/issues/137))



<a name="v0.10.3"></a>
### v0.10.3 (2015-05-31)


#### Bug Fixes

* **Global Args**  fixes a bug where globals only transfer to one subcommand ([a37842ee](https://github.com/kbknapp/clap-rs/commit/a37842eec1ee3162b86fdbda23420b221cdb1e3b), closes [#135](https://github.com/kbknapp/clap-rs/issues/135))



<a name="v0.10.2"></a>
### v0.10.2 (2015-05-30)


#### Improvements

* **Binary Names**  allows users to override the system determined bin name ([2191fe94](https://github.com/kbknapp/clap-rs/commit/2191fe94bda35771383b52872fb7f5421b178be1), closes [#134](https://github.com/kbknapp/clap-rs/issues/134))

#### Documentation

*   adds contributing guidelines ([6f76bd0a](https://github.com/kbknapp/clap-rs/commit/6f76bd0a07e8b7419b391243ab2d6687cd8a9c5f))



<a name="v0.10.1"></a>
### v0.10.1 (2015-05-26)


#### Features

*   can now specify that an app or subcommand should display help on no args or subcommands ([29ca7b2f](https://github.com/kbknapp/clap-rs/commit/29ca7b2f74376ca0cdb9d8ee3bfa99f7640cc404), closes [#133](https://github.com/kbknapp/clap-rs/issues/133))



<a name="v0.10.0"></a>
## v0.10.0 (2015-05-23)


#### Features

* **Global Args**  allows args that propagate down to child commands ([2bcc6137](https://github.com/kbknapp/clap-rs/commit/2bcc6137a83cb07757771a0afea953e68e692f0b), closes [#131](https://github.com/kbknapp/clap-rs/issues/131))

#### Improvements

* **Colors**  implements more structured colored output ([d6c3ed54](https://github.com/kbknapp/clap-rs/commit/d6c3ed54d21cf7b40d9f130d4280ff5448522fc5), closes [#129](https://github.com/kbknapp/clap-rs/issues/129))

#### Deprecations

* **SubCommand/App**  several methods and functions for stable release ([28b73855](https://github.com/kbknapp/clap-rs/commit/28b73855523ad170544afdb20665db98702fbe70))

#### Documentation

*   updates for deprecations and new features ([743eefe8](https://github.com/kbknapp/clap-rs/commit/743eefe8dd40c1260065ce086d572e9e9358bc4c))



<a name="v0.9.2"></a>
## v0.9.2 (2015-05-20)


#### Bug Fixes

* **help**  allows parent requirements to be ignored with help and version ([52218cc1](https://github.com/kbknapp/clap-rs/commit/52218cc1fdb06a42456c964d98cc2c7ac3432412), closes [#124](https://github.com/kbknapp/clap-rs/issues/124))



<a name="v0.9.1"></a>
## v0.9.1 (2015-05-18)


#### Bug Fixes

* **help**  fixes a bug where requirements are included as program name in help and version ([08ba3f25](https://github.com/kbknapp/clap-rs/commit/08ba3f25cf38b149229ba8b9cb37a5804fe6b789))



<a name="v0.9.0"></a>
## v0.9.0 (2015-05-17)


#### Improvements

* **usage**  usage strings now include parent command requirements ([dd8f21c7](https://github.com/kbknapp/clap-rs/commit/dd8f21c7c15cde348fdcf44fa7c205f0e98d2e4a), closes [#125](https://github.com/kbknapp/clap-rs/issues/125))
* **args**  allows consumer of clap to decide if empty values are allowed or not ([ab4ec609](https://github.com/kbknapp/clap-rs/commit/ab4ec609ccf692b9b72cccef5c9f74f5577e360d), closes [#122](https://github.com/kbknapp/clap-rs/issues/122))

#### Features

* **subcommands**
  *  allows optionally specifying that no subcommand is an error ([7554f238](https://github.com/kbknapp/clap-rs/commit/7554f238fd3afdd60b7e4dcf00ff4a9eccf842c1), closes [#126](https://github.com/kbknapp/clap-rs/issues/126))
  *  subcommands can optionally negate parent requirements ([4a4229f5](https://github.com/kbknapp/clap-rs/commit/4a4229f500e21c350e1ef78dd09ef27559653288), closes [#123](https://github.com/kbknapp/clap-rs/issues/123))



<a name="v0.8.6"></a>
## v0.8.6 (2015-05-17)


#### Bug Fixes

* **args**  `-` can now be parsed as a value for an argument ([bc12e78e](https://github.com/kbknapp/clap-rs/commit/bc12e78eadd7eaf9d008a8469fdd2dfd7990cb5d), closes [#121](https://github.com/kbknapp/clap-rs/issues/121))



<a name="v0.8.5"></a>
## v0.8.5 (2015-05-15)


#### Bug Fixes

* **macros**  makes macro errors consistent with others ([0c264a8c](https://github.com/kbknapp/clap-rs/commit/0c264a8ca57ec1cfdcb74dae79145d766cdc9b97), closes [#118](https://github.com/kbknapp/clap-rs/issues/118))

#### Features

* **macros**
  *  arg_enum! and simple_enum! provide a Vec<&str> of variant names ([30fa87ba](https://github.com/kbknapp/clap-rs/commit/30fa87ba4e0f3189351d8f4f78b72e616a30d0bd), closes [#119](https://github.com/kbknapp/clap-rs/issues/119))
  *  arg_enum! and simple_enum! auto-implement Display ([d1219f0d](https://github.com/kbknapp/clap-rs/commit/d1219f0d1371d872061bd0718057eca4ef47b739), closes [#120](https://github.com/kbknapp/clap-rs/issues/120))



<a name="v0.8.4"></a>
## v0.8.4 (2015-05-12)


#### Bug Fixes

* **suggestions**  --help and --version now get suggestions ([d2b3b1fa](https://github.com/kbknapp/clap-rs/commit/d2b3b1faa0bdc1c5d2350cc4635aba81e02e9d96), closes [#116](https://github.com/kbknapp/clap-rs/issues/116))



<a name="v0.8.3"></a>
## v0.8.3 (2015-05-10)


#### Bug Fixes

* **usage**  groups unfold their members in usage strings ([55d15582](https://github.com/kbknapp/clap-rs/commit/55d155827ea4a6b077a83669701e797ce1ad68f4), closes [#114](https://github.com/kbknapp/clap-rs/issues/114))

#### Performance

* **usage**  removes unneeded allocations ([fd53cd18](https://github.com/kbknapp/clap-rs/commit/fd53cd188555f5c3dc8bc341c5d7eb04b761a70f))



<a name="v0.8.2"></a>
## v0.8.2 (2015-05-08)


#### Bug Fixes

* **usage strings**  positional arguments are presented in index order ([eb0e374e](https://github.com/kbknapp/clap-rs/commit/eb0e374ecf952f1eefbc73113f21e0705936e40b), closes [#112](https://github.com/kbknapp/clap-rs/issues/112))



<a name="v0.8.1"></a>
## v0.8.1 (2015-05-06)


#### Bug Fixes

* **subcommands**  stops parsing multiple values when subcommands are found ([fc79017e](https://github.com/kbknapp/clap-rs/commit/fc79017eced04fd41cc1801331e5054df41fac17), closes [#109](https://github.com/kbknapp/clap-rs/issues/109))

#### Improvements

* **color**  reduces color in error messages ([aab44cca](https://github.com/kbknapp/clap-rs/commit/aab44cca6352f47e280c296e50c535f5d752dd46), closes [#110](https://github.com/kbknapp/clap-rs/issues/110))
* **suggestions**  adds suggested arguments to usage strings ([99447414](https://github.com/kbknapp/clap-rs/commit/994474146e9fb8b701af773a52da71553d74d4b7))



<a name="v0.8.0"></a>
## v0.8.0 (2015-05-06)


#### Bug Fixes

* **did-you-mean**  for review ([0535cfb0](https://github.com/kbknapp/clap-rs/commit/0535cfb0c711331568b4de8080eeef80bd254b68))
* **Positional**  positionals were ignored if they matched a subcmd, even after '--' ([90e7b081](https://github.com/kbknapp/clap-rs/commit/90e7b0818741668b47cbe3becd029bab588e3553))
* **help**  fixes bug where space between arg and help is too long ([632fb115](https://github.com/kbknapp/clap-rs/commit/632fb11514c504999ea86bdce47cdd34f8ebf646))

#### Features

* **from_usage**  adds ability to add value names or num of vals in usage string ([3d581976](https://github.com/kbknapp/clap-rs/commit/3d58197674ed7886ca315efb76e411608a327501), closes [#98](https://github.com/kbknapp/clap-rs/issues/98))
* **did-you-mean**
  *  gate it behind 'suggestions' ([c0e38351](https://github.com/kbknapp/clap-rs/commit/c0e383515d01bdd5ca459af9c2f7e2cf49e2488b))
  *  for possible values ([1cc2deb2](https://github.com/kbknapp/clap-rs/commit/1cc2deb29158e0e4e8b434e4ce26b3d819301a7d))
  *  for long flags (i.e. --long) ([52a0b850](https://github.com/kbknapp/clap-rs/commit/52a0b8505c99354bdf5fd1cd256cf41197ac2d81))
  *  for subcommands ([06e869b5](https://github.com/kbknapp/clap-rs/commit/06e869b5180258047ed3c60ba099de818dd25fff))
* **Flags**  adds sugestions functionality ([8745071c](https://github.com/kbknapp/clap-rs/commit/8745071c3257dd327c497013516f12a823df9530))
* **errors**  colorizes output red on error ([f8b26b13](https://github.com/kbknapp/clap-rs/commit/f8b26b13da82ba3ba9a932d3d1ab4ea45d1ab036))

#### Improvements

* **arg_enum**  allows ascii case insensitivity for enum variants ([b249f965](https://github.com/kbknapp/clap-rs/commit/b249f9657c6921c004764bd80d13ebca81585eec), closes [#104](https://github.com/kbknapp/clap-rs/issues/104))
* **clap-test**  simplified `make test` invocation ([d17dcb29](https://github.com/kbknapp/clap-rs/commit/d17dcb2920637a1f58c61c596b7bd362fd53047c))

#### Documentation

* **README**  adds details about optional and new features ([960389de](https://github.com/kbknapp/clap-rs/commit/960389de02c9872aaee9adabe86987f71f986e39))
* **clap**  fix typos caught by codespell ([8891d929](https://github.com/kbknapp/clap-rs/commit/8891d92917aa1a069cca67272be41b99e548356e))
* **from_usage**  explains new usage strings with multiple values ([05476fc6](https://github.com/kbknapp/clap-rs/commit/05476fc61cd1e5f4a4e750d258c878732a3a9c64))



<a name="v0.7.6"></a>
## v0.7.6 (2015-05-05)


#### Improvements

* **Options**  adds number of values to options in help/usage ([c1c993c4](https://github.com/kbknapp/clap-rs/commit/c1c993c419d18e35c443785053d8de9a2ef88073))

#### Features

* **from_usage**  adds ability to add value names or num of vals in usage string ([ad55748c](https://github.com/kbknapp/clap-rs/commit/ad55748c265cf27935c7b210307d2040b6a09125), closes [#98](https://github.com/kbknapp/clap-rs/issues/98))

#### Bug Fixes

* **MultipleValues**  properly distinguishes between multiple values and multiple occurrences ([dd2a7564](https://github.com/kbknapp/clap-rs/commit/dd2a75640ca68a91b973faad15f04df891356cef), closes [#99](https://github.com/kbknapp/clap-rs/issues/99))
* **help**  fixes tab alignment with multiple values ([847001ff](https://github.com/kbknapp/clap-rs/commit/847001ff6d8f4d9518e810fefb8edf746dd0f31e))

#### Documentation

* **from_usage**  explains new usage strings with multiple values ([5a3a42df](https://github.com/kbknapp/clap-rs/commit/5a3a42dfa3a783537f88dedc0fd5f0edcb8ea372))



<a name="v0.7.5"></a>
## v0.7.5 (2015-05-04)


#### Bug Fixes

* **Options**  fixes bug where options with no value don't error out ([a1fb94be](https://github.com/kbknapp/clap-rs/commit/a1fb94be53141572ffd97aad037295d4ffec82d0))



<a name="v0.7.4"></a>
## v0.7.4 (2015-05-03)


#### Bug Fixes

* **Options**  fixes a bug where option arguments in succession get their values skipped ([f66334d0](https://github.com/kbknapp/clap-rs/commit/f66334d0ce984e2b56e5c19abb1dd536fae9342a))



<a name="v0.7.3"></a>
## v0.7.3 (2015-05-03)


#### Bug Fixes

* **RequiredValues**  fixes a bug where missing values are parsed as missing arguments ([93c4a723](https://github.com/kbknapp/clap-rs/commit/93c4a7231ba1a08152648598f7aa4503ea82e4de))

#### Improvements

* **ErrorMessages**  improves error messages and corrections ([a29c3983](https://github.com/kbknapp/clap-rs/commit/a29c3983c4229906655a29146ec15a0e46dd942d))
* **ArgGroups**  improves requirement and confliction support for groups ([c236dc5f](https://github.com/kbknapp/clap-rs/commit/c236dc5ff475110d2a1b80e62903f80296163ad3))



<a name="v0.7.2"></a>
## v0.7.2 (2015-05-03)


#### Bug Fixes

* **RequiredArgs**  fixes bug where required-by-default arguments are not listed in usage ([12aea961](https://github.com/kbknapp/clap-rs/commit/12aea9612d290845ba86515c240aeeb0a21198db), closes [#96](https://github.com/kbknapp/clap-rs/issues/96))



<a name="v0.7.1"></a>
## v0.7.1 (2015-05-01)


#### Bug Fixes

* **MultipleValues**  stops evaluating values if the max or exact number of values was reached ([86d92c9f](https://github.com/kbknapp/clap-rs/commit/86d92c9fdbf9f422442e9562977bbaf268dbbae1))



<a name="v0.7.0"></a>
## v0.7.0 (2015-04-30) - BREAKING CHANGE


#### Bug Fixes

* **from_usage**  removes bug where usage strings have no help text ([ad4e5451](https://github.com/kbknapp/clap-rs/commit/ad4e54510739aeabf75f0da3278fb0952db531b3), closes [#83](https://github.com/kbknapp/clap-rs/issues/83))

#### Features

* **MultipleValues**
  *  add support for minimum and maximum number of values ([53f6b8c9](https://github.com/kbknapp/clap-rs/commit/53f6b8c9d8dc408b4fa9f833fc3a63683873c42f))
  *  adds support limited number and named values ([ae09f05e](https://github.com/kbknapp/clap-rs/commit/ae09f05e92251c1b39a83d372736fcc7b504e432))
  *  implement shorthand for options with multiple values ([6669f0a9](https://github.com/kbknapp/clap-rs/commit/6669f0a9687d4f668523145d7bd5c007d1eb59a8))
* **arg**  allow other types besides Vec for multiple value settings (**BREAKING CHANGE** [0cc2f698](https://github.com/kbknapp/clap-rs/commit/0cc2f69839b9b1db5d06330771b494783049a88e), closes [#87](https://github.com/kbknapp/clap-rs/issues/87))
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



