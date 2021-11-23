use crate::Arg;

// This test will *fail to compile* if Arg is not Send + Sync
#[test]
fn arg_send_sync() {
    fn foo<T: Send + Sync>(_: T) {}
    foo(Arg::new("test"))
}
