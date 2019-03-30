pub struct BuiltCmd<'help> {
    // Used to identify the Cmd and all it's aliases in an efficient manner
    #[doc(hidden)]
    pub ids: Vec<ArgId>,
    // Settings that change how the args are parsed, or App behaves
    #[doc(hidden)]
    pub settings: CmdFlags,
    // Global settings (i.e. all subcommands)
    #[doc(hidden)]
    pub g_settings: CmdFlags,
    // The list of valid arguments
    #[doc(hidden)]
    pub args: ArgsVec<'help>,
    // A list of valid subcommands
    #[doc(hidden)]
    pub subcommands: Vec<ArgId>,
}

impl<'help> BuiltCmd<'help> {
    pub(crate) fn parse<T: ArgV>(&mut self, it: &mut Peekable<T>) -> ClapResult<ParseResult> {
        debugln!("Cmd::parse;");
        let mut matcher = {
            let mut parser = Parser::new();

            // do the real parsing
            parser.parse(it, self)?
        };

        let global_arg_vec: Vec<u64> = self.global_args().map(|ga| ga.id).collect();
        matcher.propagate_globals(&global_arg_vec);

        Ok(matcher.into())
    }
}

// Facade for ArgsVec
#[doc(hidden)]
impl<'help> BuiltCmd<'help> {
    pub fn args(&self) -> Args<'help> { self.args.args() }
    pub fn args_mut(&mut self) -> ArgsMut<'help> { self.args.args_mut() }
    pub fn flags(&self) -> Flags<'help> { self.args.flags() }
    pub fn flags_mut(&mut self) -> FlagsMut<'help> { self.args.flags_mut() }
    pub fn options(&self) -> Options<'help> { self.args.args() }
    pub fn options_mut(&mut self) -> OptionsMut<'help> { self.args.args() }
    pub fn positionals(&self) -> Positionals<'help> { self.args.positionals() }
    pub fn positionals_mut(&mut self) -> PositionalsMut<'help> { self.args.positionals_mut() }
    pub fn global_args(&self) -> impl Iterator<Item=&Arg<'help>> {
        self.args().filter(|x| x.is_set(ArgSettings::Global))
    }
    pub fn global_args_mut(&mut self) -> impl Iterator<Item=&mut Arg<'help>> {
        self.args_mut().filter(|x| x.is_set(ArgSettings::Global))
    }
}
