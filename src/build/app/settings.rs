// Std
use std::ops::BitOr;
use std::str::FromStr;

bitflags! {
    struct AppFlags: u8 {
        const WAIT_ON_ERROR        = 1 << 1;
        const NO_BIN_NAME          = 1 << 2;
    }
}

impl_settings! { AppFlags, AppSettings,
    NoBinaryName => Flags::NO_BIN_NAME,
    WaitOnError => Flags::WAIT_ON_ERROR,
}

/// Application level settings, which affect how [`App`] operates
///
/// [`App`]: ./struct.App.html
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AppSettings {
    /// Specifies that the parser should not assume the first argument passed is the binary name.
    /// This is normally the case when using a "daemon" style mode, or an interactive CLI where one
    /// one would not normally type the binary or program name for each command.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings};
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::NoBinaryName)
    ///     .arg(Arg::from("<cmd>... 'commands to run'"))
    ///     .get_matches_from(vec!["command", "set"]);
    ///
    /// let cmds: Vec<&str> = m.values_of("cmd").unwrap().collect();
    /// assert_eq!(cmds, ["command", "set"]);
    /// ```
    NoBinaryName,

    /// Will display a message "Press \[ENTER\]/\[RETURN\] to continue..." and wait for user before
    /// exiting
    ///
    /// This is most useful when writing an application which is run from a GUI shortcut, or on
    /// Windows where a user tries to open the binary by double-clicking instead of using the
    /// command line.
    ///
    /// **NOTE:** This setting is **not** recursive with [``]s, meaning if you wish this
    /// behavior for all subcommands, you must set this on each command (needing this is extremely
    /// rare)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::WaitOnError)
    /// # ;
    /// ```
    /// [``]: ./struct..html
    WaitOnError,
}

impl FromStr for AppSettings {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match &*s.to_ascii_lowercase() {
            "nobinaryname" => Ok(AppSettings::NoBinaryName),
            "waitonerror" => Ok(AppSettings::WaitOnError),
            _ => Err("unknown AppSetting, cannot convert from str".to_owned()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::AppSettings;

    #[test]
    fn app_settings_fromstr() {
        assert_eq!(
            "nobinaryname".parse::<AppSettings>().unwrap(),
            AppSettings::NoBinaryName
        );
        assert_eq!(
            "waitonerror".parse::<AppSettings>().unwrap(),
            AppSettings::WaitOnError
        );
        assert!("hahahaha".parse::<AppSettings>().is_err());
    }
}
