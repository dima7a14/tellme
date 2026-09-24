use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[arg(short, long, global = true)]
    pub word: Option<String>,

    #[arg(short = 't', long, global = true)]
    pub dest: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Translate given word
    Translate,
    /// Display explanation for the given word
    Definition,
    /// Show the list of queried words
    Memory,
}
