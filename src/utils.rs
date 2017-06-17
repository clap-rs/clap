use std::ffi::OsString;
use std::env;
use std::path::Path;

/// Get the name of the binary of the top level command, if it exists
///
/// # Panics
///
/// Panics if the binary name contains invalid UTF-8. If that is a possibility, prefer
/// [`clap::utils::get_bin_name_os`] instead.
///
/// [`clap::utils::get_bin_name_os`]: ./function.get_bin_name_os.html
pub fn get_bin_name() -> Option<String> {
    if let Some(bn_os) = env::args_os().next() {
        let p = Path::new(&*bn_os);
        p.file_name()
            .map(|f| f.to_os_string().to_str().unwrap().to_owned())
    } else {
        None
    }
}

/// Get the name of the binary of the top level command, if it exists
pub fn get_bin_name_os() -> Option<OsString> {
    if let Some(bn_os) = env::args_os().next() {
        let p = Path::new(&*bn_os);
        p.file_name().map(|f| f.to_os_string())
    } else {
        None
    }
}
