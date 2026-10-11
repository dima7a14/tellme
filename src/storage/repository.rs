use crate::storage::Word;

#[derive(thiserror::Error, Debug)]
pub enum WordRepositoryError {
    #[error("the word repository is unavailable")]
    Unavailable(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("stored entry for \"{word}\" is corrupted")]
    Corrupted {
        word: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

pub trait WordRepository {
    fn save(&self, word: &str, payload: &Word) -> Result<(), WordRepositoryError>;
    fn get(&self, word: &str) -> Result<Option<Word>, WordRepositoryError>;
    fn list(&self) -> Result<Vec<(String, Word)>, WordRepositoryError>;
}
