use serde::Deserialize;
use std::fmt;

const KEY: &str = "a90a90a5-e266-4482-ada5-36408db2b20f";

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

pub async fn dictionary(word: &str) -> Result<DictionaryResult, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut url = reqwest::Url::parse(BASE_URL)?;
    url = url.join(word)?;
    url.query_pairs_mut().append_pair("key", KEY);

    let res = client.get(url.as_str()).send().await?;

    let api: ApiResponse = res.error_for_status()?.json().await?;

    match api {
        ApiResponse::Definitions(defs) => {
            defs.first()
                .map_or(Err("Empty definitions array returned".into()), |def| {
                    Ok(DictionaryResult {
                        definitions: def.shortdef.clone(),
                    })
                })
        }
        ApiResponse::Suggestions(suggestions) => {
            let msg = format!(
                "No exact definition found. Did you mean: ({})?",
                suggestions.join(", ")
            );

            Err(msg.into())
        }
    }
}
