// `clap_generate_fig` is distributed under the terms of both the MIT license and the Apache License
// (Version 2.0).
// See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files in this repository
// for more information.

//! Generates [Fig](https://github.com/withfig/autocomplete) completions for [`clap`](https://github.com/clap-rs/clap) based CLIs

#![doc(html_logo_url = "https://clap.rs/images/media/clap.png")]
#![doc(html_root_url = "https://docs.rs/clap_generate_fig/3.0.0-rc.0")]
#![deny(missing_docs, trivial_casts, unused_allocation, trivial_numeric_casts)]
#![forbid(unsafe_code)]
#![allow(clippy::needless_doctest_main)]

mod fig;
pub use fig::Fig;
