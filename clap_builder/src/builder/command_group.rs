// Internal
use crate::builder::IntoResettable;
use crate::builder::Str;
use crate::util::Id;

/// Family of related [commands].
///
/// By placing commands in a logical group, you can make help and documentation easier to
/// understand and navigate.
///
/// # Examples
///
/// The following example demonstrates using a `CommandGroup` separate subcommands into two groups.
///
/// ```rust
/// # use clap_builder as clap;
/// # use clap::{Command, command, CommandGroup, error::ErrorKind};
/// let result = Command::new("git")
///     .subcommands([
///         Command::new("git-apply"),
///         Command::new("git-mktree"),
///         Command::new("git-cat-file"),
///         Command::new("git-cherry"),
///     ])
///     .command_group(CommandGroup::new("manipulation")
///			.help_heading("Manipulation commands")
///         .commands(["git-apply", "git-mktree"]))
///     .command_group(CommandGroup::new("interrogation")
///			.help_heading("Interrogation commands")
///         .commands(["git-cat-file", "git-cherry"]))
///
///     .try_get_matches_from(vec!["git", "git-apply"]);
/// assert!(result.is_ok());
/// ```
///
/// This next example demonstrates having only some commands belonging to a group
/// In the documentation, groups of commands will be displayed last,
/// while commands not belonging to any group will be displayed first.
/// ```rust
/// # use clap_builder as clap;
/// # use clap::{Command, arg, ArgGroup, Id};
/// let result = Command::new("cmd")
///     .subcommands([
///         Command::new("add"),
///         Command::new("remove"),
///         Command::new("show"),
///     ])
///     .command_group(CommandGroup::new("manipulation")
///			.help_heading("Manipulation commands")
///         .commands(["add", "remove"]))
///     .try_get_matches_from(vec!["cmd", "add"]);
/// assert!(result.is_ok());
/// ```
///
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct CommandGroup {
    pub(crate) id: Id,
    pub(crate) commands: Vec<Str>,
    pub(crate) heading: Option<Str>,
}

/// # Builder
impl CommandGroup {
    /// Create a `CommandGroup` using a unique name.
    ///
    /// The name will be used to get values from the group and to refer to the group
    /// by subcommands.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap_builder as clap;
    /// # use clap::{Command, ArgGroup};
    /// CommandGroup::new("config")
    /// # ;
    /// ```
    pub fn new(id: impl Into<Id>) -> Self {
        CommandGroup::default().id(id)
    }

    /// Sets the group name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap_builder as clap;
    /// # use clap::{Command, ArgGroup};
    /// CommandGroup::default().id("config")
    /// # ;
    /// ```
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = id.into();
        self
    }

    /// Adds an [command] to this group by name
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap_builder as clap;
    /// # use clap::{Command, CommandGroup};
    /// let m = Command::new("myprog")
 	///		.subcommands([
	///			Command::new("add"),
	///			Command::new("remove"),
	///			Command::new("show"),
	///		])
	///     .command_group(CommandGroup::new("manipulation")
	///			.help_heading("Manipulation commands")
	///         .command("add"))
    ///     .get_matches_from(vec!["myprog", "add"]);
    /// ```
    /// [argument]: crate::Str
    #[must_use]
    pub fn command(mut self, cmd_name: impl IntoResettable<Str>) -> Self {
        if let Some(cmd_name) = cmd_name.into_resettable().into_option() {
            self.commands.push(cmd_name);
        } else {
            self.commands.clear();
        }
        self
    }

    /// Adds multiple [commands] to this group by name
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap_builder as clap;
    /// # use clap::{Command, Arg, ArgGroup, ArgAction};
    /// let m = Command::new("myprog")
    ///     .arg(Arg::new("flag")
    ///         .short('f')
    ///         .action(ArgAction::SetTrue))
    ///     .arg(Arg::new("color")
    ///         .short('c')
    ///         .action(ArgAction::SetTrue))
    ///     .group(ArgGroup::new("req_flags")
    ///         .args(["flag", "color"]))
    ///     .get_matches_from(vec!["myprog", "-f"]);
    /// // maybe we don't know which of the two flags was used...
    /// assert!(m.contains_id("req_flags"));
    /// // but we can also check individually if needed
    /// assert!(m.contains_id("flag"));
    /// ```
    /// [commands]: crate::Arg
    #[must_use]
    pub fn commands(mut self, ns: impl IntoIterator<Item = impl Into<Str>>) -> Self {
        for n in ns {
            self = self.command(n);
        }
        self
    }


    #[must_use]
    pub fn help_heading(mut self, heading: impl IntoResettable<Str>) -> Self {
        self.heading = heading.into_resettable().into_option();
        self
    }


    /// Getters for all args. It will return a vector of `Id`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use clap_builder as clap;
    /// # use clap::{ArgGroup};
    /// let args: Vec<&str> = vec!["a1".into(), "a4".into()];
    /// let grp = ArgGroup::new("program").args(&args);
    ///
    /// for (pos, arg) in grp.get_args().enumerate() {
    ///     assert_eq!(*arg, args[pos]);
    /// }
    /// ```
    pub fn get_commands(&self) -> impl Iterator<Item = &Str> {
        self.commands.iter()
    }
}

/// # Reflection
impl CommandGroup {
    /// Get the name of the group
    #[inline]
    pub fn get_id(&self) -> &Id {
        &self.id
    }

}

impl From<&'_ CommandGroup> for CommandGroup {
    fn from(g: &CommandGroup) -> Self {
        g.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn groups() {
        let g = ArgGroup::new("test")
            .arg("a1")
            .arg("a4")
            .args(["a2", "a3"])
            .required(true)
            .conflicts_with("c1")
            .conflicts_with_all(["c2", "c3"])
            .conflicts_with("c4")
            .requires("r1")
            .requires_all(["r2", "r3"])
            .requires("r4");

        let args: Vec<Id> = vec!["a1".into(), "a4".into(), "a2".into(), "a3".into()];
        let reqs: Vec<Id> = vec!["r1".into(), "r2".into(), "r3".into(), "r4".into()];
        let confs: Vec<Id> = vec!["c1".into(), "c2".into(), "c3".into(), "c4".into()];

        assert_eq!(g.args, args);
        assert_eq!(g.requires, reqs);
        assert_eq!(g.conflicts, confs);
    }

    #[test]
    fn test_from() {
        let g = ArgGroup::new("test")
            .arg("a1")
            .arg("a4")
            .args(["a2", "a3"])
            .required(true)
            .conflicts_with("c1")
            .conflicts_with_all(["c2", "c3"])
            .conflicts_with("c4")
            .requires("r1")
            .requires_all(["r2", "r3"])
            .requires("r4");

        let args: Vec<Id> = vec!["a1".into(), "a4".into(), "a2".into(), "a3".into()];
        let reqs: Vec<Id> = vec!["r1".into(), "r2".into(), "r3".into(), "r4".into()];
        let confs: Vec<Id> = vec!["c1".into(), "c2".into(), "c3".into(), "c4".into()];

        let g2 = ArgGroup::from(&g);
        assert_eq!(g2.args, args);
        assert_eq!(g2.requires, reqs);
        assert_eq!(g2.conflicts, confs);
    }

    // This test will *fail to compile* if ArgGroup is not Send + Sync
    #[test]
    fn arg_group_send_sync() {
        fn foo<T: Send + Sync>(_: T) {}
        foo(ArgGroup::new("test"));
    }

    #[test]
    fn arg_group_expose_is_multiple_helper() {
        let args: Vec<Id> = vec!["a1".into(), "a4".into()];

        let mut grp_multiple = ArgGroup::new("test_multiple").args(&args).multiple(true);
        assert!(grp_multiple.is_multiple());

        let mut grp_not_multiple = ArgGroup::new("test_multiple").args(&args).multiple(false);
        assert!(!grp_not_multiple.is_multiple());
    }

    #[test]
    fn arg_group_expose_get_args_helper() {
        let args: Vec<Id> = vec!["a1".into(), "a4".into()];
        let grp = ArgGroup::new("program").args(&args);

        for (pos, arg) in grp.get_args().enumerate() {
            assert_eq!(*arg, args[pos]);
        }
    }
}
