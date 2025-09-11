use clap::Args;

#[derive(Args, Debug)]
pub(crate) struct ImplicitParsers {
    /// Implicitly using `std::str::FromStr`
    #[arg(short = 'O')]
    optimization: Option<usize>,

    /// Allow invalid UTF-8 paths
    #[arg(short = 'I', value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    include: Option<std::path::PathBuf>,

    /// Handle IP addresses
    #[arg(long)]
    bind: Option<std::net::IpAddr>,

    /// Allow human-readable durations
    #[arg(long)]
    sleep: Option<jiff::SignedDuration>,
}
