use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub word: Option<String>,

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

#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("you must provide a word!")]
    MissingWord,
}

impl CliError {
    pub const fn code(&self) -> u8 {
        match self {
            Self::MissingWord => 2,
        }
    }
}

pub fn parse_word(word: Option<String>) -> Result<String, CliError> {
    word.ok_or(CliError::MissingWord)
}
