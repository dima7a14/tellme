use clap::Parser;
mod cli;

const DEFAULT_DEST: &str = "uk";

fn main() {
    let cli = cli::Cli::parse();

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mod is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    match &cli.command {
        Some(cli::Commands::Translate) => {
            let Some(word) = cli.word else {
                println!("You must provide a word!");
                return;
            };
            let dest = cli.dest.as_deref().unwrap_or(DEFAULT_DEST);
            println!("Translating a word \"{word}\" to {dest}...");
        }
        Some(cli::Commands::Definition) => {
            let Some(word) = cli.word else {
                println!("You must provide a word!");
                return;
            };
            println!("Looking for definitions for the word \"{word}\"");
        }
        Some(cli::Commands::Memory) => {
            let prev_words = ["Apple", "Sun", "Inquiry"];
            println!("Listing the list of words asked earlier {prev_words:?}");
        }
        None => {
            let Some(word) = cli.word else {
                println!("You must provide a word!");
                return;
            };
            let dest = cli.dest.as_deref().unwrap_or(DEFAULT_DEST);
            println!("Translating a word \"{word}\" to {dest} and looking for definitions...");
        }
    }
}
