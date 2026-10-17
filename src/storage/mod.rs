pub mod redb_repository;
pub mod repository;

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{explanation::dictionary::DictionaryResult, translation::TranslationResult};

#[derive(Serialize, Deserialize, Default)]
pub struct Word {
    pub translation: Option<String>,
    pub definitions: Option<Vec<String>>,
}

impl From<TranslationResult> for Word {
    fn from(value: TranslationResult) -> Self {
        Self {
            translation: Some(value.translation),
            ..Default::default()
        }
    }
}

impl From<DictionaryResult> for Word {
    fn from(value: DictionaryResult) -> Self {
        Self {
            definitions: Some(value.definitions),
            ..Default::default()
        }
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let translation = self.translation.as_deref().unwrap_or("no translation");
        let definitions = self.definitions.as_deref().unwrap_or(&[]).join("\n");

        write!(f, "Translation: {translation}\nDefinitions:\n{definitions}")
    }
}
