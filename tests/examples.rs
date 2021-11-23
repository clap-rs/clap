#![cfg(not(tarpaulin))]

#[test]
fn example_tests() {
    let t = trycmd::TestCases::new();
    t.register_bins(trycmd::cargo::compile_examples([]).unwrap());
    t.case("examples/*.md");
    #[cfg(not(feature = "unstable-multicall"))]
    {
        t.skip("examples/24a_multicall_busybox.md");
        t.skip("examples/24b_multicall_hostname.md");
    }
}
