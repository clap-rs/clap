// Std
use std::io::Write;

// Internal
use build::App;

pub struct FishGen<'a, 'b>(&'b App<'a, 'b>)
where
    'a: 'b;

impl<'a, 'b> FishGen<'a, 'b> {
    pub fn new(app: &'b App<'a, 'b>) -> Self { FishGen(app) }

    pub fn generate_to<W: Write>(&self, buf: &mut W) {
        let command = self.0.bin_name.as_ref().unwrap();
        let mut buffer = String::new();
        gen_fish_inner(command, self, command, &mut buffer);
        w!(buf, buffer.as_bytes());
    }
}

// Escape string inside single quotes
fn escape_string(string: &str) -> String { string.replace("\\", "\\\\").replace("'", "\\'") }

fn gen_fish_inner(root_command: &str, comp_gen: &FishGen, subcommand: &str, buffer: &mut String) {
    debugln!("FishGen::gen_fish_inner;");
    // example :
    //
    // complete
    //      -c {command}
    //      -d "{description}"
    //      -s {short}
    //      -l {long}
    //      -a "{possible_arguments}"
    //      -r # if require parameter
    //      -f # don't use file completion
    //      -n "__fish_use_subcommand"               # complete for command "myprog"
    //      -n "__fish_seen_subcommand_from subcmd1" # complete for command "myprog subcmd1"

    let mut basic_template = format!("complete -c {} -n ", root_command);
    if root_command == subcommand {
        basic_template.push_str("\"__fish_use_subcommand\"");
    } else {
        basic_template.push_str(format!("\"__fish_seen_subcommand_from {}\"", subcommand).as_str());
    }

    for (_, option) in opts!(comp_gen.0) {
        let mut template = basic_template.clone();
        if let Some(data) = option.short {
            template.push_str(format!(" -s {}", data).as_str());
        }
        if let Some(data) = option.long {
            template.push_str(format!(" -l {}", data).as_str());
        }
        if let Some(data) = option.help {
            template.push_str(format!(" -d '{}'", escape_string(data)).as_str());
        }
        if let Some(ref data) = option.possible_vals {
            template.push_str(format!(" -r -f -a \"{}\"", data.join(" ")).as_str());
        }
        buffer.push_str(template.as_str());
        buffer.push_str("\n");
    }

    for (_, flag) in flags!(comp_gen.0) {
        let mut template = basic_template.clone();
        if let Some(data) = flag.short {
            template.push_str(format!(" -s {}", data).as_str());
        }
        if let Some(data) = flag.long {
            template.push_str(format!(" -l {}", data).as_str());
        }
        if let Some(data) = flag.help {
            template.push_str(format!(" -d '{}'", escape_string(data)).as_str());
        }
        buffer.push_str(template.as_str());
        buffer.push_str("\n");
    }

    for subcommand in subcommands!(comp_gen.0) {
        let mut template = basic_template.clone();
        template.push_str(" -f");
        template.push_str(format!(" -a \"{}\"", &subcommand.name).as_str());
        if let Some(data) = subcommand.about {
            template.push_str(format!(" -d '{}'", escape_string(data)).as_str())
        }
        buffer.push_str(template.as_str());
        buffer.push_str("\n");
    }

    // generate options of subcommands
    for subcommand in &comp_gen.0.subcommands {
        let sub_comp_gen = FishGen::new(&subcommand);
        gen_fish_inner(root_command, &sub_comp_gen, &subcommand.to_string(), buffer);
    }
}
