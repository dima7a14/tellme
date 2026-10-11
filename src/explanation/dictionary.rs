use serde::Deserialize;
use std::env;
use std::fmt;

fn parse_key() -> Result<String, Box<dyn std::error::Error>> {
    env::var("MERRIAM_WEBSTER_API_KEY").map_err(|_| {
        Box::<dyn std::error::Error>::from(
            "MERRIAM_WEBSTER_API_KEY environment variable is not set.\n
            Copy .env.example to .env and add your Merriam-Webster API key.",
        )
    })
}

const BASE_URL: &str = "https://www.dictionaryapi.com/api/v3/references/collegiate/json/";

#[derive(Debug, Deserialize)]
struct DictionaryResponse {
    pub shortdef: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ApiResponse {
    Definitions(Vec<DictionaryResponse>), // e.g. [{"shortdef": ["..."]}]
    Suggestions(Vec<String>),             // e.g. ["kitty", "kitten", "..."]
}

#[derive(Debug)]
pub struct DictionaryResult {
    pub definitions: Vec<String>,
}

impl fmt::Display for DictionaryResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = self.definitions.join("\n");

        write!(f, "Definitions:\n{result}")
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DictionaryError {
    #[error(transparent)]
    Parse(#[from] url::ParseError),
    #[error(transparent)]
    Query(#[from] Box<dyn std::error::Error>),
    #[error("HTTP request failed")]
    Get(#[from] reqwest::Error),
    #[error("{0}")]
    Api(String),
}

pub async fn dictionary(word: &str) -> Result<DictionaryResult, DictionaryError> {
    let client = reqwest::Client::new();
    let mut url = reqwest::Url::parse(BASE_URL)?;
    url = url.join(word)?;
    url.query_pairs_mut().append_pair("key", &parse_key()?);

    let res = client.get(url.as_str()).send().await?;

    let api: ApiResponse = res.error_for_status()?.json().await?;

    match api {
        ApiResponse::Definitions(defs) => defs.first().map_or(
            Err(DictionaryError::Api(
                "Empty definitions array returned".into(),
            )),
            |def| {
                Ok(DictionaryResult {
                    definitions: def.shortdef.clone(),
                })
            },
        ),
        ApiResponse::Suggestions(suggestions) => {
            let msg = format!(
                "No exact definition found. Did you mean: {}?",
                suggestions.join(", ")
            );

            Err(DictionaryError::Api(msg))
        }
    }
}
