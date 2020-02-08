use version_sync::assert_html_root_url_updated;

#[test]
fn test_html_root_url() {
    assert_html_root_url_updated!("src/lib.rs");
}
