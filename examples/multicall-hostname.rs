// Note: this requires the `unstable-multicall` feature

use clap::App;

fn main() {
    let app = App::new(env!("CARGO_CRATE_NAME"))
        .arg_required_else_help(true)
        .subcommand_value_name("APPLET")
        .subcommand_help_heading("APPLETS")
        .subcommand(App::new("hostname").about("show hostname part of FQDN"))
        .subcommand(App::new("dnsdomainname").about("show domain name part of FQDN"));

    let app = app.multicall(true);

    match app.get_matches().subcommand_name() {
        Some("hostname") => println!("www"),
        Some("dnsdomainname") => println!("example.com"),
        _ => unreachable!("parser should ensure only valid subcommand names are used"),
    }
}
