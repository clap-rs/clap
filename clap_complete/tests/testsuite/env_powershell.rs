#![cfg(feature = "unstable-dynamic")]

use clap_complete::env::{EnvCompleter as _, Powershell};

fn registration() -> String {
    let mut buf = Vec::new();
    Powershell
        .write_registration("COMPLETE", "demo", "demo", "demo --complete", &mut buf)
        .unwrap();
    String::from_utf8(buf).unwrap()
}

/// Under legacy native argument passing (Windows PowerShell 5.1 and PowerShell
/// 7.0-7.2) an empty `''` argument is dropped, so the empty current word never
/// reaches the program and the previous word is completed again. A quoted `""`
/// does reach the program, so it must be used when `$PSNativeCommandArgumentPassing`
/// is absent (before 7.3) or explicitly set to `Legacy`.
#[test]
fn empty_word_uses_quoted_argument_for_legacy_passing() {
    let reg = registration();
    assert!(
        reg.contains("$PSNativeCommandArgumentPassing"),
        "registration must detect legacy argument passing:\n{reg}"
    );
    assert!(
        reg.contains(r#"$args += ' ""';"#),
        "legacy passing needs a quoted \"\" argument:\n{reg}"
    );
}

/// PowerShell 7.3+ and later (and `Windows`/`Standard` passing modes) already
/// forward `''` as an empty argument, so the original form must be kept.
#[test]
fn modern_passing_keeps_original_form() {
    let reg = registration();
    assert!(
        reg.contains(r#"$args += " ''";"#),
        "modern passing must keep the original '' form:\n{reg}"
    );
}
