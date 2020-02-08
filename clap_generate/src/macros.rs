macro_rules! w {
    ($buf:expr, $to_w:expr) => {
        match $buf.write_all($to_w) {
            Ok(..) => (),
            Err(..) => panic!("Failed to write to generated file"),
        }
    };
}

macro_rules! get_zsh_arg_conflicts {
    ($app:expr, $arg:ident, $msg:ident) => {
        if let Some(ref conf_vec) = $arg.blacklist {
            let mut v = vec![];

            for arg_name in conf_vec {
                let arg = find!($app, arg_name).expect($msg);

                if let Some(s) = arg.short {
                    v.push(format!("-{}", s));
                }

                if let Some(l) = arg.long {
                    v.push(format!("--{}", l));
                }
            }

            v.join(" ")
        } else {
            String::new()
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

macro_rules! find {
    ($app:expr, $name:expr, $what:ident) => {
        $what!($app).find(|a| &a.name == $name)
    };
    ($app:expr, $name:expr) => {
        $app.args.args.iter().find(|a| {
            if let Some(v) = a.index {
                &v == $name
            } else {
                false
            }
        })
    };
}
