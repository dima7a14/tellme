use clap::{arg, Command};
use tellme::{self, fetch_definitions};

fn cli() -> Command {
    Command::new("tellme")
        .about("A simple CLI for wordsapi.com")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("definitions")
                .about("Returns all definitions for the word")
                .arg(arg!(<WORD> "The word to lookup"))
                .arg_required_else_help(true),
        )
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("definitions", sub_matches)) => {
            let word = sub_matches.get_one::<String>("WORD").expect("required");
            println!("Looking for definitions of the word {}", word);
            fetch_definitions(word).await.expect("No error");
        }
        _ => unreachable!(),
    }
}
