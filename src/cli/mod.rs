use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Translate given word
    Translate { word: String, dest: Option<String> },
    /// Display explanation for the given word
    Definition { word: String },
    /// Show the list of queried words
    Memory,
}
