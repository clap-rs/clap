#![cfg(not(tarpaulin))]

#[test]
#[cfg(feature = "help")]
#[cfg(feature = "error-context")]
#[cfg(feature = "usage")]
fn example_tests() {
    let t = trycmd::TestCases::new();
    let features = [
        #[cfg(feature = "std")]
        "std",
        #[cfg(feature = "derive")]
        "derive",
        #[cfg(feature = "cargo")]
        "cargo",
        #[cfg(feature = "color")]
        "color",
        #[cfg(feature = "env")]
        "env",
        #[cfg(feature = "suggestions")]
        "suggestions",
        #[cfg(feature = "unicode")]
        "unicode",
        #[cfg(feature = "string")]
        "string",
        #[cfg(feature = "wrap_help")]
        "wrap_help",
        #[cfg(feature = "unstable-replace")]
        "unstable-replace",
    ]
    .join(" ");
    t.register_bins(trycmd::cargo::compile_examples(["--features", &features]).unwrap());
    t.case("examples/**/*.md");
}
