use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[arg(short = 't', long = "destination", value_name = "Destination language")]
    pub dest: Option<String>,

    #[arg(short = 'w', long = "word", value_name = "Word")]
    pub word: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Translate,
    Definition,
    Memory,
}
