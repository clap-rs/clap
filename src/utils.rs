use std::env;
use std::path::Path;

/// Get the name of the binary of the top level command, if it exists
pub fn get_bin_name() -> Option<String> {
    if let Some(name) = env::args().next() {
        let bn_os = name.into();
        let p = Path::new(&*bn_os);
        p.file_name().map(|f| f.to_os_string().to_str().to_owned())
    } else {
        None
    }
}
