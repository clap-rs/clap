use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "my-app")]
pub struct BasicCommand {
    #[clap(short, global = true)]
    config: bool,

    #[clap(short, conflicts_with("config"))]
    v: bool,

    #[command(subcommand)]
    test: Option<BasicCommandSubCommand>,
}

#[derive(Subcommand, Debug)]
pub enum BasicCommandSubCommand {
    /// Subcommand
    Test {
        #[clap(short, action(clap::ArgAction::Count))]
        debug: u8,
    },
}

// Checks to make sure boolean valued "Flag options" do not generate
// suggestions for a parameter. i.e:
//     --boolean_flag=BOOLEAN_FLAG
//
// This is both confusing and suggest erroneous behavior as clap will fail if you
// pass a value to a boolean flag
#[derive(Parser, Debug)]
#[command(name = "my-app")]
pub struct FlagWithoutValue {
    #[clap(long)]
    boolean_flag: bool,
}
