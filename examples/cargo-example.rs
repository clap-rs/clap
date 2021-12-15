fn main() {
    let app = clap::App::new("cargo")
        .bin_name("cargo")
        .setting(clap::AppSettings::SubcommandRequired)
        .subcommand(
            clap::app_from_crate!().name("example").arg(
                clap::arg!(--"manifest-path" <PATH>)
                    .required(false)
                    .allow_invalid_utf8(true),
            ),
        );
    let matches = app.get_matches();
    let matches = match matches.subcommand() {
        Some(("example", matches)) => matches,
        _ => unreachable!("clap should ensure we don't get here"),
    };
    let manifest_path = matches
        .value_of_os("manifest-path")
        .map(std::path::PathBuf::from);
    println!("{:?}", manifest_path);
}
