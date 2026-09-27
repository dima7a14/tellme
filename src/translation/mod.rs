use serde::Deserialize;

const URL: &str = "https://api.mymemory.translated.net/get";

#[derive(Deserialize, Debug)]
struct ResponseData {
    #[serde(rename = "translatedText")]
    translated_text: String,
    #[serde(rename = "match")]
    score: f32,
}

#[derive(Deserialize, Debug)]
struct TranslateResponse {
    #[serde(rename = "responseData")]
    response_data: ResponseData,
}

#[derive(Debug)]
pub struct TranslationResult {
    pub translation: String,
    pub score: f32,
}

pub async fn translate(
    word: &str,
    dest: &str,
) -> Result<TranslationResult, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("{URL}?q={word}&langpair=en|{dest}");
    let res = client.get(url).send().await?;

    match res.error_for_status() {
        Ok(res) => {
            let (translation, score) = match res.json::<TranslateResponse>().await {
                Ok(res) => {
                    let translated_word = res.response_data.translated_text;
                    let score = res.response_data.score;

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
