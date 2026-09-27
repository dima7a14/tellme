use std::process::{ExitCode, Termination};

use clap::Parser;
mod cli;
mod translation;

const DEFAULT_DEST: &str = "uk";

#[tokio::main]
async fn main() -> ExitCode {
    match inner_run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => e.report(),
    }
}

async fn inner_run() -> Result<(), cli::CliError> {
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

            println!(
                "Translation: {} (Score: {}).",
                result.translation, result.score
            );
            Ok(())
        }
        Some(cli::Commands::Definition) => {
            let word = cli::parse_word(cli.word)?;
            println!("Looking for definitions for the word \"{word}\"");
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

            println!(
                "Translation: {} (Score: {}).",
                translation_result.translation, translation_result.score
            );
            Ok(())
        }
    }
}
