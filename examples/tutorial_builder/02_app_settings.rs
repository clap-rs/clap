use clap::{app_from_crate, arg, AppSettings};

fn main() {
    let matches = app_from_crate!()
        .global_setting(AppSettings::AllArgsOverrideSelf)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::AllowNegativeNumbers)
        .arg(arg!(--two <VALUE>))
        .arg(arg!(--one <VALUE>))
        .get_matches();

    println!("two: {:?}", matches.value_of("two").expect("required"));
    println!("one: {:?}", matches.value_of("one").expect("required"));
}
