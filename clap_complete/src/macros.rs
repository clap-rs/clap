macro_rules! w {
    ($buf:expr, $to_w:expr) => {
        match $buf.write_all($to_w) {
            Ok(..) => (),
            Err(err) if err.kind() == std::io::ErrorKind::BrokenPipe => {
                // Is there a better exit code for this?
                std::process::exit(1);
            }
            Err(err) => {
                panic!("Failed to write to generated file: {0}", err);
            }
        }
    };
}

#[cfg(feature = "debug")]
macro_rules! debug {
    ($($arg:tt)*) => {
        eprint!("[{:>w$}] \t", module_path!(), w = 28);
        eprintln!($($arg)*)
    }
}

#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($($arg:tt)*) => {};
}
