//! To run this test, use:
//! ```
//! RUSTFLAGS="--cfg=nightly" cargo +nightly test --test default_field_values --features derive --features help --features usage
//! ```
#![cfg(nightly)]
#![feature(default_field_values)]
#![cfg(feature = "derive")]
#![cfg(feature = "help")]
#![cfg(feature = "usage")]

mod tests;
