use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[clap(name = "rust-boilerplate", about = "Some about", author, version)]
pub struct CliArguments {
    /// Verbose mode (-v, -vv, -vvv)
    #[clap(short, long, action = ArgAction::Count)]
    pub verbose: u8,
}
