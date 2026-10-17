use std::process::{ExitCode, Termination};

use clap::Parser;

use crate::explanation::dictionary;
use crate::storage::Word;
use crate::storage::redb_repository::{RedbError, RedbRepository};
use crate::storage::repository::{WordRepository, WordRepositoryError};

mod cli;
mod explanation;
mod storage;
mod translation;

const DEFAULT_DEST: &str = "uk";

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error(transparent)]
    Translation(#[from] translation::TranslationError),
    #[error(transparent)]
    Dictionary(#[from] dictionary::DictionaryError),
    #[error(transparent)]
    Storage(#[from] WordRepositoryError),
    #[error("failed to establish the database connection")]
    Database(#[source] RedbError),
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
    let storage = RedbRepository::new().map_err(AppError::Database)?;

    match &cli.command {
        Some(cli::Commands::Translate) => {
            let word = cli::parse_word(cli.word)?;
            let dest = cli.dest.as_deref().unwrap_or(DEFAULT_DEST);

            if let Some(existing) = storage.get(&word)? {
                println!("Found word \"{word}\" in the cache!");
                println!("{existing}");
            } else {
                println!("Translating a word \"{word}\" to {dest}...");
                let result = translation::translate(&word, dest).await?;
                println!("{result}");

                storage.save(
                    &word,
                    Word {
                        translation: Some(result.translation),
                        ..Default::default()
                    },
                )?;
            }

            Ok(())
        }
        Some(cli::Commands::Definition) => {
            let word = cli::parse_word(cli.word)?;

            if let Some(existing) = storage.get(&word)? {
                println!("Found word \"{word}\" in the cache!");
                println!("{existing}");
            } else {
                println!("Looking for definitions for the word \"{word}\"");
                let dict = explanation::dictionary::dictionary(&word).await?;
                println!("{dict}");

                storage.save(
                    &word,
                    Word {
                        definitions: Some(dict.definitions),
                        ..Default::default()
                    },
                )?;
            }

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

            if let Some(existing) = storage.get(&word)? {
                println!("Found word \"{word}\" in the cache!");
                println!("{existing}");
            } else {
                println!("Translating a word \"{word}\" to {dest} and looking for definitions...");
                let translation_result = translation::translate(&word, dest).await?;
                println!("{translation_result}");
                let dict = explanation::dictionary::dictionary(&word).await?;
                println!("{dict}");

                storage.save(
                    &word,
                    Word {
                        translation: Some(translation_result.translation),
                        definitions: Some(dict.definitions),
                    },
                )?;
            }

            Ok(())
        }
    }
}
