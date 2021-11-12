use clap::{App, AppSettings};

fn main() {
    let app = App::new(env!("CARGO_CRATE_NAME"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(App::new("hostname").about("show hostname part of FQDN"))
        .subcommand(App::new("dnsdomainname").about("show domain name part of FQDN"));

    #[cfg(feature = "unstable-multicall")]
    let app = app.setting(AppSettings::Multicall);

    match app.get_matches().subcommand_name() {
        Some("hostname") => println!("www"),
        Some("dnsdomainname") => println!("example.com"),
        _ => unreachable!("parser should ensure only valid subcommand names are used"),
    }
}
