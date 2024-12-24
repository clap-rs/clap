#![no_main]

use libfuzzer_sys::fuzz_target;
use fuzzed_data_provider_rs::FuzzedDataProvider;
use std::ffi::OsString;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    test: String,

    #[arg(short, long)]
    param: String,

    #[arg(short, long)]
    name: String,
}

fuzz_target!(|data: &[u8]| {
    let mut fdp = FuzzedDataProvider::new(data);
    let _ = Args::try_parse_from(vec![
        OsString::from(fdp.consume_random_length_string(fdp.remaining_bytes())),
        OsString::from(fdp.consume_random_length_string(fdp.remaining_bytes())),
        OsString::from(fdp.consume_random_length_string(fdp.remaining_bytes())),
    ]);
});

