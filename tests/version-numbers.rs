#[macro_use]
extern crate version_sync;

#[test]
fn test_readme_deps() {
    assert_markdown_deps_updated!("README.md");
}

#[test]
fn test_html_root_url() {
    assert_html_root_url_updated!("src/lib.rs");
}

#[test]
fn test_changelog_updated() {
    assert_contains_regex!("CHANGELOG.md", r"^###? v{version} \(\d{4}-\d\d-\d\d\)");
}
