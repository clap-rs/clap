use clap::Command;

fn main() {
    let root = Command::new("test").defer(|root| add_sub_commands(root, 1));

    let _m = root.get_matches();
}

fn add_sub_commands(parent: Command, level: usize) -> Command {
    if level >= 5 {
        return parent;
    }
    let mut res = parent;
    for i in 0..5 {
        let name = format!("s-{level}_{i}");
        let sub_cmd = Command::new(&name)
            .about(&name)
            .defer(move |cmd| add_sub_commands(cmd, level + 1));
        // res = res.subcommand(add_sub_commands(sub_cmd));
        res = res.subcommand(sub_cmd);
    }
    res
}
