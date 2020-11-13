use cargo_up::{ra_ap_syntax::ast, Runner, Semantics, Upgrader, Version};

pub fn runner() -> Runner {
    Runner::new()
        .minimum("2.33.0")
        .unwrap()
        .version(
            Version::new("3.0.0-rc.0")
                .unwrap()
                .peers(&["structopt"])
                // .replace_dep("structopt", "clap", features = ["derive"])
                .rename_structs("clap::args::subcommand", &[["SubCommand", "App"]])
                .rename_methods(
                    "structopt::StructOpt",
                    &[
                        ["from_args", "parse"],
                        ["from_iter", "parse_from"],
                        ["from_iter_safe", "try_parse_from"],
                        ["from_clap", "from_arg_matches"],
                        ["clap", "into_app"],
                    ],
                )
                .rename_variants(
                    "clap::errors::ErrorKind",
                    &[
                        ["HelpDisplayed", "DisplayHelp"],
                        ["VersionDisplayed", "DisplayVersion"],
                        [
                            "MissingArgumentOrSubcommand",
                            "DisplayHelpOnMissingArgumentOrSubcommand",
                        ],
                    ],
                )
                .rename_variants(
                    "clap::app::settings::AppSettings",
                    &[
                        ["DisableHelpFlags", "DisableHelpFlag"],
                        ["DisableVersion", "DisableVersionFlag"],
                        ["VersionlessSubcommands", "DisableVersionForSubcommands"],
                    ],
                )
                .rename_variants(
                    "clap::args::settings::ArgSettings",
                    &[
                        ["CaseInsensitive", "IgnoreCase"],
                        ["AllowLeadingHyphen", "AllowHyphenValues"],
                        ["EmptyValues", "AllowEmptyValues"],
                    ],
                )
                .rename_methods(
                    "clap::app::App",
                    &[
                        ["from_yaml", "from"],
                        ["arg_from_usage", "arg"],
                        ["help", "override_help"],
                        ["usage", "override_usage"],
                        ["template", "help_template"],
                        ["get_matches_safe", "try_get_matches"],
                        ["get_matches_from_safe", "try_get_matches_from"],
                        ["get_matches_from_safe_borrow", "try_get_matches_from_mut"],
                        ["set_term_width", "term_width"],
                    ],
                )
                .rename_methods(
                    "clap::args::arg::Arg",
                    &[
                        ["help", "about"],
                        ["from_usage", "from"],
                        ["set", "setting"],
                        ["unset", "unset_setting"],
                        ["from_yaml", "from"],
                        ["with_name", "new"],
                        ["required_if", "required_if_eq"],
                        ["required_ifs", "required_if_eq_any"],
                        ["required_unless", "required_unless_present"],
                        ["required_unless_one", "required_unless_present_any"],
                        ["required_unless_all", "required_unless_present_all"],
                    ],
                )
                .rename_methods(
                    "clap::args::group::ArgGroup",
                    &[["from_yaml", "from"], ["with_name", "new"]],
                )
                .rename_methods(
                    "clap::args::subcommand::SubCommand",
                    &[["from_yaml", "from"], ["with_name", "new"]],
                )
                .rename_members("clap::errors::Error", &[["message", "cause"]]) // TODO: check
                .hook_method_call_expr(&print_method_calls),
        )
        .version(Version::new("3.0.0").unwrap())
}

fn print_method_calls(
    upgrader: &mut Upgrader,
    method_call_expr: &ast::MethodCallExpr,
    semantics: &Semantics,
) {
    if let Some(name_ref) = method_call_expr.name_ref() {
        // println!("method: {}", name_ref.text());
    }
}
