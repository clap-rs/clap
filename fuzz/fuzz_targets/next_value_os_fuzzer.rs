#![no_main]

use libfuzzer_sys::fuzz_target;
use fuzzed_data_provider_rs::FuzzedDataProvider;
use std::ffi::OsString;
use clap_lex::RawArgs;

fuzz_target!(|data: &[u8]| {
    let mut fdp = FuzzedDataProvider::new(data);

    // Construct payload
    let mut input = vec![OsString::from("bin")];
    while fdp.remaining_bytes() > 0 {
        input.push(OsString::from(fdp.consume_random_length_string(fdp.remaining_bytes())));
    }

    let raw = RawArgs::new(input);
    let mut cursor = raw.cursor();
    let _ = raw.next_os(&mut cursor);

    while let Some(parsed) = raw.next(&mut cursor) {
        if let Some(mut short) = parsed.to_short() {
            while !short.is_empty() {
                let next = short.next_value_os();
                if next.is_some() {
                    let _ = next.unwrap();
                }
            }
        }
    }
});

