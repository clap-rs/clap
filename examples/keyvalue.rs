#[macro_use]
extern crate structopt;

use structopt::StructOpt;

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let pos = s.find('=').ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].into(), s[pos + 1..].into()))
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "D", parse(try_from_str = "parse_key_val"))]
    defines: Vec<(String, String)>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
