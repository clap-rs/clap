#[test]
fn example_tests() {
    let t = trycmd::TestCases::new();
    let features: &[&str] = &[
        #[cfg(feature = "unstable-dynamic")]
        "unstable-dynamic",
    ];
    let features = features.join(" ");
    t.register_bins(trycmd::cargo::compile_examples(["--features", &features]).unwrap());
    t.case("examples/**/*.md");
}
