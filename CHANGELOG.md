<a name="v2.1.0"></a>
## v2.1.0 (2016-02-10)


#### Features

* **Defult Values:**  adds support for default values in args ([73211952](https://github.com/kbknapp/clap-rs/commit/73211952964a79d97b434dd567e6d7d34be7feb5), closes [#418](https://github.com/kbknapp/clap-rs/issues/418))

#### Documentation

* **Default Values:**  adds better examples and notes for default values ([9facd74f](https://github.com/kbknapp/clap-rs/commit/9facd74f843ef3807c5d35259558a344e6c25905))


<a name="v2.0.6"></a>
### v2.0.6 (2016-02-09)


#### Improvements

* **Positional Arguments:**  now displays value name if appropriate ([f0a99916](https://github.com/kbknapp/clap-rs/commit/f0a99916c59ce675515c6dcdfe9a40b130510908), closes [#420](https://github.com/kbknapp/clap-rs/issues/420))


<a name="v2.0.5"></a>
### v2.0.5 (2016-02-05)


#### Bug Fixes

* **Multiple Values:**  fixes bug where number_of_values wasnt respected ([72c387da](https://github.com/kbknapp/clap-rs/commit/72c387da0bb8a6f526f863770f08bb8ca0d3de03))


<a name="v2.0.4"></a>
### v2.0.4 (2016-02-04)


#### Bug Fixes

*   adds support for building ArgGroups from standalone YAML ([fcbc7e12](https://github.com/kbknapp/clap-rs/commit/fcbc7e12f5d7b023b8f30cba8cad28a01cf6cd26))
*   Stop lonely hyphens from causing panic ([85b11468](https://github.com/kbknapp/clap-rs/commit/85b11468b0189d5cc15f1cfac5db40d17a0077dc), closes [#410](https://github.com/kbknapp/clap-rs/issues/410))
* **AppSettings:**  fixes bug where subcmds didn't receive parent ver ([a62e4527](https://github.com/kbknapp/clap-rs/commit/a62e452754b3b0e3ac9a15aa8b5330636229ead1))

<a name="v2.0.3"></a>
### v2.0.3 (2016-02-02)


#### Improvements

* **values:**  adds support for up to u64::max values per arg ([c7abf7d7](https://github.com/kbknapp/clap-rs/commit/c7abf7d7611e317b0d31d97632e3d2e13570947c))
* **occurrences:**  Allow for more than 256 occurrences of an argument. ([3731ddb3](https://github.com/kbknapp/clap-rs/commit/3731ddb361163f3d6b86844362871e48c80fa530))

#### Features

* **AppSettings:**  adds HidePossibleValuesInHelp to skip writing those values ([cdee7a0e](https://github.com/kbknapp/clap-rs/commit/cdee7a0eb2beeec723cb98acfacf03bf629c1da3))

#### Bug Fixes

* **value_t_or_exit:**  fixes typo which causes value_t_or_exit to return a Result ([ee96baff](https://github.com/kbknapp/clap-rs/commit/ee96baffd306cb8d20ddc5575cf739bb1a6354e8))


<a name="v2.0.2"></a>
### v2.0.2 (2016-01-31)


#### Improvements

* **arg_enum:**  enum declared with arg_enum returns [&'static str; #] instead of Vec ([9c4b8a1a](https://github.com/kbknapp/clap-rs/commit/9c4b8a1a6b12949222f17d1074578ad7676b9c0d))

#### Bug Fixes

*   clap_app! should be gated by unstable, not nightly feature ([0c8b84af](https://github.com/kbknapp/clap-rs/commit/0c8b84af6161d5baf683688eafc00874846f83fa))
* **SubCommands:**  fixed where subcmds weren't recognized after mult args ([c19c17a8](https://github.com/kbknapp/clap-rs/commit/c19c17a8850602990e24347aeb4427cf43316223), closes [#405](https://github.com/kbknapp/clap-rs/issues/405))
* **Usage Parser:**  fixes a bug where literal single quotes weren't allowed in help strings ([0bcc7120](https://github.com/kbknapp/clap-rs/commit/0bcc71206478074769e311479b34a9f74fe80f5c), closes [#406](https://github.com/kbknapp/clap-rs/issues/406))


<a name="v2.0.1"></a>
### v2.0.1 (2016-01-30)


#### Bug Fixes

*   fixes cargo features to NOT require nightly with unstable features ([dcbcc60c](https://github.com/kbknapp/clap-rs/commit/dcbcc60c9ba17894be636472ea4b07a82d86a9db), closes [#402](https://github.com/kbknapp/clap-rs/issues/402))


<a name="v2.0.0"></a>
## v2.0.0 (2016-01-28)


#### Improvements

* **From Usage:**  vastly improves the usage parser ([fa3a2f86](https://github.com/kbknapp/clap-rs/commit/fa3a2f86bd674c5eb07128c95098fab7d1437247), closes [#350](https://github.com/kbknapp/clap-rs/issues/350))

#### Features

*   adds support for external subcommands ([177fe5cc](https://github.com/kbknapp/clap-rs/commit/177fe5cce745c2164a8e38c23be4c4460d2d7211), closes [#372](https://github.com/kbknapp/clap-rs/issues/372))
*   adds support values with a leading hyphen ([e4d429b9](https://github.com/kbknapp/clap-rs/commit/e4d429b9d52e95197bd0b572d59efacecf305a59), closes [#385](https://github.com/kbknapp/clap-rs/issues/385))
*   adds support for turning off the value delimiter ([508db850](https://github.com/kbknapp/clap-rs/commit/508db850a87c2e251cf6b6ddead9ad56b29f9e57), closes [#352](https://github.com/kbknapp/clap-rs/issues/352))
*   adds support changing the value delimiter ([dafeae8a](https://github.com/kbknapp/clap-rs/commit/dafeae8a526162640f6a68da434370c64d190889), closes [#353](https://github.com/kbknapp/clap-rs/issues/353))
*   adds support for comma separated values ([e69da6af](https://github.com/kbknapp/clap-rs/commit/e69da6afcd2fe48a3c458ca031db40997f860eda), closes [#348](https://github.com/kbknapp/clap-rs/issues/348))
*   adds support with options with optional values ([4555736c](https://github.com/kbknapp/clap-rs/commit/4555736cad01441dcde4ea84a285227e0844c16e), closes [#367](https://github.com/kbknapp/clap-rs/issues/367))
* **UTF-8:**  adds support for invalid utf8 in values ([c5c59dec](https://github.com/kbknapp/clap-rs/commit/c5c59dec0bc33b86b2e99d30741336f17ec84282), closes [#269](https://github.com/kbknapp/clap-rs/issues/269))
* **v2:**  implementing the base of 2.x ([a3536054](https://github.com/kbknapp/clap-rs/commit/a3536054512ba833533dc56615ce3663d884381c))

#### Bug Fixes

*   fixes nightly build with new lints ([17599195](https://github.com/kbknapp/clap-rs/commit/175991956c37dc83ba9c49396e927a1cb65c5b11))
*   fixes Windows build for 2x release ([674c9b48](https://github.com/kbknapp/clap-rs/commit/674c9b48c7c92079cb180cc650a9e39f34781c32), closes [#392](https://github.com/kbknapp/clap-rs/issues/392))
*   fixes yaml build for 2x base ([adceae64](https://github.com/kbknapp/clap-rs/commit/adceae64c8556d00ab715677377b216f9f468ad7))

#### Documentation

*   updates examples for 2x release ([1303b360](https://github.com/kbknapp/clap-rs/commit/1303b3607468f362ab1b452d5614c1a064dc69b4), closes [#394](https://github.com/kbknapp/clap-rs/issues/394))
*   updates examples for 2x release ([0a011f31](https://github.com/kbknapp/clap-rs/commit/0a011f3142aec338d388a6c8bfe22fa7036021bb), closes [#394](https://github.com/kbknapp/clap-rs/issues/394))
*   updates documentation for v2 release ([8d51724e](https://github.com/kbknapp/clap-rs/commit/8d51724ef73dfde5bb94fb9466bc5463a1cc1502))
*   updating docs for 2x release ([576d0e0e](https://github.com/kbknapp/clap-rs/commit/576d0e0e2c7b8f386589179bbf7419b93abacf1c))
* **README.md:**
  *  updates readme for v2 release ([acaba01a](https://github.com/kbknapp/clap-rs/commit/acaba01a353c12144b9cd9a3ce447400691849b0), closes [#393](https://github.com/kbknapp/clap-rs/issues/393))
  *  fix typo and make documentation conspicuous ([07b9f614](https://github.com/kbknapp/clap-rs/commit/07b9f61495d927f69f7abe6c0d85253f0f4e6107))

#### BREAKING CHANGES

* **Fewer liftimes! Yay!**
 * `App<'a, 'b, 'c, 'd, 'e, 'f>` => `App<'a, 'b>`
 * `Arg<'a, 'b, 'c, 'd, 'e, 'f>` => `Arg<'a, 'b>`
 * `ArgMatches<'a, 'b>` => `ArgMatches<'a>`
* **Simply Renamed**
 * `App::arg_group` => `App::group`
 * `App::arg_groups` => `App::groups`
 * `ArgGroup::add` => `ArgGroup::arg`
 * `ArgGroup::add_all` => `ArgGroup::args`
 * `ClapError` => `Error`
  * struct field `ClapError::error_type` => `Error::kind`
 * `ClapResult` => `Result`
 * `ClapErrorType` => `ErrorKind`
* **Removed Deprecated Functions and Methods**
 * `App::subcommands_negate_reqs`
 * `App::subcommand_required`
 * `App::arg_required_else_help`
 * `App::global_version(bool)`
 * `App::versionless_subcommands`
 * `App::unified_help_messages`
 * `App::wait_on_error`
 * `App::subcommand_required_else_help`
 * `SubCommand::new`
 * `App::error_on_no_subcommand`
 * `Arg::new`
 * `Arg::mutually_excludes`
 * `Arg::mutually_excludes_all`
 * `Arg::mutually_overrides_with`
 * `simple_enum!`
* **Renamed Error Variants**
 * `InvalidUnicode` => `InvalidUtf8`
 * `InvalidArgument` => `UnknownArgument`
* **Usage Parser**
 * Value names can now be specified inline, i.e. `-o, --option <FILE> <FILE2> 'some option which takes two files'`
 * **There is now a priority of order to determine the name** - This is perhaps the biggest breaking change. See the documentation for full details. Prior to this change, the value name took precedence. **Ensure your args are using the proper names (i.e. typically the long or short and NOT the value name) throughout the code**
* `ArgMatches::values_of` returns an `Values` now which implements `Iterator` (should not break any code)
* `crate_version!` returns `&'static str` instead of `String`
* Using the `clap_app!` macro requires compiling with the `unstable` feature because the syntax could change slightly in the future


<a name="v1.5.5"></a>
### v1.5.5 (2016-01-04)


#### Bug Fixes

*   fixes an issue where invalid short args didn't cause an error ([c9bf7e44](https://github.com/kbknapp/clap-rs/commit/c9bf7e4440bd2f9b524ea955311d433c40a7d1e0))
*   prints the name in version and help instead of binary name ([8f3817f6](https://github.com/kbknapp/clap-rs/commit/8f3817f665c0cab6726bc16c56a53b6a61e44448), closes [#368](https://github.com/kbknapp/clap-rs/issues/368))
*   fixes an intentional panic issue discovered via clippy ([ea83a3d4](https://github.com/kbknapp/clap-rs/commit/ea83a3d421ea8856d4cac763942834d108b71406))


<a name="v1.5.4"></a>
### v1.5.4 (2015-12-18)


#### Examples

* **17_yaml:**  conditinonally compile 17_yaml example ([575de089](https://github.com/kbknapp/clap-rs/commit/575de089a3e240c398cb10e6cf5a5c6b68662c01))

#### Improvements

*   clippy improvements ([99cdebc2](https://github.com/kbknapp/clap-rs/commit/99cdebc23da3a45a165f14b27bebeb2ed828a2ce))

#### Bug Fixes


* **errors:**  return correct error type in WrongNumValues error builder ([5ba8ba9d](https://github.com/kbknapp/clap-rs/commit/5ba8ba9dcccdfa74dd1c44260e64b359bbb36be6))
*   ArgRequiredElseHelp setting now takes precedence over missing required args ([faad83fb](https://github.com/kbknapp/clap-rs/commit/faad83fbef6752f3093b6e98fca09a9449b830f4), closes [#362](https://github.com/kbknapp/clap-rs/issues/362))


<a name="v1.5.3"></a>
### v1.5.3 (2015-11-20)


#### Bug Fixes

* **Errors:**  fixes some instances when errors are missing a final newline ([c4d2b171](https://github.com/kbknapp/clap-rs/commit/c4d2b1711994479ad64ee52b6b49d2ceccbf2118))




<a name="v1.5.2"></a>
### v1.5.2 (2015-11-14)


#### Bug Fixes

* **Errors:**  fixes a compiling bug when built on Windows or without the color feature ([a35f7634](https://github.com/kbknapp/clap-rs/commit/a35f76346fe6ecc88dda6a1eb13627186e7ce185))



<a name="v1.5.1"></a>
### v1.5.1 (2015-11-13)


#### Bug Fixes

* **Required Args:**  fixes a bug where required args are not correctly accounted for ([f03b88a9](https://github.com/kbknapp/clap-rs/commit/f03b88a9766b331a63879bcd747687f2e5a2661b), closes [#343](https://github.com/kbknapp/clap-rs/issues/343))



<a name="v1.5.0"></a>
## v1.5.0 (2015-11-13)


#### Bug Fixes

*   fixes a bug with required positional args in usage strings ([c6858f78](https://github.com/kbknapp/clap-rs/commit/c6858f78755f8e860204323c828c8355a066dc83))

#### Documentation

* **FAQ:**  updates readme with slight changes to FAQ ([a4ef0fab](https://github.com/kbknapp/clap-rs/commit/a4ef0fab73c8dc68f1b138965d1340459c113398))

#### Improvements

*   massive errors overhaul ([cdc29175](https://github.com/kbknapp/clap-rs/commit/cdc29175bc9c53e5b4aec86cbc04c1743154dae6))
* **ArgMatcher:**  huge refactor and deduplication of code ([8988853f](https://github.com/kbknapp/clap-rs/commit/8988853fb8825e8f841fde349834cc12cdbad081))
* **Errors:**  errors have been vastly improved ([e59bc0c1](https://github.com/kbknapp/clap-rs/commit/e59bc0c16046db156a88ba71a037db05028e995c))
* **Traits:**  refactoring some configuration into traits ([5800cdec](https://github.com/kbknapp/clap-rs/commit/5800cdec6dce3def4242b9f7bd136308afb19685))

#### Performance

* **App:**
  *  more BTreeMap->Vec, Opts and SubCmds ([bc4495b3](https://github.com/kbknapp/clap-rs/commit/bc4495b32ec752b6c4b29719e831c043ef2a26ce))
  *  changes flags BTreeMap->Vec ([d357640f](https://github.com/kbknapp/clap-rs/commit/d357640fab55e5964fe83efc3c771e53aa3222fd))
  *  removed unneeded BTreeMap ([78971fd6](https://github.com/kbknapp/clap-rs/commit/78971fd68d7dc5c8e6811b4520cdc54e4188f733))
  *  changes BTreeMap to VecMap in some instances ([64b921d0](https://github.com/kbknapp/clap-rs/commit/64b921d087fdd03775c95ba0bcf65d3f5d36f812))
  *  removed excess clones ([ec0089d4](https://github.com/kbknapp/clap-rs/commit/ec0089d42ed715d293fb668d3a90b0db0aa3ec39))



<a name="v1.4.7"></a>
### v1.4.7 (2015-11-03)


#### Documentation

*   Clarify behavior of Arg::multiple with options. ([434f497a](https://github.com/kbknapp/clap-rs/commit/434f497ab6d831f8145cf09278c97ca6ee6c6fe7))
*   Fix typos and improve grammar. ([c1f66b5d](https://github.com/kbknapp/clap-rs/commit/c1f66b5de7b5269fbf8760a005ef8c645edd3229))

#### Bug Fixes

* **Error Status:**  fixes bug where --help and --version return non-zero exit code ([89b51fdf](https://github.com/kbknapp/clap-rs/commit/89b51fdf8b1ab67607567344e2317ff1a757cb12))



<a name="v1.4.6"></a>
### v1.4.6 (2015-10-29)


#### Features

*   allows parsing without a binary name for daemons and interactive CLIs ([aff89d57](https://github.com/kbknapp/clap-rs/commit/aff89d579b5b85c3dc81b64f16d5865299ec39a2), closes [#318](https://github.com/kbknapp/clap-rs/issues/318))

#### Bug Fixes

* **Errors:**  tones down quoting in some error messages ([34ce59ed](https://github.com/kbknapp/clap-rs/commit/34ce59ede53bfa2eef722c74881cdba7419fd9c7), closes [#309](https://github.com/kbknapp/clap-rs/issues/309))
* **Help and Version:**  only builds help and version once ([e3be87cf](https://github.com/kbknapp/clap-rs/commit/e3be87cfc095fc41c9811adcdc6d2b079f237d5e))
* **Option Args:**  fixes bug with args and multiple values ([c9a9548a](https://github.com/kbknapp/clap-rs/commit/c9a9548a8f96cef8a3dd9a980948325fbbc1b91b), closes [#323](https://github.com/kbknapp/clap-rs/issues/323))
* **POSIX Overrides:**  fixes bug where required args are overridden ([40ed2b50](https://github.com/kbknapp/clap-rs/commit/40ed2b50c3a9fe88bfdbaa43cef9fd6493ecaa8e))
* **Safe Matches:**  using 'safe' forms of the get_matches family no longer exit the process ([c47025dc](https://github.com/kbknapp/clap-rs/commit/c47025dca2b3305dea0a0acfdd741b09af0c0d05), closes [#256](https://github.com/kbknapp/clap-rs/issues/256))
* **Versionless SubCommands:**  fixes a bug where the -V flag was needlessly built ([27df8b9d](https://github.com/kbknapp/clap-rs/commit/27df8b9d98d13709dad3929a009f40ebff089a1a), closes [#329](https://github.com/kbknapp/clap-rs/issues/329))

#### Documentation

*   adds comparison in readme ([1a8bf31e](https://github.com/kbknapp/clap-rs/commit/1a8bf31e7a6b87ce48a66af2cde1645b2dd5bc95), closes [#325](https://github.com/kbknapp/clap-rs/issues/325))



<a name="v1.4.5"></a>
### v1.4.5 (2015-10-06)


#### Bug Fixes

*   fixes crash on invalid arg error ([c78ce128](https://github.com/kbknapp/clap-rs/commit/c78ce128ebbe7b8f730815f8176c29d76f4ade8c))



<a name="v1.4.4"></a>
### v1.4.4 (2015-10-06)


#### Documentation

*   clean up some formatting ([b7df92d7](https://github.com/kbknapp/clap-rs/commit/b7df92d7ea25835701dd22ddff984b9749f48a00))
*   move the crate-level docs to top of the lib.rs file ([d7233bf1](https://github.com/kbknapp/clap-rs/commit/d7233bf122dbf80ba8fc79e5641be2df8af10e7a))
*   changes doc comments to rustdoc comments ([34b601be](https://github.com/kbknapp/clap-rs/commit/34b601be5fdde76c1a0859385b359b96d66b8732))
*   fixes panic in 14_groups example ([945b00a0](https://github.com/kbknapp/clap-rs/commit/945b00a0c27714b63bdca48d003fe205fcfdc578), closes [#295](https://github.com/kbknapp/clap-rs/issues/295))
*   avoid suggesting star dependencies. ([d33228f4](https://github.com/kbknapp/clap-rs/commit/d33228f40b5fefb84cf3dd51546bfb340dcd9f5a))
* **Rustdoc:**  adds portions of the readme to main rustdoc page ([6f9ee181](https://github.com/kbknapp/clap-rs/commit/6f9ee181e69d90bd4206290e59d6f3f1e8f0cbb2), closes [#293](https://github.com/kbknapp/clap-rs/issues/293))

#### Bug Fixes

*   grammar error in some conflicting option errors ([e73b07e1](https://github.com/kbknapp/clap-rs/commit/e73b07e19474323ad2260da66abbf6a6d4ecbd4f))
* **Unified Help:**  sorts both flags and options as a unified category ([2a223dad](https://github.com/kbknapp/clap-rs/commit/2a223dad82901fa2e74baad3bfc4c7b94509300f))
* **Usage:**  fixes a bug where required args aren't filtered properly ([72b453dc](https://github.com/kbknapp/clap-rs/commit/72b453dc170af3050bb123d35364f6da77fc06d7), closes [#277](https://github.com/kbknapp/clap-rs/issues/277))
* **Usage Strings:**  fixes a bug ordering of elements in usage strings ([aaf0d6fe](https://github.com/kbknapp/clap-rs/commit/aaf0d6fe7aa2403e76096c16204d254a9ee61ee2), closes [#298](https://github.com/kbknapp/clap-rs/issues/298))

#### Features

*   supports -aValue style options ([0e3733e4](https://github.com/kbknapp/clap-rs/commit/0e3733e4fec2015c2d566a51432dcd92cb69cad3))
* **Trailing VarArg:**  adds opt-in setting for final arg being vararg ([27018b18](https://github.com/kbknapp/clap-rs/commit/27018b1821a4bcd5235cfe92abe71b3c99efc24d), closes [#278](https://github.com/kbknapp/clap-rs/issues/278))



<a name="v1.4.3"></a>
### v1.4.3 (2015-09-30)


#### Features

*   allows accessing arg values by group name ([c92a4b9e](https://github.com/kbknapp/clap-rs/commit/c92a4b9eff2d679957f61c0c41ff404b40d38a91))

#### Documentation

*   use links to examples instead of plain text ([bb4fe237](https://github.com/kbknapp/clap-rs/commit/bb4fe237858535627271465147add537e4556b43))

#### Bug Fixes

* **Help Message:**  required args no longer double list in usage ([1412e639](https://github.com/kbknapp/clap-rs/commit/1412e639e0a79df84936d1101a837f90077d1c83), closes [#277](https://github.com/kbknapp/clap-rs/issues/277))
* **Possible Values:**  possible value validation is restored ([f121ae74](https://github.com/kbknapp/clap-rs/commit/f121ae749f8f4bfe754ef2e8a6dfc286504b5b75), closes [#287](https://github.com/kbknapp/clap-rs/issues/287))



<a name="v1.4.2"></a>
### v1.4.2 (2015-09-23)


#### Bug Fixes

* **Conflicts:**  fixes bug with conflicts not removing required args ([e17fcec5](https://github.com/kbknapp/clap-rs/commit/e17fcec53b3216ad047a13dddc6f740473fad1a1), closes [#271](https://github.com/kbknapp/clap-rs/issues/271))



<a name="v1.4.1"></a>
### v1.4.1 (2015-09-22)


#### Examples

*   add clap_app quick example ([4ba6249c](https://github.com/kbknapp/clap-rs/commit/4ba6249c3cf4d2e083370d1fe4dcc7025282c28a))

#### Features

* **Unicode:**  allows non-panicing on invalid unicode characters ([c5bf7ddc](https://github.com/kbknapp/clap-rs/commit/c5bf7ddc8cfb876ec928a5aaf5591232bbb32e5d))

#### Documentation

*   properly names Examples section for rustdoc ([87ba5445](https://github.com/kbknapp/clap-rs/commit/87ba54451d7ec7b1c9b9ef134f90bbe39e6fac69))
*   fixes various typos and spelling ([f85640f9](https://github.com/kbknapp/clap-rs/commit/f85640f9f6d8fd3821a40e9b8b7a34fabb789d02))
* **Arg:**  unhides fields of the Arg struct ([931aea88](https://github.com/kbknapp/clap-rs/commit/931aea88427edf43a3da90d5a500c1ff2b2c3614))

#### Bug Fixes

*   flush the buffer in App::print_version() ([cbc42a37](https://github.com/kbknapp/clap-rs/commit/cbc42a37d212d84d22b1777d08e584ff191934e7))
*   Macro benchmarks ([13712da1](https://github.com/kbknapp/clap-rs/commit/13712da1d36dc7614eec3a10ad488257ba615751))



<a name="v1.4.0"></a>
## v1.4.0 (2015-09-09)


#### Features

*   allows printing help message by library consumers ([56b95f32](https://github.com/kbknapp/clap-rs/commit/56b95f320875c62dda82cb91b29059671e120ed1))
*   allows defining hidden args and subcmds ([2cab4d03](https://github.com/kbknapp/clap-rs/commit/2cab4d0334ea3c2439a1d4bfca5bf9905c7ea9ac), closes [#231](https://github.com/kbknapp/clap-rs/issues/231))
*   Builder macro to assist with App/Arg/Group/SubCommand building ([443841b0](https://github.com/kbknapp/clap-rs/commit/443841b012a8d795cd5c2bd69ae6e23ef9b16477))
* **Errors:**  allows consumers to write to stderr and exit on error ([1e6403b6](https://github.com/kbknapp/clap-rs/commit/1e6403b6a863574fa3cb6946b1fb58f034e8664c))



<a name="v1.3.2"></a>
### v1.3.2 (2015-09-08)


#### Documentation

*   fixed ErrorKind docs ([dd057843](https://github.com/kbknapp/clap-rs/commit/dd05784327fa070eb6ce5ce89a8507e011d8db94))
* **ErrorKind:**  changed examples content ([b9ca2616](https://github.com/kbknapp/clap-rs/commit/b9ca261634b89613bbf3d98fd74d55cefbb31a8c))

#### Bug Fixes

*   fixes a bug where the help subcommand wasn't overridable ([94003db4](https://github.com/kbknapp/clap-rs/commit/94003db4b5eebe552ca337521c1c001295822745))

#### Features

*   adds abiltiy not consume self when parsing matches and/or exit on help ([94003db4](https://github.com/kbknapp/clap-rs/commit/94003db4b5eebe552ca337521c1c001295822745))
* **App:**  Added ability for users to handle errors themselves ([934e6fbb](https://github.com/kbknapp/clap-rs/commit/934e6fbb643b2385efc23444fe6fce31494dc288))



<a name="v1.3.1"></a>
### v1.3.1 (2015-09-04)


#### Examples

* **17_yaml:**  fixed example ([9b848622](https://github.com/kbknapp/clap-rs/commit/9b848622296c8c5c7b9a39b93ddd41f51df790b5))

#### Performance

*   changes ArgGroup HashSets to Vec ([3cb4a48e](https://github.com/kbknapp/clap-rs/commit/3cb4a48ebd15c20692f4f3a2a924284dc7fd5e10))
*   changes BTreeSet for Vec in some instances ([baab2e3f](https://github.com/kbknapp/clap-rs/commit/baab2e3f4060e811abee14b1654cbcd5cf3b5fea))



<a name="v1.3.0"></a>
## v1.3.0 (2015-09-01)


#### Features

* **YAML:**  allows building a CLI from YAML files ([86cf4c45](https://github.com/kbknapp/clap-rs/commit/86cf4c45626a36b8115446952f9069f73c1debc3))
* **ArgGroups:**  adds support for building ArgGroups from yaml ([ecf88665](https://github.com/kbknapp/clap-rs/commit/ecf88665cbff367018b29161a1b75d44a212707d))
* **Subcommands:**  adds support for subcommands from yaml ([e415cf78](https://github.com/kbknapp/clap-rs/commit/e415cf78ba916052d118a8648deba2b9c16b1530))

#### Documentation

* **YAML:**  adds examples for using YAML to build a CLI ([ab41d7f3](https://github.com/kbknapp/clap-rs/commit/ab41d7f38219544750e6e1426076dc498073191b))
* **Args from YAML:**  fixes doc examples ([19b348a1](https://github.com/kbknapp/clap-rs/commit/19b348a10050404cd93888dbbbe4f396681b67d0))
* **Examples:**  adds better usage examples instead of having unused variables ([8cbacd88](https://github.com/kbknapp/clap-rs/commit/8cbacd8883004fe71a8ea036ec4391c7dd8efe94))

#### Examples

*   Add AppSettings example ([12705079](https://github.com/kbknapp/clap-rs/commit/12705079ca96a709b4dd94f7ddd20a833b26838c))

#### Bug Fixes

* **Unified Help Messages:**  fixes a crash from this setting and no opts ([169ffec1](https://github.com/kbknapp/clap-rs/commit/169ffec1003d58d105d7ef2585b3425e57980000), closes [#210](https://github.com/kbknapp/clap-rs/issues/210))



<a name="v1.2.5"></a>
### v1.2.5 (2015-08-27)


#### Examples

*   add custom validator example ([b9997d1f](https://github.com/kbknapp/clap-rs/commit/b9997d1fca74d4d8f93971f2a01bdf9798f913d5))
*   fix indentation ([d4f1b740](https://github.com/kbknapp/clap-rs/commit/d4f1b740ede410fd2528b9ecd89592c2fd8b1e20))

#### Features

* **Args:**  allows opts and args to define a name for help and usage msgs ([ad962ec4](https://github.com/kbknapp/clap-rs/commit/ad962ec478da999c7dba0afdb84c266f4d09b1bd))



<a name="v1.2.4"></a>
### v1.2.4 (2015-08-26)


#### Bug Fixes

* **Possible Values:**  fixes a bug where suggestions arent made when using --long=value format ([3d5e9a6c](https://github.com/kbknapp/clap-rs/commit/3d5e9a6cedb26668839b481c9978e2fbbab8be6f), closes [#192](https://github.com/kbknapp/clap-rs/issues/192))



<a name="v1.2.3"></a>
### v1.2.3 (2015-08-24)


#### Bug Fixes

* **App, Args:**  fixed subcommand reqs negation ([b41afa8c](https://github.com/kbknapp/clap-rs/commit/b41afa8c3ded3d1be12f7a2f8ea06cc44afc9458), closes [#188](https://github.com/kbknapp/clap-rs/issues/188))



<a name="v1.2.2"></a>
### v1.2.2 (2015-08-23)


#### Bug Fixes

*   fixed confusing error message, also added test for it ([fc7a31a7](https://github.com/kbknapp/clap-rs/commit/fc7a31a745efbf1768ee2c62cd3bb72bfe30c708))
* **App:**  fixed requirmets overriding ([9c135eb7](https://github.com/kbknapp/clap-rs/commit/9c135eb790fa16183e5bdb2009ddc3cf9e25f99f))



<a name="v1.2.1"></a>
### v1.2.1 (2015-08-20)


#### Documentation

* **README.md:**  updates for new features ([16cf9245](https://github.com/kbknapp/clap-rs/commit/16cf9245fb5fc4cf6face898e358368bf9961cbb))

#### Features

*   implements posix compatible conflicts for long args ([8c2d48ac](https://github.com/kbknapp/clap-rs/commit/8c2d48acf5473feebd721a9049a9c9b7051e70f9))
*   added overrides to support conflicts in POSIX compatible manner ([0b916a00](https://github.com/kbknapp/clap-rs/commit/0b916a00de26f6941538f6bc5f3365fa302083c1))
* **Args:**  allows defining POSIX compatible argument conflicts ([d715646e](https://github.com/kbknapp/clap-rs/commit/d715646e69759ccd95e01f49b04f489827ecf502))

#### Bug Fixes

*   fixed links in cargo and license buttons ([6d9837ad](https://github.com/kbknapp/clap-rs/commit/6d9837ad9a9e006117cd7372fdc60f9a3889c7e2))

#### Performance

* **Args and Apps:**  changes HashSet->Vec in some instances for increased performance ([d0c3b379](https://github.com/kbknapp/clap-rs/commit/d0c3b379700757e0a9b0c40af709f8af1f5b4949))



<a name="v1.2.0"></a>
### v1.2.0 (2015-08-15)


#### Bug Fixes

*   fixed misspell and enum name ([7df170d7](https://github.com/kbknapp/clap-rs/commit/7df170d7f4ecff06608317655d1e0c4298f62076))
*   fixed use for clap crate ([dc3ada73](https://github.com/kbknapp/clap-rs/commit/dc3ada738667d4b689678f79d14251ee82004ece))

#### Documentation

*   updates docs for new features ([03496547](https://github.com/kbknapp/clap-rs/commit/034965471782d872ca495045b58d34b31807c5b1))
*   fixed docs for previous changes ([ade36778](https://github.com/kbknapp/clap-rs/commit/ade367780c366425de462506d256e0f554ed3b9c))

#### Improvements

* **AppSettings:**  adds ability to add multiple settings at once ([4a00e251](https://github.com/kbknapp/clap-rs/commit/4a00e2510d0ca8d095d5257d51691ba3b61c1374))

#### Features

*   Replace application level settings with enum variants ([618dc4e2](https://github.com/kbknapp/clap-rs/commit/618dc4e2c205bf26bc43146164e65eb1f6b920ed))
* **Args:**  allows for custom argument value validations to be defined ([84ae2ddb](https://github.com/kbknapp/clap-rs/commit/84ae2ddbceda34b5cbda98a6959edaa52fde2e1a), closes [#170](https://github.com/kbknapp/clap-rs/issues/170))



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



