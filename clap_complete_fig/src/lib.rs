// `clap_complete_fig` is distributed under the terms of both the MIT license and the Apache License
// (Version 2.0).
// See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files in this repository
// for more information.

//! Generates [Fig](https://github.com/withfig/autocomplete) completions for [`clap`](https://github.com/clap-rs/clap) based CLIs

#![doc(html_logo_url = "https://raw.githubusercontent.com/clap-rs/clap/master/assets/clap.png")]
#![warn(missing_docs, trivial_casts, unused_allocation, trivial_numeric_casts)]
#![forbid(unsafe_code)]
#![allow(clippy::needless_doctest_main)]

mod fig;
pub use fig::Fig;
