macro_rules! w {
    ($buf:expr, $to_w:expr) => {
        match $buf.write_all($to_w) {
            Ok(..) => (),
            Err(..) => panic!("Failed to write to generated file"),
        }
    };
}

#[cfg(feature = "debug")]
#[cfg_attr(feature = "debug", macro_use)]
#[cfg_attr(feature = "debug", allow(unused_macros))]
mod debug_macros {
    macro_rules! debugln {
        ($fmt:expr) => (println!(concat!("DEBUG:clap_generate:", $fmt)));
        ($fmt:expr, $($arg:tt)*) => (println!(concat!("DEBUG:clap_generate:",$fmt), $($arg)*));
    }
    macro_rules! sdebugln {
        ($fmt:expr) => (println!($fmt));
        ($fmt:expr, $($arg:tt)*) => (println!($fmt, $($arg)*));
    }
    macro_rules! debug {
        ($fmt:expr) => (print!(concat!("DEBUG:clap_generate:", $fmt)));
        ($fmt:expr, $($arg:tt)*) => (print!(concat!("DEBUG:clap_generate:",$fmt), $($arg)*));
    }
    macro_rules! sdebug {
        ($fmt:expr) => (print!($fmt));
        ($fmt:expr, $($arg:tt)*) => (print!($fmt, $($arg)*));
    }
}

#[cfg(not(feature = "debug"))]
#[cfg_attr(not(feature = "debug"), macro_use)]
#[cfg_attr(not(feature = "debug"), allow(unused_macros))]
mod debug_macros {
    macro_rules! debugln {
        ($fmt:expr) => {};
        ($fmt:expr, $($arg:tt)*) => {};
    }
    macro_rules! sdebugln {
        ($fmt:expr) => {};
        ($fmt:expr, $($arg:tt)*) => {};
    }
    macro_rules! debug {
        ($fmt:expr) => {};
        ($fmt:expr, $($arg:tt)*) => {};
    }
}
