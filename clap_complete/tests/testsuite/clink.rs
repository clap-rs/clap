use crate::common;

#[test]
fn basic() {
    let name = "my-app";
    let cmd = common::basic_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/basic.lua"],
        clap_complete::shells::Clink,
        cmd,
        name,
    );
}
