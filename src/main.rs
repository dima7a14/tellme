use clap::{Command, arg};

fn cli() -> Command {
    Command::new("tellme")
        .about("My personal CLI tool for learning English unknown words")
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

fn main() {
    let _matches = cli().get_matches();

    // match matches.subcommand() {
    //     Some(("definitions", sub_matches)) => {
    //         let word = sub_matches.get_one::<String>("WORD").expect("required");
    //         println!("Looking for definitions of the word {}", word);
    //         fetch_definitions(word).await.expect("No error");
    //     }
    //     _ => unreachable!(),
    // }
}
