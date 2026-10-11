use std::process::{ExitCode, Termination};

use clap::Parser;

use crate::explanation::dictionary;

mod cli;
mod explanation;
// mod storage;
mod translation;

const DEFAULT_DEST: &str = "uk";

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error(transparent)]
    Translation(#[from] translation::TranslationError),
    #[error(transparent)]
    Dictionary(#[from] dictionary::DictionaryError),
    // #[error(transparent)]
    // Storage(#[from] storage::StorageError),
    #[error(transparent)]
    Cli(#[from] cli::CliError),
}

impl Termination for AppError {
    fn report(self) -> ExitCode {
        eprintln!("{self}");
        let code = match &self {
            Self::Cli(e) => e.code(),
            _ => 1,
        };

        ExitCode::from(code)
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let _ = dotenvy::dotenv();
    match inner_run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => e.report(),
    }
}

async fn inner_run() -> Result<(), AppError> {
    let cli = cli::Cli::parse();

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mod is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    match &cli.command {
        Some(cli::Commands::Translate) => {
            let word = cli::parse_word(cli.word)?;
            let dest = cli.dest.as_deref().unwrap_or(DEFAULT_DEST);
            println!("Translating a word \"{word}\" to {dest}...");
            let result = translation::translate(&word, dest).await?;
            println!("{result}");

            Ok(())
        }
        Some(cli::Commands::Definition) => {
            let word = cli::parse_word(cli.word)?;
            println!("Looking for definitions for the word \"{word}\"");
            let dict = explanation::dictionary::dictionary(&word).await?;
            println!("{dict}");
            Ok(())
        }
        Some(cli::Commands::Memory) => {
            let prev_words = ["Apple", "Sun", "Inquiry"];
            println!("Listing the list of words asked earlier {prev_words:?}");
            Ok(())
        }
        None => {
            let word = cli::parse_word(cli.word)?;
            let dest = cli.dest.as_deref().unwrap_or(DEFAULT_DEST);
            println!("Translating a word \"{word}\" to {dest} and looking for definitions...");
            let translation_result = translation::translate(&word, dest).await?;
            println!("{translation_result}");
            let dict = explanation::dictionary::dictionary(&word).await?;
            println!("{dict}");

            Ok(())
        }
    }
}
