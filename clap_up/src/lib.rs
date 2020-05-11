use cargo_up::{
    ra_hir::Semantics, ra_ide_db::RootDatabase, ra_syntax::ast, Runner, Upgrader, Version,
};

pub fn runner() -> Runner {
    Runner::new()
        .minimum("2.33.0")
        .unwrap()
        .version(
            Version::new("3.0.0-beta.1")
                .unwrap()
                .peers(&["structopt"])
                // .replace("clap::args::SubCommand", "App")
                // .replace_dep("structopt", "clap", features = ["derive"])
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
                    ],
                )
                .rename_methods(
                    "clap::args::arg::Arg",
                    &[
                        ["help", "about"],
                        ["from_usage", "from"],
                        ["set", "setting"],
                        ["unset", "unset_setting"],
                    ],
                )
                .rename_methods(
                    "clap::args::subcommand::SubCommand",
                    &[["with_name", "new"], ["from_yaml", "from"]],
                )
                .rename_members("clap::errors::Error", &[["message", "cause"]])
                .hook_method_call_expr(&print_method_calls),
        )
        .version(
            Version::new("3.0.0-rc.0")
                .unwrap()
                .rename_methods("clap::build::app::App", &[["set_term_width", "term_width"]])
                .rename_methods(
                    "clap::build::arg::Arg",
                    &[["from_yaml", "from"], ["with_name", "new"]],
                )
                .rename_methods(
                    "clap::build::arg_group::ArgGroup",
                    &[["from_yaml", "from"], ["with_name", "new"]],
                ),
        )
}

fn print_method_calls(
    upgrader: &mut Upgrader,
    method_call_expr: &ast::MethodCallExpr,
    semantics: &Semantics<RootDatabase>,
) {
    if let Some(name_ref) = method_call_expr.name_ref() {
        // println!("method: {}", name_ref.text());
    }
}
