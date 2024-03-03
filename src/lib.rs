use reqwest;
use std::error::Error;

pub async fn fetch_definitions(word: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://wordsapiv1.p.mashape.com/words/{word}"))
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await?
        .text()
        .await?;

    println!("body = {}", response);

    Ok(())
}
