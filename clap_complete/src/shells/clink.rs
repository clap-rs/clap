use crate::{generator::utils, Generator};
use clap::*;
use std::fmt::Write;

/// Generate clink completion lua script
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Clink;

impl Generator for Clink {
    fn file_name(&self, name: &str) -> String {
        format!("{name}.lua")
    }

    fn generate(&self, cmd: &clap::Command, buf: &mut dyn std::io::prelude::Write) {
        let bin_name = cmd
            .get_bin_name()
            .expect("crate::generate should have set the bin_name");

        let result = format!(
            r#"clink.argmatcher("{bin_name}")
{}"#,
            generate_inner(cmd, 0)
        );
        w!(buf, result.as_bytes());
    }
}

fn generate_inner(p: &Command, depth: usize) -> String {
    let mut b = String::new();
    let indent = " ".repeat(depth * 4);

    for opt in p.get_opts() {
        writeln!(b, "{}:addarg(\"--{}\")", indent, opt.get_id()).unwrap();
        if let Some(help) = opt.get_help() {
            writeln!(
                b,
                "{}:adddescriptions({{\"--{}\", description = \"{}\"}})",
                indent,
                opt.get_id(),
                help
            )
            .unwrap();
            if let Some(short) = opt.get_short() {
                writeln!(
                    b,
                    "{}:adddescriptions({{\"-{}\", description = \"{}\"}})",
                    indent, short, help
                )
                .unwrap()
            }
        }
    }

    for flag in utils::flags(p) {
        if let Some(shorts) = flag.get_short_and_visible_aliases() {
            for short in shorts {
                writeln!(b, "{}:addflags(\"-{}\")", indent, short).unwrap();
                if let Some(help) = flag.get_help() {
                    writeln!(
                        b,
                        "{}:adddescriptions({{\"-{}\", description = \"{}\"}})",
                        indent, short, help
                    )
                    .unwrap();
                }
            }
        }
        if let Some(longs) = flag.get_long_and_visible_aliases() {
            for long in longs {
                writeln!(b, "{}:addflags(\"--{}\")", indent, long).unwrap();
                if let Some(help) = flag.get_help() {
                    writeln!(
                        b,
                        "{}:adddescriptions({{\"--{}\", description = \"{}\"}})",
                        indent, long, help
                    )
                    .unwrap();
                }
            }
        }
    }

    for sub_cmd in p.get_subcommands() {
        writeln!(
            b,
            "{}:addarg({{ \"{}\" .. clink.argmatcher()\n{}{}}})",
            indent,
            sub_cmd.get_name(),
            generate_inner(sub_cmd, depth + 1),
            indent
        )
        .unwrap();
    }

    b
}
