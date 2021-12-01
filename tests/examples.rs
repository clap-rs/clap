#![cfg(not(tarpaulin))]

#[test]
fn example_tests() {
    let t = trycmd::TestCases::new();
    let features = [
        #[cfg(feature = "debug")]
        "debug",
        #[cfg(feature = "doc")]
        "doc",
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
        #[cfg(feature = "wrap_help")]
        "wrap_help",
        #[cfg(feature = "unsable-replace")]
        "unsable-replace",
        #[cfg(feature = "unsable-multicall")]
        "unsable-multicall",
        #[cfg(feature = "unsable-grouped")]
        "unsable-grouped",
    ]
    .join(" ");
    t.register_bins(trycmd::cargo::compile_examples(["--features", &features]).unwrap());
    t.case("examples/**/*.md");
    #[cfg(not(feature = "unstable-multicall"))]
    {
        t.skip("examples/24a_multicall_busybox.md");
        t.skip("examples/24b_multicall_hostname.md");
    }
}
