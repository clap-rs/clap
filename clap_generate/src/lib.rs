use std::ffi::OsString;
use std::io::Error;
use std::io::Write;
use std::path::PathBuf;

#[deprecated(since = "3.0.0", note = "Renamed to `clap_complete`")]
pub use clap_complete::Generator;
#[deprecated(since = "3.0.0", note = "Renamed to `clap_complete`")]
pub use clap_complete::Shell;

#[deprecated(since = "3.0.0", note = "Renamed to `clap_complete::generators`")]
pub mod generators {
    pub use clap_complete::generator::*;
    pub use clap_complete::shells::*;
}

#[deprecated(since = "3.0.0", note = "Renamed to `clap_complete::utils`")]
pub mod utils {
    pub use clap_complete::generator::utils::*;
}

#[deprecated(since = "3.0.0", note = "Renamed to `clap_complete::generate_to`")]
pub fn generate_to<G, S, T>(
    gen: G,
    cmd: &mut clap::Command,
    bin_name: S,
    out_dir: T,
) -> Result<PathBuf, Error>
where
    G: Generator,
    S: Into<String>,
    T: Into<OsString>,
{
    clap_complete::generate_to(gen, cmd, bin_name, out_dir)
}

#[deprecated(since = "3.0.0", note = "Renamed to `clap_complete`")]
pub fn generate<G, S>(gen: G, cmd: &mut clap::Command, bin_name: S, buf: &mut dyn Write)
where
    G: Generator,
    S: Into<String>,
{
    clap_complete::generate(gen, cmd, bin_name, buf)
}
