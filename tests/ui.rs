#![cfg(not(tarpaulin))]

#[test]
#[cfg(feature = "help")]
#[cfg(feature = "error-context")]
#[cfg(feature = "usage")]
fn ui_tests() {
    let t = trycmd::TestCases::new();
    let features = [
        // Default
        #[cfg(feature = "std")]
        "std",
        #[cfg(feature = "color")]
        "color",
        #[cfg(feature = "suggestions")]
        "suggestions",
        // Optional
        #[cfg(feature = "derive")]
        "derive",
        #[cfg(feature = "cargo")]
        "cargo",
        #[cfg(feature = "wrap_help")]
        "wrap_help",
        #[cfg(feature = "env")]
        "env",
        #[cfg(feature = "unicode")]
        "unicode",
        #[cfg(feature = "string")]
        "string",
        // In-work
    ]
    .join(" ");
    t.register_bins(trycmd::cargo::compile_examples(["--features", &features]).unwrap());
    t.case("tests/ui/*.toml");
}
