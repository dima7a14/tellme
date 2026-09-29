use std::fmt;

use serde::{Deserialize, Deserializer, de::Visitor};

const URL: &str = "https://api.mymemory.translated.net/get";

#[derive(Deserialize, Debug)]
struct ResponseData {
    #[serde(rename = "translatedText")]
    translated_text: String,
    #[serde(rename = "match")]
    score: Option<f64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_field_names)]
struct TranslateResponse {
    response_data: ResponseData,
    response_status: Status,
    response_details: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Status(u32);

impl Status {
    fn is_ok(self) -> bool {
        (200..300).contains(&self.0)
    }
}

struct StatusVisitor;

impl Visitor<'_> for StatusVisitor {
    type Value = Status;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("an integer or a numeric string")
    }

    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
        u32::try_from(v).map(Status).map_err(E::custom)
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        v.parse::<u32>().map(Status).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(StatusVisitor)
    }
}

#[derive(Debug)]
pub struct TranslationResult {
    pub translation: String,
    pub score: f64,
}

pub async fn translate(
    word: &str,
    dest: &str,
) -> Result<TranslationResult, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut url = reqwest::Url::parse(URL)?;

    url.query_pairs_mut()
        .append_pair("q", word)
        .append_pair("langpair", &format!("en|{dest}"));

    let res = client.get(url.as_str()).send().await?;

    match res.error_for_status() {
        Ok(res) => {
            let (translation, score) = match res.json::<TranslateResponse>().await {
                Ok(res) => {
                    let response_status = res.response_status;

                    if !response_status.is_ok() {
                        return Err(res.response_details.into());
                    }

                    let translated_word = res.response_data.translated_text;
                    let score = res.response_data.score.unwrap_or_default();

                    (translated_word, score)
                }
                Err(err) => {
                    eprintln!("{err:?}");

                    return Err(Box::new(err));
                }
            };

            Ok(TranslationResult { translation, score })
        }
        Err(err) => {
            eprintln!("{err:?}");
            Err(Box::new(err))
        }
    }
}
