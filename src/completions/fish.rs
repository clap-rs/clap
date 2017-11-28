// Std
use std::io::Write;

// Internal
use app::parser::Parser;

pub struct FishGen<'a, 'b>
where
    'a: 'b,
{
    p: &'b Parser<'a, 'b>,
}

impl<'a, 'b> FishGen<'a, 'b> {
    pub fn new(p: &'b Parser<'a, 'b>) -> Self { FishGen { p: p } }

    pub fn generate_to<W: Write>(&self, buf: &mut W) {
        let command = self.p.meta.bin_name.as_ref().unwrap();

        // function to detect subcommand
        let detect_subcommand_function = r#"function __fish_using_command
    set cmd (commandline -opc)
    if [ (count $cmd) -eq (count $argv) ]
        for i in (seq (count $argv))
            if [ $cmd[$i] != $argv[$i] ]
                return 1
            end
        end
        return 0
    end
    return 1
end

"#.to_string();

        let mut buffer = detect_subcommand_function;
        gen_fish_inner(command, self, &command.to_string(), &mut buffer);
        w!(buf, buffer.as_bytes());
    }
}

// Escape string inside single quotes
fn escape_string(string: &str) -> String { string.replace("\\", "\\\\").replace("'", "\\'") }

fn gen_fish_inner(root_command: &str, comp_gen: &FishGen, parent_cmds: &str, buffer: &mut String) {
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
    //      -n "__fish_using_command myprog subcmd1" # complete for command "myprog subcmd1"

    let basic_template = format!(
        "complete -c {} -n \"__fish_using_command {}\"",
        root_command,
        parent_cmds
    );

    for option in comp_gen.p.opts() {
        let mut template = basic_template.clone();
        if let Some(data) = option.s.short {
            template.push_str(format!(" -s {}", data).as_str());
        }
        if let Some(data) = option.s.long {
            template.push_str(format!(" -l {}", data).as_str());
        }
        if let Some(data) = option.b.help {
            template.push_str(format!(" -d '{}'", escape_string(data)).as_str());
        }
        if let Some(ref data) = option.v.possible_vals {
            template.push_str(format!(" -r -f -a \"{}\"", data.join(" ")).as_str());
        }
        buffer.push_str(template.as_str());
        buffer.push_str("\n");
    }

    for flag in comp_gen.p.flags() {
        let mut template = basic_template.clone();
        if let Some(data) = flag.s.short {
            template.push_str(format!(" -s {}", data).as_str());
        }
        if let Some(data) = flag.s.long {
            template.push_str(format!(" -l {}", data).as_str());
        }
        if let Some(data) = flag.b.help {
            template.push_str(format!(" -d '{}'", escape_string(data)).as_str());
        }
        buffer.push_str(template.as_str());
        buffer.push_str("\n");
    }

    for subcommand in &comp_gen.p.subcommands {
        let mut template = basic_template.clone();
        template.push_str(" -f");
        template.push_str(format!(" -a \"{}\"", &subcommand.p.meta.name).as_str());
        if let Some(data) = subcommand.p.meta.about {
            template.push_str(format!(" -d '{}'", escape_string(data)).as_str())
        }
        buffer.push_str(template.as_str());
        buffer.push_str("\n");
    }

    // generate options of subcommands
    for subcommand in &comp_gen.p.subcommands {
        let sub_comp_gen = FishGen::new(&subcommand.p);
        // make new "parent_cmds" for different subcommands
        let mut sub_parent_cmds = parent_cmds.to_string();
        if !sub_parent_cmds.is_empty() {
            sub_parent_cmds.push_str(" ");
        }
        sub_parent_cmds.push_str(&subcommand.p.meta.name);
        gen_fish_inner(root_command, &sub_comp_gen, &sub_parent_cmds, buffer);
    }
}
