use cargo_up::{
    ra_ap_syntax::{
        ast::{ArgListOwner, CallExpr},
        AstNode,
    },
    Runner, Version,
};

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
                        ["args_from_usage", "args"],
                        ["help", "override_help"],
                        ["usage", "override_usage"],
                        ["template", "help_template"],
                        ["get_matches_safe", "try_get_matches"],
                        ["get_matches_from_safe", "try_get_matches_from"],
                        ["get_matches_from_safe_borrow", "try_get_matches_from_mut"],
                        ["set_term_width", "term_width"],
                        ["with_defaults", "new"],
                        ["version_message", "mut_arg"],
                        ["version_short", "mut_arg"],
                        ["help_message", "mut_arg"],
                        ["help_short", "mut_arg"],
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
                .hook_method_call_expr_on("clap::app::App", "args_from_usage", |u, n, _| {
                    let arg = n.arg_list().unwrap().args().last();

                    u.insert(
                        arg.unwrap().syntax().text_range().end(),
                        ".lines().map(|l| l.trim()).filter(|l| !l.is_empty())",
                    );
                })
                .hook_path_expr_on("clap::app::App", "with_defaults", |u, n, _| {
                    if let Some(parent) = n.syntax().parent() {
                        if let Some(call_expr) = CallExpr::cast(parent) {
                            // TODO: Add full path
                            u.insert(
                                call_expr.syntax().text_range().end(),
                                ".author(crate_authors!()).version(crate_version!())",
                            );
                        }
                    }
                })
                .hook_method_call_expr_on("clap::app::App", "version_message", |u, n, _| {
                    let arg_list = n.arg_list().unwrap();

                    u.insert(
                        arg_list.l_paren_token().unwrap().text_range().end(),
                        "\"version\", |a| a.about(",
                    );
                    u.insert(arg_list.r_paren_token().unwrap().text_range().start(), ")")
                })
                .hook_method_call_expr_on("clap::app::App", "version_short", |u, n, _| {
                    let arg_list = n.arg_list().unwrap();

                    u.insert(
                        arg_list.l_paren_token().unwrap().text_range().end(),
                        "\"version\", |a| a.short(",
                    );
                    u.insert(
                        arg_list.r_paren_token().unwrap().text_range().start(),
                        ".trim_start_matches(|c| c == '-').chars().nth(0).unwrap_or('V'))",
                    )
                })
                .hook_method_call_expr_on("clap::app::App", "help_message", |u, n, _| {
                    let arg_list = n.arg_list().unwrap();

                    u.insert(
                        arg_list.l_paren_token().unwrap().text_range().end(),
                        "\"help\", |a| a.about(",
                    );
                    u.insert(arg_list.r_paren_token().unwrap().text_range().start(), ")")
                })
                .hook_method_call_expr_on("clap::app::App", "help_short", |u, n, _| {
                    let arg_list = n.arg_list().unwrap();

                    u.insert(
                        arg_list.l_paren_token().unwrap().text_range().end(),
                        "\"help\", |a| a.short(",
                    );
                    u.insert(
                        arg_list.r_paren_token().unwrap().text_range().start(),
                        ".trim_start_matches(|c| c == '-').chars().nth(0).unwrap_or('h'))",
                    )
                }),
        )
        .version(Version::new("3.0.0").unwrap())
}
