use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Result};

/// `ArgGroup`s are a family of related arguments and way for you to say, "Any of these arguments".
/// By placing arguments in a logical group, you can make easier requirement and exclusion rules
/// intead of having to list each individually, or when you want a rule to apply "any but not all"
/// arguments.
///
/// For instance, you can make an entire ArgGroup required, this means that one (and *only* one)
/// argument. from that group must be present. Using more than one argument from an ArgGroup causes
/// a failure (graceful exit).
///
/// You can also do things such as name an ArgGroup as a confliction or requirement, meaning any
/// of the arguments that belong to that group will cause a failure if present, or must present
/// respectively.
///
/// Perhaps the most common use of `ArgGroup`s is to require one and *only* one argument to be
/// present out of a given set. Imagine that you had multiple arguments, and you want one of them
/// to be required, but making all of them required isn't feasible because perhaps they conflict
/// with each other. For example, lets say that you were building an application where one could
/// set a given version number by supplying a string with an option argument, i.e.
/// `--set-ver v1.2.3`, you also wanted to support automatically using a previous version number
/// and simply incrementing one of the three numbers. So you create three flags `--major`,
/// `--minor`, and `--patch`. All of these arguments shouldn't be used at one time but you want to
/// specify that *at least one* of them is used. For this, you can create a group.
///
/// # Example
///
/// ```no_run
/// # use clap::{App, ArgGroup};
/// let _ = App::new("app")
/// .args_from_usage("--set-ver [ver] 'set the version manually'
///                   --major         'auto increase major'
///                   --minor         'auto increase minor'
///                   --patch         'auto increase patch")
/// .arg_group(ArgGroup::with_name("vers")
///                     .add_all(vec!["ver", "major", "minor","patch"])
///                     .required(true))
/// # .get_matches();
pub struct ArgGroup<'n, 'ar> {
    #[doc(hidden)]
    pub name: &'n str,
    #[doc(hidden)]
    pub args: HashSet<&'ar str>,
    #[doc(hidden)]
    pub required: bool,
    #[doc(hidden)]
    pub requires: Option<HashSet<&'ar str>>,
    #[doc(hidden)]
    pub conflicts: Option<HashSet<&'ar str>>
}

impl<'n, 'ar> ArgGroup<'n, 'ar> {
    /// Creates a new instace of `ArgGroup` using a unique string name.
    /// The name will only be used by the library consumer and not displayed to the use.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, ArgGroup};
    /// # let matches = App::new("myprog")
    /// #                 .arg_group(
    /// ArgGroup::with_name("conifg")
    /// # ).get_matches();
    pub fn with_name(n: &'n str) -> Self {
        ArgGroup {
            name: n,
            required: false,
            args: HashSet::new(),
            requires: None,
            conflicts: None
        }
    }

    /// Adds an argument to this group by name
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, ArgGroup};
    /// # let matches = App::new("myprog")
    /// #                 .arg_group(
    /// # ArgGroup::with_name("conifg")
    /// .add("config")
    /// # ).get_matches();
    pub fn add(mut self, n: &'ar str) -> Self {
        self.args.insert(n);
        self
    }

    /// Adds multiple arguments to this group by name using a Vec
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, ArgGroup};
    /// # let matches = App::new("myprog")
    /// #                 .arg_group(
    /// # ArgGroup::with_name("conifg")
    /// .add_all(vec!["config", "input", "output"])
    /// # ).get_matches();
    pub fn add_all(mut self, ns: Vec<&'ar str>) -> Self {
        for n in ns {
            self = self.add(n);
        }
        self
    }

    /// Sets the requirement of this group. A required group will be displayed in the usage string
    /// of the application in the format `[arg|arg2|arg3]`. A required `ArgGroup` simply states
    /// that one, and only one argument from this group *must* be present at runtime (unless
    /// conflicting with another argument).
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, ArgGroup};
    /// # let matches = App::new("myprog")
    /// #                 .arg_group(
    /// # ArgGroup::with_name("conifg")
    /// .required(true)
    /// # ).get_matches();
    pub fn required(mut self, r: bool) -> Self {
        self.required = r;
        self
    }

    /// Sets the requirement rules of this group. This is not to be confused with a required group.
    /// Requirement rules function just like argument requirement rules, you can name other
    /// arguments or groups that must be present when one of the arguments from this group is used.
    ///
    /// **NOTE:** The name provided may be an argument, or group name
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, ArgGroup};
    /// # let matches = App::new("myprog")
    /// #                 .arg_group(
    /// # ArgGroup::with_name("conifg")
    /// .requires("config")
    /// # ).get_matches();
    pub fn requires(mut self, n: &'ar str) -> Self {
        if let Some(ref mut reqs) = self.requires {
            reqs.insert(n);
        } else {
            let mut hs = HashSet::new();
            hs.insert(n);
            self.requires = Some(hs);
        }
        self
    }

    /// Sets the requirement rules of this group. This is not to be confused with a required group.
    /// Requirement rules function just like argument requirement rules, you can name other
    /// arguments or groups that must be present when one of the arguments from this group is used.
    ///
    /// **NOTE:** The names provided may be an argument, or group name
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, ArgGroup};
    /// # let matches = App::new("myprog")
    /// #                 .arg_group(
    /// # ArgGroup::with_name("conifg")
    /// .requires_all(vec!["config", "input"])
    /// # ).get_matches();
    pub fn requires_all(mut self, ns: Vec<&'ar str>) -> Self {
        for n in ns {
            self = self.requires(n);
        }
        self
    }

    /// Sets the exclusion rules of this group. Exclusion rules function just like argument
    /// exclusion rules, you can name other arguments or groups that must not be present when one
    /// of the arguments from this group are used.
    ///
    /// **NOTE:** The name provided may be an argument, or group name
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, ArgGroup};
    /// # let matches = App::new("myprog")
    /// #                 .arg_group(
    /// # ArgGroup::with_name("conifg")
    /// .conflicts_with("config")
    /// # ).get_matches();
    pub fn conflicts_with(mut self, n: &'ar str) -> Self {
        if let Some(ref mut confs) = self.conflicts {
            confs.insert(n);
        } else {
            let mut hs = HashSet::new();
            hs.insert(n);
            self.conflicts = Some(hs);
        }
        self
    }

    /// Sets the exclusion rules of this group. Exclusion rules function just like argument
    /// exclusion rules, you can name other arguments or groups that must not be present when one
    /// of the arguments from this group are used.
    ///
    /// **NOTE:** The names provided may be an argument, or group name
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, ArgGroup};
    /// # let matches = App::new("myprog")
    /// #                 .arg_group(
    /// # ArgGroup::with_name("conifg")
    /// .conflicts_with_all(vec!["config", "input"])
    /// # ).get_matches();
    pub fn conflicts_with_all(mut self, ns: Vec<&'ar str>) -> Self {
        for n in ns {
            self = self.conflicts_with(n);
        }
        self
    }
}

impl<'n, 'ar> Debug for ArgGroup<'n, 'ar> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{
            name:{:?},
            args: {:?},
            required: {:?},
            requires: {:?},
            conflicts: {:?},
}}", self.name, self.args, self.required, self.requires, self.conflicts)
    }
}
